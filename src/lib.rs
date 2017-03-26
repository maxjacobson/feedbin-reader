// TODO: remove duplication in client logic
// TODO: organize library code into multiple files
// TODO: move tests into their own tests directory

extern crate hyper;
extern crate hyper_native_tls;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate serde_derive;

extern crate serde_json;

mod errors {
    error_chain!{}
}

use errors::*;

use std::io::Read;

#[derive(Clone, Debug)]
pub struct Etag {
    fingerprint: String,
}

impl hyper::header::Header for Etag {
    fn header_name() -> &'static str {
        "ETag"
    }

    fn parse_header(raw: &[Vec<u8>]) -> hyper::Result<Etag> {
        if raw.len() == 1 {
            let line = &raw[0];
            match std::str::from_utf8(&line) {
                Ok(fingerprint) => return Ok(Etag { fingerprint: fingerprint.to_owned() }),
                _ => return Err(hyper::Error::Header),
            }
        }

        Err(hyper::Error::Header)
    }
}

impl hyper::header::HeaderFormat for Etag {
    fn fmt_header(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.fingerprint)
    }
}

#[derive(Debug, Deserialize)]
pub struct Subscription {
    created_at: String,
    feed_id: i32,
    feed_url: String,
    id: i32,
    site_url: String,
    title: String,
}

// TODO: add feedbin request id (idk what it is but it's in the response)
// TODO: add last modified time
pub struct Subscriptions {
    list: Vec<Subscription>,
    pub etag: Etag,
}

impl Subscriptions {
    pub fn len(&self) -> usize {
        self.list.len()
    }
}


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

        let resp = client.get("https://api.feedbin.com/v2/authentication.json")
            .headers(self.basic_auth_headers())
            .send()
            .chain_err(|| "Unable to request auth status")?;

        Ok(resp.status == hyper::Ok)
    }

    // TODO: allow providing etag
    // TODO: allow providing "since" value
    pub fn subscriptions(&self) -> Result<Subscriptions> {
        let ssl = hyper_native_tls::NativeTlsClient::new().
            chain_err(|| "Unable to intialize ssl client")?;
        let connector = hyper::net::HttpsConnector::new(ssl);
        let client = hyper::Client::with_connector(connector);

        let mut resp = client.get("https://api.feedbin.com/v2/subscriptions.json")
            .headers(self.basic_auth_headers())
            .send()
            .chain_err(|| "Unable to request subscriptions")?;


        let mut body = String::new();
        resp.read_to_string(&mut body).chain_err(|| "Unable to read response")?;

        let subscriptions: Vec<Subscription> =
            serde_json::from_str(&body).chain_err(|| "Couldn't deserialize response")?;

        // FIXME: no unwrap please
        // This is an Option, not a Result, so how does that fit into the picture?
        let etag: &Etag = resp.headers.get().unwrap();

        Ok(Subscriptions {
               list: subscriptions,
               etag: etag.to_owned(),
           })
    }

    fn basic_auth_headers(&self) -> hyper::header::Headers {
        let mut headers = hyper::header::Headers::new();
        headers.set(hyper::header::Authorization(hyper::header::Basic {
                                                     username: self.email.to_owned(),
                                                     password: Some(self.password.to_owned()),
                                                 }));

        headers
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
