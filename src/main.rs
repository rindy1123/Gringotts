#![warn(clippy::all, clippy::pedantic)]

use std::env;

use credential::Credential;

mod cli;
mod credential;

const DEFAULT_FILENAME: &str = ".credentials.db";

fn main() {
    let args: Vec<String> = env::args().into_iter().collect();
    let home_path = env::var("HOME").unwrap();
    let path_to_db = format!("{home_path}/{DEFAULT_FILENAME}");
    match args.get(1) {
        None => {
            let email = cli::read_email();
            let password = cli::read_password();
            Credential::new(email, password).write(&path_to_db).unwrap();
        }
        Some(flag) => {
            if flag == "-l" {
                let credentials = Credential::read(&path_to_db).unwrap_or_default();
                println!("email|password");
                for credential in credentials {
                    credential.print();
                }
            }
        }
    }
}
