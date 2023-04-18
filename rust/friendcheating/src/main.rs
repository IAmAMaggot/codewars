fn remove_nb(m: i32) -> Vec<(i32, i32)> {
    let mut v = Vec::new();
    let n = m as i64;
    let seq_sum = n * (n + 1) / 2;
    for a in 1..n {
        let b = (seq_sum - a) / (a+1);
        if b <= n && seq_sum-a-b == a*b {
            v.push(((a as i32),(b as i32)));
        }
    }
    v
}

fn testing(n: i32, exp: Vec<(i32, i32)>) -> () {
    assert_eq!(remove_nb(n), exp)
}

#[test]
fn basics_remove_nb() {
    
    testing(26, vec![(15, 21), (21, 15)]);
    testing(100, vec![]);
    testing(101, vec![(55, 91), (91, 55)]);
    testing(102, vec![(70, 73), (73, 70)]);
    
}
