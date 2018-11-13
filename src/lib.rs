fn is_prime(n: u128, array: &Vec<u128>) -> bool {
    let mut res: bool = true;
    let sqrtn: usize = ((n as f64).sqrt() as usize) + 1;
    match n {
        0 | 1 => res = false,
        2 | 3 => res = true,
        _ if n % 2 == 0 => res = false,
        _ => {
            let mut i: usize = 0;
            while (i < array.len()) && (i <= sqrtn) && (res == true) {
                if array[i] != 0 && array[i] != 1 && n % array[i] == 0 {
                    res = false;
                }
                i += 1;
            }
        }
    }
    res
}

pub fn calc_prime(start: u128, stop: u128, array: &mut Vec<u128>) {
    let mut var: u128 = start;
    while var < stop {
        if is_prime(var, &array) {
            array.push(var);
        }
        var += 1;
    }
}

pub fn print_vec(array: &Vec<u128>) {
        for n in array.iter() {
            print!("{}   ", n);
        }
        println!("");
}
