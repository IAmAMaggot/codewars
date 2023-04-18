fn is_valid_walk(walk: &[char]) -> bool {
    let takes_ten_minutes = match walk.len() {
        10 => true,
        _ => return false,
    };
    let mut pos = (0i32, 0i32);
    for dir in walk.iter() {
        match dir {
            'n' => pos.0+=1,
            's' => pos.0-=1,
            'e' => pos.1+=1,
            'w' => pos.1-=1,
            _ => {},
        }
    }
    let brings_you_back = pos==(0,0);
    if takes_ten_minutes && brings_you_back {
        return true;
    } else {
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::is_valid_walk;

    #[test]
    fn sample_tests() {
        assert!(  is_valid_walk(&['n','s','n','s','n','s','n','s','n','s']));
        assert!(! is_valid_walk(&['w','e','w','e','w','e','w','e','w','e','w','e']));
        assert!(! is_valid_walk(&['w']));
        assert!(! is_valid_walk(&['n','n','n','s','n','s','n','s','n','s']));
        assert!(! is_valid_walk(&['e', 'e', 'e', 'e', 'w', 'w', 's', 's', 's', 's']))
    }
}
