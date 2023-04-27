fn bulb_switch(n: i32) -> i32 {
    (n as f64).sqrt() as i32
}

fn main() {
    println!("ret={}", bulb_switch(3));
}

#[test]
fn test_bulb_switch() {
    assert_eq!(bulb_switch(0), 0);
    assert_eq!(bulb_switch(1), 1);
    assert_eq!(bulb_switch(3), 1);
}
