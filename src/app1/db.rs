use bson::{from_document, to_document, DateTime};
use chrono::offset::Utc;
use futures::prelude::*;
use mongodb::{
    bson::doc,
    options::FindOptions,
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
    pub quantity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_created: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<DateTime>,
}


// Book manager
#[derive(Debug, Clone)]
pub struct BooksMgr {
    coll: Collection,
}


// Manages interaction with books database collection
//
impl BooksMgr {
    // Create new instance of books manager using provided MongoDB URL
    //
    pub async fn new(db_url: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let client = Client::with_uri_str(db_url).await?;
        let coll = client.database(DB_NAME).collection(COLL_NAME);
        Ok(Self { coll })
    }


    // Query books collection returning list of all books & quantities
    //
    pub async fn db_find_books(&self, book: &Book) -> Result<Vec<Book>, Box<dyn Error>> {
        let mut results = vec![];
        let filter_doc = if book.title.is_some() && book.author.is_some() {
            doc! {
                "title": get_or_err(book.title.as_ref(), "title")?,
                "author": get_or_err(book.author.as_ref(), "author")?,
            }
        } else if book.title.is_some() {
            doc! {"title": get_or_err(book.title.as_ref(), "title")?,}
        } else if book.author.is_some() {
            doc! {"author": get_or_err(book.author.as_ref(), "author")?}
        } else {
            doc! {}
        };
        let find_options = FindOptions::builder()
            .projection(doc! {
                "title": 1, "author": 1, "year": 1, "quantity": 1, "explicit": 1,
                "first_created": 1, "last_modified": 1
            })
            .sort(doc! {"year": 1})
            .build();
        let mut cursor = self.coll.find(filter_doc, find_options).await?;

        while let Some(doc) = cursor.next().await {
            results.push(from_document(doc?)?);
        }

        Ok(results)
    }


    // Insert new book record
    //
    pub async fn db_insert_book(&self, book: &mut Book) -> Result<(), Box<dyn Error>> {
        err_if_none(&book.title, "title")?;
        err_if_none(&book.author, "author")?;
        err_if_none(&book.year, "year")?;
        err_if_none(&book.quantity, "quantity")?;
        let now = Some(DateTime(Utc::now()));
        book.first_created = now;
        book.last_modified = now;
        self.coll.insert_one(to_document(&book)?, None).await?;
        Ok(())
    }


    // Update existing book record adding new quantity
    //
    pub async fn db_update_book(&self, book: &Book) -> Result<(), Box<dyn Error>> {
        let title = get_or_err(book.title.as_ref(), "title")?;
        let author = get_or_err(book.author.as_ref(), "author")?;
        let quantity = get_or_err(book.quantity.as_ref(), "quantity")?;
        self.coll
            .update_one(
                doc! {"title": title, "author": author},
                doc! {"$inc": {"quantity": quantity}, "$set": {"last_modified": Utc::now()}},
                None,
            )
            .await?;
        Ok(())
    }


    // Delete book record from books collection which matches book title
    //
    pub async fn db_delete_book(&self, book: &Book) -> Result<(), Box<dyn Error>> {
        let title = get_or_err(book.title.as_ref(), "title")?;
        let author = get_or_err(book.author.as_ref(), "author")?;
        self.coll
            .delete_one(
                doc! {"title": title, "author": author},
                None,
            )
            .await?;
        Ok(())
    }
}


// Validate specific variable field has a value, returning an error if no value
//
fn err_if_none<T>(field: &Option<T>, fieldname: &str)
                  -> Result<(), Box<dyn Error>> {
    match field {
        Some(_) => Ok(()),
        None => Err(format!("Field  `{}` is empty, but is required", fieldname).into()),
    }
}


// Validate specific variable field has a value, returning it, otherwise returning an error
//
fn get_or_err<'a, T>(field: Option<&'a T>, fieldname: &str)
                     -> Result<&'a T, Box<dyn Error>> {
    match field {
        Some(val) => Ok(val),
        None => Err(format!("Field  `{}` is empty, but is required", fieldname).into()),
    }
}

