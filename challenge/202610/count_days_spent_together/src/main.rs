fn count_days_together(
    arrive_alice: String,
    leave_alice: String,
    arrive_bob: String,
    leave_bob: String,
) -> i32 {
    fn to_year_days(s: &str) -> i32 {
        let month_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let cs: Vec<_> = s.bytes().collect();
        let month = (cs[0] - b'0') as i32 * 10 + (cs[1] - b'0') as i32;
        let day = (cs[3] - b'0') as i32 * 10 + (cs[4] - b'0') as i32;

        let d: i32 = month_days.iter().take(month as usize - 1).sum();
        d + day
    }

    let alice_from = to_year_days(&arrive_alice);
    let alice_to = to_year_days(&leave_alice);
    let bob_from = to_year_days(&arrive_bob);
    let bob_to = to_year_days(&leave_bob);

    let start = std::cmp::max(alice_from, bob_from);
    let end = std::cmp::min(alice_to, bob_to);
    if end >= start { end - start + 1 } else { 0 }
}

fn main() {
    let ret = count_days_together(
        "08-15".to_string(),
        "08-18".to_string(),
        "08-16".to_string(),
        "08-19".to_string(),
    );
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(
        count_days_together(
            "08-15".to_string(),
            "08-18".to_string(),
            "08-16".to_string(),
            "08-19".to_string()
        ),
        3
    );
    assert_eq!(
        count_days_together(
            "10-01".to_string(),
            "10-31".to_string(),
            "11-01".to_string(),
            "12-31".to_string()
        ),
        0
    );
}
