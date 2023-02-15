#![warn(clippy::all, clippy::pedantic)]

use std::env;

use credential::Credential;

mod cli;
mod credential;

const DEFAULT_FILENAME: &str = ".credentials.db";

/// TODO
/// - config
/// - encryption
fn main() {
    let args: Vec<String> = env::args().into_iter().collect();
    let home_path = env::var("HOME").unwrap();
    let path_to_db = format!("{home_path}/{DEFAULT_FILENAME}");
    match args.get(1) {
        None => {
            let email = cli::read_email();
            let password = cli::read_password();
            Credential::new(None, email, password)
                .write(&path_to_db)
                .unwrap();
        }
        Some(flag) => match flag.as_str() {
            "-l" => {
                let credentials = Credential::read(&path_to_db).unwrap_or_default();
                println!("id|email|password");
                for credential in credentials {
                    credential.print();
                }
            }
            "-d" => {
                let id = cli::read_id_for_delete();
                Credential::delete(id, &path_to_db).unwrap();
            }
            "-u" => {
                let id = cli::read_id_for_update();
                let email = cli::read_email();
                let password = cli::read_password();
                Credential::new(Some(id), email, password)
                    .update(&path_to_db)
                    .unwrap();
            }
            _ => todo!(),
        },
    }
}
