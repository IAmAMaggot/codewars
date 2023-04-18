fn parse(code: &str) -> Vec<i32> {
    let mut out = Vec::new();
    let mut val: i32 = 0;
    for command in code.chars() {
        match command {
            'i' => val+=1,
            'd' => val-=1,
            's' => val=val.pow(2),
            'o' => out.push(val),
             _ => {},
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_tests() {
        assert_eq!(parse("iiisdoso"),
            vec![8, 64]);
        assert_eq!(parse("iiisdosodddddiso"),
            vec![8, 64, 3600]);
    }
}
