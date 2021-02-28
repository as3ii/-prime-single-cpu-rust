fn is_prime(n: u128, array: &[u128]) -> bool {
    let mut res: bool = true;
    match n {
        0 | 1 => res = false,
        2 | 3 => res = true,
        _ if n % 2 == 0 => res = false,
        _ => {
            #[allow(
                clippy::cast_sign_loss,
                clippy::cast_precision_loss,
                clippy::cast_possible_truncation
            )]
            let sqrtn: u128 = ((n as f64).sqrt() as u128) + 1;
            let mut i: usize = 0;
            while (i < array.len()) && (array[i] <= sqrtn) && res {
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

pub fn print_vec(array: &[u128]) {
    println!("{:?}", array);
}
