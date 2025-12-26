fn best_closing_time(customers: String) -> i32 {
    let mut penalties = customers
        .chars()
        .fold(0, |acc, c| if c == 'Y' { acc + 1 } else { acc });
    let mut min_penalties = penalties;
    let mut min_hour = 0;

    for (i, c) in customers.chars().into_iter().enumerate() {
        if c == 'Y' {
            penalties -= 1;
        } else {
            penalties += 1;
        }

        if penalties < min_penalties {
            min_penalties = penalties;
            min_hour = (i + 1) as i32;
        }
    }

    min_hour
}

fn main() {
    let ret = best_closing_time("YYNY".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(best_closing_time("YYNY".to_string()), 2);
    assert_eq!(best_closing_time("NNNNN".to_string()), 0);
    assert_eq!(best_closing_time("YYYY".to_string()), 4);
}
