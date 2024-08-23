use mongodb::{Client, IndexModel, bson::doc};
use mongodb::options::IndexOptions;
use crate::models::schema::Follow;

pub async fn create_follow_indexes(client: &Client) {
    let db = client.database("social_app");
    let collection = db.collection::<Follow>("follows");

    let follower_index = IndexModel::builder()
        .keys(doc! { "follower_id": 1 })
        .options(IndexOptions::builder().build())
        .build();

    let following_index = IndexModel::builder()
        .keys(doc! { "following_id": 1 })
        .options(IndexOptions::builder().build())
        .build();

    let follower_following_index = IndexModel::builder()
        .keys(doc! { "follower_id": 1, "following_id": 1 })
        .options(IndexOptions::builder().unique(true).build())
        .build();

    collection.create_index(follower_index, None).await.unwrap();
    collection.create_index(following_index, None).await.unwrap();
    collection.create_index(follower_following_index, None).await.unwrap();
}