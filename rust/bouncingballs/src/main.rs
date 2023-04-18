fn bouncing_ball(h: f64,  bounce: f64,  window: f64) -> i32 {
    if h<=0f64 || bounce <= 0f64 || bounce >= 1f64 || window >= h || window <= 0f64 {
        return -1;
    }
    let mut bounce_height = h * bounce;
    let mut counter = 1;
    while bounce_height > window {
        counter+=2;
        bounce_height *= bounce;
    }
    counter
}

fn testequal(h: f64,  bounce: f64,  window: f64, exp: i32) -> () {
    assert_eq!(bouncing_ball(h, bounce, window), exp)
}

#[test]
fn tests_bouncing_ball() {

    testequal(3.0, 0.66, 1.5, 3);
    testequal(30.0, 0.66, 1.5, 15);
    testequal(40.0, 0.4, 10.0, 3);
    testequal(10.0, 0.6, 10.0, -1);
  
}
