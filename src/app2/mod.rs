use serde::{Deserialize, Serialize};
use std::error::Error;
use std::net::Ipv4Addr;
use warp::{http, Filter};


mod db;
use db::{Book, BookScoresMgr, Score};


const LISTEN_ADDRESS: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const LISTEN_PORT: u16 = 8282;
const RSC_VERSION: &str = "v1";
const RSC_NAME: &str = "books";
const PAYLOAD_LIMIT: u64 = 1024 * 16;


// Book record to extract from/to JSON payload
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BookPayload {
    pub title: Option<String>,
    pub author: Option<String>,
    pub year: Option<i32>,
    pub reference: Option<String>,
    pub score: Option<f32>,
}


// App2 main function to setup book scores REST API service
//
pub async fn app2_main(url: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("App2 running against MongoDB database at '{}'", url);
    let book_scores_mgr = BookScoresMgr::new(&url).await?;
    let book_scores_mgr_ref = warp::any().map(move || book_scores_mgr.clone());
    let api_path_filter_chain = warp::path(RSC_VERSION)
        .and(warp::path(RSC_NAME))
        .and(warp::path::end());
    let api_path_json_capture_filter_chain = api_path_filter_chain
        .and(capture_book_body_json())
        .and(book_scores_mgr_ref.clone());
    // CREATE: HTTP POST filter chain
    let add_items = warp::post()
        .and(api_path_json_capture_filter_chain.clone())
        .and_then(insert_book_score);
    // READ: HTTP GET filter chain
    let get_items = warp::get()
        .and(api_path_filter_chain)
        .and(capture_book_query_string())
        .and(book_scores_mgr_ref.clone())
        .and_then(get_book_score);
    // UPDATE: HTTP PUT filter chain
    let update_item = warp::put()
        .and(api_path_json_capture_filter_chain.clone())
        .and_then(update_book_score);
    // DELETE: HTTP DELETE filter chain
    let delete_item = warp::delete()
        .and(api_path_json_capture_filter_chain.clone())
        .and_then(delete_book_score);
    let routes = add_items.or(get_items).or(update_item).or(delete_item);
    println!("- HTTP REST API listening on: http://{}:{}/{}/{}", LISTEN_ADDRESS, LISTEN_PORT,
             RSC_VERSION, RSC_NAME);
    println!("- Eg1: http://{}:{}/{}/{}?title=The%20Last%20Man&author=Mary%20Shelley",
             LISTEN_ADDRESS, LISTEN_PORT, RSC_VERSION, RSC_NAME);
    println!("- Eg2: http://{}:{}/{}/{}?title=The%20Day%20of%20the%20Triffids&\
             author=John%20Wyndham",
             LISTEN_ADDRESS, LISTEN_PORT, RSC_VERSION, RSC_NAME);
    warp::serve(routes).run((LISTEN_ADDRESS, LISTEN_PORT)).await;
    Ok(())
}


// Capture book http query string parameters
//
fn capture_book_query_string() -> impl Filter<Extract = (BookPayload,),
                                              Error = warp::Rejection> + Clone {
    warp::query::query()
}


// Capture book http request payload JSON content
//
fn capture_book_body_json() -> impl Filter<Extract = (BookPayload,),
                                           Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(PAYLOAD_LIMIT).and(warp::body::json())
}


// Insert book score sub-record in back-end DB
//
async fn insert_book_score(book_payload: BookPayload, book_scores_mgr: BookScoresMgr)
                           -> Result<impl warp::Reply, warp::Rejection> {
    match book_scores_mgr.db_insert_book_score(&book_payload_to_book(&book_payload)).await {
        Ok(_) => Ok(warp::reply::with_status("Added new review score for book",
                    http::StatusCode::CREATED)),
        Err(e) => {
            eprintln!("Error inserting data: {:#?}", e);
            Err(warp::reject())
        }
    }
}


// Update book score sub-record in back-end DB
//
async fn update_book_score(book_payload: BookPayload, book_scores_mgr: BookScoresMgr)
                           -> Result<impl warp::Reply, warp::Rejection> {
    match book_scores_mgr.db_update_book_score(&book_payload_to_book(&book_payload)).await {
        Ok(_) => Ok(warp::reply::with_status("Updated existing review score for book",
                    http::StatusCode::CREATED)),
        Err(e) => {
            eprintln!("Error updating data: {:#?}", e);
            Err(warp::reject())
        }
    }
}


// Find all book scores sub-records from back-end DB
//
async fn get_book_score(book_payload: BookPayload, book_scores_mgr: BookScoresMgr)
                        -> Result<impl warp::Reply, warp::Rejection> {
    match book_scores_mgr.db_find_book_scores(&book_payload_to_book(&book_payload)).await {
        Ok(result) => Ok(warp::reply::json(&book_to_book_payload(&result))),
        Err(e) => {
            eprintln!("Error finding data: {}", e.to_string());
            Err(warp::reject())
        }
    }
}


// Delete specific book score sub-record from back-end DB
//
async fn delete_book_score(book_payload: BookPayload, book_scores_mgr: BookScoresMgr)
                           -> Result<impl warp::Reply, warp::Rejection> {
    match book_scores_mgr.db_delete_book_scores(&book_payload_to_book(&book_payload)).await {
        Ok(_) => Ok(warp::reply::with_status("Removed review score for book",
                    http::StatusCode::OK)),
        Err(e) => {
            eprintln!("Error deleting data: {}", e.to_string());
            Err(warp::reject())
        }
    }
}


// Take contents of Book payload and put into Book record to be passed to DB tier
//
fn book_payload_to_book(book_payload: &BookPayload)
                        -> Book {
    let rating = match book_payload.score {
        Some(val) => Some(val as i32),
        None => None,
    };
    let scores = Some(vec![Score { reference: book_payload.reference.clone(), rating }]);
    Book {
        title: book_payload.title.clone(),
        author: book_payload.author.clone(),
        year: book_payload.year,
        scores,
        last_modified: None,
    }
}


// Build response Books payload based on book records returned from DB tier
//
fn book_to_book_payload(optional_book: &Option<Book>)
                        -> BookPayload {
    match optional_book {
        Some(book) => {
            let avg_score = match &book.scores {
                Some(scores) => {
                    let res = scores
                        .iter()
                        .map(|score| score.rating.unwrap_or(0))
                        .sum::<i32>() as f32 / scores.len() as f32;

                    if res.is_nan() {
                        None
                    } else {
                        Some(res)
                    }
                }
                None => None,
            };

            let note = Some(String::from(
                if avg_score.is_some() {
                    "Average score accross all reviews"
                } else {
                    "No scores recorded"
                }
            ));


            BookPayload {
                title: book.title.clone(),
                author: book.author.clone(),
                year: book.year,
                reference: note,
                score: avg_score,
            }
        }
        None => BookPayload {
            title: None,
            author: None,
            year: None,
            reference: None,
            score: None,
        },
    }
}

