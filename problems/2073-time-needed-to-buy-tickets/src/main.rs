fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
    let mut tickets = tickets;
    let k = k as usize;

    let mut count = 0;
    loop {
        for (i, n) in tickets.iter_mut().enumerate() {
            if *n == 0 {
                continue;
            }

            *n -= 1;
            count += 1;

            if *n == 0 && i == k {
                return count;
            }
        }
    }
}

fn main() {
    let ret = time_required_to_buy(vec![2, 3, 2], 2);
    println!("ret={}", ret);
}

#[test]
fn test_time_required_to_buy() {
    assert_eq!(time_required_to_buy(vec![2, 3, 2], 2), 6);
    assert_eq!(time_required_to_buy(vec![5, 1, 1, 1], 0), 8);
}
