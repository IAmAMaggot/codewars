fn is_perfect_power(n: u64) -> Option<(u64, u32)> {
    let biggest_m = (n as f64).sqrt().floor() as u64;
    let mut current_m = biggest_m;
    let mut current_k: u32 = 2;
    while (current_k as u64) < n {
        while current_m > 1 {
            let mk = match current_m.checked_pow(current_k) {
                Some(i) => i,
                None => return None
            };
            if mk == n { return Some((current_m, current_k)) };
            current_m -= 1;
        }
        current_m = biggest_m;
        current_k += 1;
    }
    None
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::is_perfect_power;
    // #[test]
    // fn rasi_examples() {
    //     assert_eq!(is_perfect_power(7), Some((2, 2)), "4 = 2^2");
    //     assert_eq!(is_perfect_power(9), Some((3, 2)), "9 = 3^2");
    //     assert_eq!(is_perfect_power(5), None, "5 is not a perfect power");
    // }

    #[test]
    fn basic_examples() {
        assert_eq!(is_perfect_power(4), Some((2, 2)), "4 = 2^2");
        assert_eq!(is_perfect_power(9), Some((3, 2)), "9 = 3^2");
        assert_eq!(is_perfect_power(5), None, "5 is not a perfect power");
    }
    
    #[test]
    fn first_perfect_powers() {
        let pp = &[4, 8, 9, 16, 25, 27, 32, 36, 49, 64, 81, 100, 121, 125, 128, 144, 169, 196, 216, 225, 243, 256, 289, 324, 343, 361, 400, 441, 484];
        for p in pp {
            if is_perfect_power(*p) == None {
                assert!(false, "{} wasn't recognized as a perfect power", p)
            } else {
                assert!(true)
            }
        }
    }
}
