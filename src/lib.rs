// TODO: remove duplication in client logic
// TODO: move tests into their own tests directory

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate serde_derive;

mod errors;
mod etag;
mod subscription;
mod subscriptions;
pub mod user;

#[cfg(test)]
mod tests {
    use std::env;
    use user::User;

    #[test]
    fn authenticated() {
        let email = env::var("FEEDBIN_USERNAME").unwrap();
        let password = env::var("FEEDBIN_PASSWORD").unwrap();
        let user = User::new(email, password);

        assert_eq!(user.authenticated().unwrap(), true);
    }

    #[test]
    fn not_authenticated() {
        let email = env::var("FEEDBIN_USERNAME").unwrap();
        let password = String::from("foobar");
        let user = User::new(email, password);

        assert_eq!(user.authenticated().unwrap(), false);
    }

    #[test]
    fn subscriptions() {
        let email = env::var("FEEDBIN_USERNAME").unwrap();
        let password = env::var("FEEDBIN_PASSWORD").unwrap();
        let user = User::new(email, password);

        let subscriptions = user.subscriptions().unwrap();

        assert_ne!(subscriptions.len(), 0);
    }

    #[test]
    #[ignore]
    fn subscriptions_with_date() {
        panic!("TKTK");
    }
}
