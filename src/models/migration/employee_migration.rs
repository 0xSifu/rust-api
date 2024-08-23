use mongodb::{Client, IndexModel, bson::doc};
use mongodb::options::IndexOptions;
use crate::models::schema::Employee;

pub async fn create_employee_indexes(client: &Client) {
    let db = client.database("social_app");
    let collection = db.collection::<Employee>("employees");

    let email_index = IndexModel::builder()
        .keys(doc! { "email": 1 })
        .options(IndexOptions::builder().unique(true).build())
        .build();

    collection.create_index(email_index, None).await.unwrap();
}
