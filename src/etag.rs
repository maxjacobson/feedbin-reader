extern crate hyper;
use std;

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
