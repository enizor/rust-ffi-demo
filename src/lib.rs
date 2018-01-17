// pub extern "C" tells the compiler to create a C Application Binary Inerface
#[no_mangle]
pub extern "C" fn nb_of_perfect(max: i32) -> i32 {
    let mut n = 1;
    let mut res = vec![];
    while n < max {
        if is_perfect(n) {
            res.push(n);
        }
        n += 1;
    }
    res.len() as i32
}

// this function will stay stay 'private' to the library
fn is_perfect(n: i32) -> bool {
    let mut divisor = 1;
    let mut sum = 0;
    while divisor * divisor < n {
        if n % divisor == 0 {
            sum += divisor;
            if n / divisor != divisor {
                sum += n / divisor;
            }
        }
        divisor +=1
    }
    sum == 2*n
}

// Rust makes unit testing simple, by appending to a file or creating a full test.rs test suite
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(is_perfect(8128), true);
        assert_eq!(is_perfect(999999999), false);
        assert_eq!(nb_of_perfect(1000),3)
    }
}
