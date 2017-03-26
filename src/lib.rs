extern crate hyper;
extern crate hyper_native_tls;

#[macro_use]
extern crate error_chain;

mod errors {
    error_chain!{}
}

use errors::*;

pub struct User {
    email: String,
    password: String,
}

impl User {
    pub fn new(email: String, password: String) -> User {
        User {
            email: email,
            password: password,
        }
    }

    pub fn authenticated(&self) -> Result<bool> {
        let ssl = hyper_native_tls::NativeTlsClient::new().
            chain_err(|| "Unable to intialize ssl client")?;
        let connector = hyper::net::HttpsConnector::new(ssl);
        let client = hyper::Client::with_connector(connector);

        let mut headers = hyper::header::Headers::new();
        headers.set(hyper::header::Authorization(hyper::header::Basic {
                                                     username: self.email.to_owned(),
                                                     password: Some(self.password.to_owned()),
                                                 }));

        let resp = client.get("https://api.feedbin.com/v2/authentication.json")
            .headers(headers)
            .send()
            .chain_err(|| "Unable to request auth status")?;

        Ok(resp.status == hyper::Ok)
    }
}


#[cfg(test)]
mod tests {
    use std::env;
    use User;

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
}
