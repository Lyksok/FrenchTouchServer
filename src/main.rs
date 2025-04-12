use ft_server::api;
use ft_server::db;
use std::io::Error;
use text_io::read;

fn main() -> Result<(), Error> {
    println!("Welcome to FrenchTouchServer, what would you like to do ?");
    loop {
        println!();
        println!("0. Exit");
        println!("1. Run the server");
        println!("2. Interact with database");
        print!("Your option (0..2): ");
        let input = read!();

        let e = match input {
            0 => return Ok(()),
            1 => api::run_api::run_api(),
            2 => db::db_main::db_main(),
            _ => {
                println!("Option does not exist.");
                Ok(())
            }
        };
        match e {
            Err(e) => return Err(e),
            Ok(_) => continue,
        }
    }
}
