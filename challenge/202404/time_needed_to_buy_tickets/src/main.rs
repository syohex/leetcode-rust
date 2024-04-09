fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut tickets = tickets;
    let len = tickets.len();
    let mut ret = 0;

    loop {
        for i in 0..len {
            if tickets[i] == 0 {
                continue;
            }

            tickets[i] -= 1;
            ret += 1;

            if i == k && tickets[i] == 0 {
                return ret;
            }
        }
    }
}

fn main() {
    let tickets = vec![5, 1, 1, 1];
    let ret = time_required_to_buy(tickets, 0);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let tickets = vec![2, 3, 2];
        let ret = time_required_to_buy(tickets, 2);
        assert_eq!(ret, 6);
    }
    {
        let tickets = vec![5, 1, 1, 1];
        let ret = time_required_to_buy(tickets, 0);
        assert_eq!(ret, 8);
    }
}
