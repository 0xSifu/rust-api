use mongodb::{Client, IndexModel, bson::doc};
use mongodb::options::IndexOptions;
use crate::models::schema::Story;

pub async fn create_story_indexes(client: &Client) {
    let db = client.database("social_app");
    let collection = db.collection::<Story>("stories");

    let author_index = IndexModel::builder()
        .keys(doc! { "author_id": 1 })
        .options(IndexOptions::builder().build())
        .build();

    let created_at_index = IndexModel::builder()
        .keys(doc! { "created_at": 1 })
        .options(IndexOptions::builder().build())
        .build();

    collection.create_index(author_index, None).await.unwrap();
    collection.create_index(created_at_index, None).await.unwrap();
}