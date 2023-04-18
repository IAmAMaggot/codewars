fn converter(x: u64) -> Vec<u64> {
    let mut v = Vec::new();
    let mut reg = x;
    while reg != 0 {
        v.push(reg%10);
        reg /= 10;
    }
    println!("{:?}", v);
    v
}

fn persistence(num: u64) -> u64 {
    let mut counter = 0;
    let mut current = num;
    while current>=10 {
        counter+=1;
        let v = converter(current);
        current = v.iter().fold(1, |prod, val| prod * val);
    }
    counter
}

#[cfg(test)]
mod tests {

    #[test]
    fn sample_tests() {
        assert_eq!(super::persistence(39), 3);
        assert_eq!(super::persistence(4), 0);
        assert_eq!(super::persistence(25), 2);
        assert_eq!(super::persistence(999), 4);
    }
}
