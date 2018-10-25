fn is_prime(n: u128, array: &Vec<u128>) -> bool {
    let mut res: bool = true;
    let sqrtn: usize = ((n as f64).sqrt() as usize) + 1;
    if n < 2 {
        res = false;
    } else if n < 4 {
        res = true;
    } else if n % 2 == 0 {
        res = false;
    } else {
        let mut i: usize = 0;
        while (i < array.len()) && (i <= sqrtn) && (res == true) {
            if array[i] != 0 && array[i] != 1 && n % array[i] == 0 {
                res = false;
            }
            i += 1;
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
    let mut i: usize = 0;
    while i < (array.len() - 4 as usize) {
        i += 4;
        println!(
            "{}    {}    {}    {}",
            array[i - 4],
            array[i - 3],
            array[i - 2],
            array[i - 1]
        );
    }
    if i < array.len() {
        for k in i..array.len() {
            print!("{}    ", array[k]);
        }
        println!("");
    }
}
