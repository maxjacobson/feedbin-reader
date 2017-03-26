#[derive(Debug, Deserialize)]
pub struct Subscription {
    created_at: String,
    feed_id: i32,
    feed_url: String,
    id: i32,
    site_url: String,
    title: String,
}
