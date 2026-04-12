fn traffic_signal(timer: i32) -> String {
    if timer == 0 {
        "Green".to_string()
    } else if timer == 30 {
        "Orange".to_string()
    } else if 30 < timer && timer <= 90 {
        "Red".to_string()
    } else {
        "Invalid".to_string()
    }
}

fn main() {
    println!("ret={}", traffic_signal(90));
}

#[test]
fn test() {
    assert_eq!(traffic_signal(60), "Red");
    assert_eq!(traffic_signal(5), "Invalid");
}
