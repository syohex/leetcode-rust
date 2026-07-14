fn seconds_between_times(start_time: String, end_time: String) -> i32 {
    fn to_seconds(s: &str) -> i32 {
        let cs: Vec<i32> = s
            .chars()
            .filter(|&c| c != ':')
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();

        (cs[0] * 10 + cs[1]) * 3600 + (cs[2] * 10 + cs[3]) * 60 + cs[4] * 10 + cs[5]
    }

    to_seconds(&end_time) - to_seconds(&start_time)
}

fn main() {
    let ret = seconds_between_times("12:34:56".to_string(), "13:00:00".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(
        seconds_between_times("01:00:00".to_string(), "01:00:25".to_string()),
        25
    );
    assert_eq!(
        seconds_between_times("12:34:56".to_string(), "13:00:00".to_string()),
        1504
    );
}
