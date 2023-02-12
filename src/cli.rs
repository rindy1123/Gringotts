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

pub fn read_id() -> usize {
    print!("Enter the record's ID you want to remove: ");
    io::stdout().flush().unwrap();
    loop {
        let mut id_as_string = String::new();
        io::stdin().read_line(&mut id_as_string).unwrap();
        match id_as_string.trim_end().parse::<usize>() {
            Ok(id) => return id,
            Err(_) => {
                print!("Please enter number; Try again: ");
                io::stdout().flush().unwrap();
            }
        }
    }
}
