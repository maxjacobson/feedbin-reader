extern crate rustyline;

// TODO: flesh out this interface to be a fully-featured RSS client with a
//       readline interface
// TODO: but _then_ go ahead and try to make it a more beautiful TUI using
//       something like termion (so you don't need to press enter after each
//       command, and so you have control over the whole screen)
fn main() {
    let mut rl = rustyline::Editor::<()>::new();
    loop {
        match rl.readline("> ") {
            Ok(line) => {
                if line == "a" {
                    println!("OK, checking auth status...");
                } else {
                    println!("Unknown command {:?}", line);
                }
            }
            Err(err) => {
                // TODO: this captures things like C-c and C-d; we should take
                //       this opportunity to cleanup and sync, etc
                println!("Error: {}", err);
                println!("Exiting..");
                break;
            }
        }
    }
}
