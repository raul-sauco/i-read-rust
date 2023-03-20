use async_graphql::SimpleObject;

#[derive(Clone, SimpleObject)]
pub struct Book {
    pub id: usize,
    pub name: String,
    pub genre: String,
    pub author: String,
}
