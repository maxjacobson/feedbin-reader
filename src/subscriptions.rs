use etag::Etag;
use subscription::Subscription;

// TODO: add feedbin request id (idk what it is but it's in the response)
// TODO: add last modified time
pub struct Subscriptions {
    pub list: Vec<Subscription>,
    pub etag: Etag,
}

impl Subscriptions {
    pub fn len(&self) -> usize {
        self.list.len()
    }
}
