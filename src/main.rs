extern crate rustyline;
extern crate xdg;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
struct Credentials {
    email: String,
    password: String,
}

impl Credentials {
    // TODO: use error chain here, get rid of pesky unwraps
    fn new() -> Credentials {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("feedbin-reader").unwrap();

        match xdg_dirs.find_config_file("feedbin-reader.toml") {
            Some(config_path) => {
                let mut f = File::open(config_path).unwrap();
                let mut s = String::new();
                f.read_to_string(&mut s).unwrap();

                toml::from_str(&s).unwrap()
            }
            None => {
                let mut rl = rustyline::Editor::<()>::new();
                println!("Enter email:");
                let email = rl.readline("> ").unwrap();
                println!("Enter password:");
                let password = rl.readline("> ").unwrap();

                let creds = Credentials {
                    email: email,
                    password: password,
                };

                let new_config_path = xdg_dirs.place_config_file("feedbin-reader.toml").unwrap();

                let mut config_file = File::create(new_config_path).unwrap();

                let serialized = toml::to_string(&creds).unwrap();
                config_file.write_all(serialized.as_bytes()).unwrap();

                creds
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
    let credentials = Credentials::new();

    println!("{:?}", credentials);

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
