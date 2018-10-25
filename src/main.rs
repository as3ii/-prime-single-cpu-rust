mod lib;
use std::env;
use std::io::{stdin, stdout, Write};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let max: u128;
    if args.len() != 2 {
        let mut response = String::new();
        print!("Write max result: ");
        stdout().flush()?;
        stdin()
            .read_line(&mut response)
            .expect("Failed to read line");
        let len = response.trim_right().len();
        response.truncate(len);
        match response.parse::<u128>() {
            Err(err) => panic!("Give only unsigned integer as argument; Error: {:?}", err),
            Ok(n) => max = n,
        }
    } else {
        match args[1].parse::<u128>() {
            Err(err) => panic!("Give only unsigned integer as argument; Error: {:?}", err),
            Ok(n) => max = n,
        }
    }

    let mut result: Vec<u128> = Vec::new();

    lib::calc_prime(0, max, &mut result);
    print!("Do you want to see te result? [y/n] ");
    stdout().flush()?;
    let mut response = String::new();
    stdin()
        .read_line(&mut response)
        .expect("Failed to read line");
    println!("{}", response);
    if response == "y\n" {
        lib::print_vec(&result);
    }
    Ok(())
}
