use std::collections::HashMap;

use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};

use crate::model::Book;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn book(&self, _ctx: &Context<'_>, id: Option<usize>) -> Option<Book> {
        match id {
            Some(id) => return get_book(id),
            None => None,
        }
    }
}

/**
 * A temporary helper function to test our schema.
 */
fn get_book(id: usize) -> Option<Book> {
    let mut hm: HashMap<usize, Book> = HashMap::new();
    hm.insert(
        1,
        Book {
            id: 1,
            name: String::from("The name of the wind"),
            genre: String::from("Fantasy"),
            author: String::from("Patrick Rothfuss"),
        },
    );
    hm.get(&id).cloned()
}

/**
 * The Schema at the moment only has one query.
 */
pub type IReadSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
