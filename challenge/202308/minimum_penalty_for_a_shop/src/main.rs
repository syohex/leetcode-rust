fn best_closing_time(customers: String) -> i32 {
    let mut count = customers.chars().filter(|c| *c == 'Y').count();
    let mut min_count = count;
    let mut ret = 0;

    for (i, c) in customers.chars().enumerate() {
        if c == 'Y' {
            count -= 1;
        } else {
            count += 1;
        }

        if count < min_count {
            min_count = count;
            ret = (i + 1) as i32;
        }
    }

    ret
}

fn main() {
    let customers = "YYNY".to_string();
    let ret = best_closing_time(customers);
    println!("ret={ret}");
}

#[test]
fn test_best_closing_time() {
    {
        let customers = "YYNY".to_string();
        let ret = best_closing_time(customers);
        assert_eq!(ret, 2);
    }
    {
        let customers = "NNNNN".to_string();
        let ret = best_closing_time(customers);
        assert_eq!(ret, 0);
    }
    {
        let customers = "YYYY".to_string();
        let ret = best_closing_time(customers);
        assert_eq!(ret, 4);
    }
}
