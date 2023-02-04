use std::io::{self, Write};

pub fn read_email() -> String {
    print!("Email(or Username): ");
    io::stdout().flush().unwrap();
    let mut email = String::new();
    io::stdin().read_line(&mut email).unwrap();
    return email.trim_end().to_owned();
}

pub fn read_password() -> String {
    print!("Password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    return password.trim_end().to_owned();
}
