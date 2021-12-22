use bson::DateTime;
use mongodb::{
    bson::doc,
    options::FindOneOptions,
    {Client, Collection},
};
use serde::{Deserialize, Serialize};
use std::error::Error;

const DB_NAME: &str = "library";
const COLL_NAME: &str = "books";

// Book record
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Book {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scores: Option<Vec<Score>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<DateTime>,
}

// Score sub-record
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Score {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<i32>,
}

// Book scores manager
#[derive(Debug, Clone)]
pub struct BookScoresMgr {
    coll: Collection<Book>,
}

// Manages interaction with books database collection
//
impl BookScoresMgr {
    // Create new instance of book score manager using provided MongoDB URL
    //
    pub async fn new(db_url: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let client = Client::with_uri_str(db_url).await?;
        let coll = client.database(DB_NAME).collection(COLL_NAME);
        Ok(Self { coll })
    }

    // Query books collection returning list of book scores for a book
    //
    pub async fn db_find_book_scores(&self, book: &Book) -> Result<Option<Book>, Box<dyn Error>> {
        if book.title.is_none() || book.author.is_none() {
            return Ok(None);
        }

        let title = get_or_err(book.title.as_ref(), "title")?;
        let author = get_or_err(book.author.as_ref(), "author")?;
        let find_options = FindOneOptions::builder()
            .projection(doc! {"title": 1, "author": 1, "year": 1, "scores": 1, "last_modified": 1})
            .build();
        let doc = self.coll.find_one(doc! {"title": title, "author": author}, find_options).await?;
        Ok(doc)
    }

    // Insert new book score
    //
    pub async fn db_insert_book_score(&self, book: &Book) -> Result<(), Box<dyn Error>> {
        let title = get_or_err(book.title.as_ref(), "title")?;
        let author = get_or_err(book.author.as_ref(), "author")?;
        let scores = get_or_err(book.scores.as_ref(), "scores")?;
        let score = get_or_err(scores.first(), "scores[0]")?;
        let reference = get_or_err(score.reference.as_ref(), "scores[0].reference")?;
        let rating = get_or_err(score.rating.as_ref(), "scores[0].rating")?;
        self.coll
            .update_one(
                doc! {"title": title, "author": author},
                doc! {
                    "$push": {"scores": {"reference": reference, "rating": rating}},
                    "$set": {"last_modified": DateTime::now()}
                },
                None,
            )
            .await?;
        Ok(())
    }

    // Update existing book record adding new quantity
    //
    pub async fn db_update_book_score(&self, book: &Book) -> Result<(), Box<dyn Error>> {
        self.db_delete_book_scores(book).await?;
        self.db_insert_book_score(book).await?;
        Ok(())
    }

    // Delete a score from a book's record for the matching reviewer reference
    //
    pub async fn db_delete_book_scores(&self, book: &Book) -> Result<(), Box<dyn Error>> {
        let title = get_or_err(book.title.as_ref(), "title")?;
        let author = get_or_err(book.author.as_ref(), "author")?;
        let scores = get_or_err(book.scores.as_ref(), "scores")?;
        let score = get_or_err(scores.first(), "scores[0]")?;
        if let Some(reference) = &score.reference {
            self.coll
                .update_one(
                    doc! {"title": title, "author": author},
                    doc! {
                        "$pull": {"scores": {"reference": reference}},
                        "$set": {"last_modified": DateTime::now()}
                    },
                    None,
                )
                .await?;
        }

        Ok(())
    }
}

// Validate specific variable field has a value, returning it, otherwise returning an error
//
fn get_or_err<'a, T>(field: Option<&'a T>, fieldname: &str) -> Result<&'a T, Box<dyn Error>> {
    match field {
        Some(val) => Ok(val),
        None => Err(format!("Field `{}` is empty, but is required", fieldname).into()),
    }
}
