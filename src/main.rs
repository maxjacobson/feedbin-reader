// TODO
// - restructure the non- main.rs code into a nested crate, which I think we
//   might need for some code isolation purposes?

extern crate rustyline;
extern crate xdg;
#[macro_use]
extern crate serde_derive;
extern crate toml;

#[macro_use]
extern crate error_chain;

use std::fs::File;
use std::io::prelude::*;

mod errors {
    error_chain!{}
}

use errors::*;

#[derive(Debug, Deserialize, Serialize)]
struct Credentials {
    email: String,
    password: String,
}

impl Credentials {
    fn new() -> Result<Credentials> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("feedbin-reader").
            chain_err(|| "Couldn't setup config directory")?;

        match xdg_dirs.find_config_file("feedbin-reader.toml") {
            Some(config_path) => {
                let mut f = File::open(config_path).chain_err(|| "Couldn't open config file path")?;
                let mut s = String::new();
                f.read_to_string(&mut s).chain_err(|| "Couldn't read config file")?;

                Ok(toml::from_str(&s).chain_err(|| "Couldn't parse config file")?)
            }
            None => {
                let mut rl = rustyline::Editor::<()>::new();
                println!("Enter email:");
                let email = rl.readline("> ").chain_err(|| "Couldn't read email")?;
                println!("Enter password:");
                let password = rl.readline("> ").chain_err(|| "Couldn't read password")?;

                let creds = Credentials {
                    email: email,
                    password: password,
                };

                let new_config_path = xdg_dirs.place_config_file("feedbin-reader.toml")
                    .chain_err(|| "Couldn't create path to config file")?;

                let mut config_file =
                    File::create(new_config_path).chain_err(|| "Couldn't create config file")?;

                let serialized = toml::to_string(&creds).
                    chain_err(|| "Couldn't serialize configuration as toml")?;

                config_file.write_all(serialized.as_bytes())
                    .chain_err(|| "Couldn't write configuration to file")?;

                Ok(creds)
            }
        }
    }
}


// TODO: flesh out this interface to be a fully-featured RSS client with a
//       readline interface
// TODO: but _then_ go ahead and try to make it a more beautiful TUI using
//       something like termion (so you don't need to press enter after each
//       command, and so you have control over the whole screen)
fn main() {
    match Credentials::new() {
        Ok(credentials) => {
            println!("{:?}", credentials);
        }
        Err(ref e) => {
            println!("error: {}", e);

            for e in e.iter().skip(1) {
                println!("caused by: {}", e);
            }

            // The backtrace is not always generated. Try to run this example
            // with `RUST_BACKTRACE=1`.
            if let Some(backtrace) = e.backtrace() {
                println!("backtrace: {:?}", backtrace);
            }
        }
    }

    // next steps
    // - make a user
    // - confirm the user's credentials are correct
    //      - if not, instruct them to check the config file and bail out
    //
    // - start a command loop
    // - add a "sync" command
    //      - downloads subscriptions and puts them in database if they already
    //        exist
    //      - remove subscriptions from database if they no longer exist on
    //        server
    //      - adds a sync-event record with time and success status
    //      - (in the future) submits any pending events done locally, such as
    //        deleting a subscription. This should automatically run when the
    //        user does C-C or C-D, etc
    //  - add a sync-history-audit-log command
    //  - add a "list subscriptions" command
}
