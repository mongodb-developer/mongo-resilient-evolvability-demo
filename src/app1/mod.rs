use serde::{Deserialize, Serialize};
use std::error::Error;
use std::net::Ipv4Addr;
use warp::{http, Filter};

mod db;
use db::{Book, BooksMgr};

const LISTEN_ADDRESS: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const LISTEN_PORT: u16 = 8181;
const RSC_VERSION: &str = "v1";
const RSC_NAME: &str = "books";
const PAYLOAD_LIMIT: u64 = 1024 * 16;

// Book record to extract from/to JSON payload
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BookPayload {
    pub title: Option<String>,
    pub author: Option<String>,
    pub year: Option<i32>,
    pub quantity: Option<i32>,
    pub explicit: Option<bool>,
}

// App1 main function to setup books REST API service
//
pub async fn app1_main(url: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("App1 running against MongoDB database at '{}'", url);
    let booksmgr = BooksMgr::new(url).await?;
    let booksmgr_ref = warp::any().map(move || booksmgr.clone());
    let api_path_filter_chain =
        warp::path(RSC_VERSION).and(warp::path(RSC_NAME)).and(warp::path::end());
    let api_path_json_capture_filter_chain =
        api_path_filter_chain.and(capture_book_body_json()).and(booksmgr_ref.clone());
    // CREATE: HTTP POST filter chain
    let add_items =
        warp::post().and(api_path_json_capture_filter_chain.clone()).and_then(insert_book_list);
    // READ: HTTP GET filter chain
    let get_items = warp::get()
        .and(api_path_filter_chain)
        .and(capture_book_query_string())
        .and(booksmgr_ref.clone())
        .and_then(get_books_list);
    // UPDATE: HTTP PUT filter chain
    let update_item =
        warp::put().and(api_path_json_capture_filter_chain.clone()).and_then(update_book_list);
    // DELETE: HTTP DELETE filter chain
    let delete_item =
        warp::delete().and(api_path_json_capture_filter_chain.clone()).and_then(delete_book_list);
    let routes = add_items.or(get_items).or(update_item).or(delete_item);
    println!(
        "- HTTP REST API listening on: http://{}:{}/{}/{}",
        LISTEN_ADDRESS, LISTEN_PORT, RSC_VERSION, RSC_NAME
    );
    println!(
        "- Eg: http://{}:{}/{}/{}?title=The%20Day%20of%20the%20Triffids&author=John%20Wyndham",
        LISTEN_ADDRESS, LISTEN_PORT, RSC_VERSION, RSC_NAME
    );
    warp::serve(routes).run((LISTEN_ADDRESS, LISTEN_PORT)).await;
    Ok(())
}

// Capture book http query string parameters
//
fn capture_book_query_string(
) -> impl Filter<Extract = (BookPayload,), Error = warp::Rejection> + Clone {
    warp::query::query()
}

// Capture book http request payload JSON content
//
fn capture_book_body_json(
) -> impl Filter<Extract = (BookPayload,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(PAYLOAD_LIMIT).and(warp::body::json())
}

// Insert book record in back-end DB
//
async fn insert_book_list(
    book_payload: BookPayload, booksmgr: BooksMgr,
) -> Result<impl warp::Reply, warp::Rejection> {
    match booksmgr.db_insert_book(&mut book_payload_to_book(&book_payload)).await {
        Ok(_) => Ok(warp::reply::with_status(
            "Inserted new book into the book list",
            http::StatusCode::CREATED,
        )),
        Err(e) => {
            eprintln!("Error inserting data: {:#?}", e);
            Err(warp::reject())
        }
    }
}

// Update book record in back-end DB
//
async fn update_book_list(
    book_payload: BookPayload, booksmgr: BooksMgr,
) -> Result<impl warp::Reply, warp::Rejection> {
    match booksmgr.db_update_book(&book_payload_to_book(&book_payload)).await {
        Ok(_) => Ok(warp::reply::with_status(
            "Incremented book amount in the book list",
            http::StatusCode::CREATED,
        )),
        Err(e) => {
            eprintln!("Error updating data: {:#?}", e);
            Err(warp::reject())
        }
    }
}

// Find all book records from back-end DB
//
async fn get_books_list(
    book_payload: BookPayload, booksmgr: BooksMgr,
) -> Result<impl warp::Reply, warp::Rejection> {
    match booksmgr.db_find_books(&book_payload_to_book(&book_payload)).await {
        Ok(result) => Ok(warp::reply::json(&books_to_books_payload(&result))),
        Err(e) => {
            eprintln!("Error deleting data: {}", e.to_string());
            Err(warp::reject())
        }
    }
}

// Delete specific book record from back-end DB
//
async fn delete_book_list(
    book_payload: BookPayload, booksmgr: BooksMgr,
) -> Result<impl warp::Reply, warp::Rejection> {
    match booksmgr.db_delete_book(&book_payload_to_book(&book_payload)).await {
        Ok(_) => Ok(warp::reply::with_status("Removed book from books list", http::StatusCode::OK)),
        Err(e) => {
            eprintln!("Error deleting data: {}", e.to_string());
            Err(warp::reject())
        }
    }
}

// Take contents of Book payload and put into Book record to be passed to DB tier
//
fn book_payload_to_book(book_payload: &BookPayload) -> Book {
    Book {
        title: book_payload.title.clone(),
        author: book_payload.author.clone(),
        year: book_payload.year,
        quantity: book_payload.quantity,
        explicit: book_payload.explicit,
        first_created: None,
        last_modified: None,
    }
}

// Build response Books payload based on book records returned from DB tier
//
fn books_to_books_payload(books: &[Book]) -> Vec<BookPayload> {
    let mut books_payload = vec![];

    for book in books {
        books_payload.push(BookPayload {
            title: book.title.clone(),
            author: book.author.clone(),
            year: book.year,
            quantity: book.quantity,
            explicit: book.explicit,
        });
    }

    books_payload
}
