mod lib;
use std::env;
use std::io::{stdin, stdout, Write};
use std::time::Instant;

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

    let start_time = Instant::now();

    lib::calc_prime(0, max, &mut result);

    println!(
        "Proces finished in {}s {}ms",
        start_time.elapsed().as_secs(),
        start_time.elapsed().subsec_millis()
    );

    print!("Do you want to see te result? [y/n] ");
    stdout().flush()?;
    let mut response = String::new();
    stdin()
        .read_line(&mut response)
        .expect("Failed to read line");
    let len = response.trim_right().len();
    response.truncate(len);
    if response == "y" {
        lib::print_vec(&result);
    }
    Ok(())
}
