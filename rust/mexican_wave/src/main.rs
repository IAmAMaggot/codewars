fn wave(s: &str) -> Vec<String> {
    let mut vec = vec![];
    for (i, c) in s.chars().enumerate() {
        let mut gen_word = "".to_string();
        if c == ' ' {
            continue;
        }
        for (j, d) in s.chars().enumerate() {
            if i==j {
                gen_word += &d.to_uppercase().to_string();
            } else {
                gen_word += &d.to_string();
            }
        }
        vec.push(gen_word);
    }
    vec
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let expect = ["Hello", "hEllo", "heLlo", "helLo", "hellO"];
        assert_eq!(wave("hello"), expect);
        
        let expect = ["Codewars", "cOdewars", "coDewars", "codEwars", "codeWars", "codewArs", "codewaRs", "codewarS"];
        assert_eq!(wave("codewars"), expect);
        
        let expect: [&str; 0] = [];
        assert_eq!(wave(""), expect);
        
        let expect = ["Two words", "tWo words", "twO words", "two Words", "two wOrds", "two woRds", "two worDs", "two wordS"];
        assert_eq!(wave("two words"), expect);
        
        let expect = [" Gap ", " gAp ", " gaP "];
        assert_eq!(wave(" gap "), expect);
    }
}
