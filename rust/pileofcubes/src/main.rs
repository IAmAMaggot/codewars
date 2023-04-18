fn find_nb(m: u64) -> i32 {
    let mut volume_left = m;
    let mut number_of_cubes: u64 = 0;
    loop { 
        if volume_left == 0 {
            return number_of_cubes as i32;
        }
        number_of_cubes += 1;
        volume_left = match volume_left.checked_sub(number_of_cubes.pow(3)) {
            Some(i) => i,
            None => return -1,
        };
    }
}

#[cfg(test)]
mod sample_tests {
    use super::find_nb;

    fn do_test(n: u64, exp: i32) {
        assert_eq!(find_nb(n), exp, 
                   "\nYour result (left) did not match expected output (right) for n={n}");
    }

    #[test]
    fn basics_find_nb() {
        let cases = [
            (4,               -1),
            (16,              -1),
            (4183059834009,   2022),
            (24723578342962,  -1),
            (135440716410000, 4824),
            (40539911473216,  3568),
            (26825883955641,  3218),
        ];
        for (n, expected) in cases {
            do_test(n, expected);
        }
    }
    
}
