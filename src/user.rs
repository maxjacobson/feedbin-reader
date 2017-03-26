use errors::*;
use subscription::Subscription;
use subscriptions::Subscriptions;
use etag::Etag;
extern crate hyper;
extern crate hyper_native_tls;
use std::io::Read;
extern crate serde_json;

#[derive(Debug)]
pub struct User {
    pub email: String,
    pub password: String,
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
