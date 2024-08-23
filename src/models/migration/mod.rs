pub mod employee_migration;
pub mod follow_migration;
pub mod story_migration;

use mongodb::Client;

pub async fn run_migrations(client: &Client) {
    employee_migration::create_employee_indexes(client).await;
    follow_migration::create_follow_indexes(client).await;
    story_migration::create_story_indexes(client).await;
}
