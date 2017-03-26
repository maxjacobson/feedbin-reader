#[macro_use]
extern crate error_chain;
extern crate hyper;
extern crate hyper_native_tls;

mod errors {
    error_chain!{}
}
use errors::*;

#[derive (Debug)]
pub struct User {
    pub email: String,
    pub password: String,
}

impl User {
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
    #[test]
    fn it_works() {
    }
}
