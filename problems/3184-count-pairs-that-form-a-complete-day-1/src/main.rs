fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
    let mut ret = 0;
    let len = hours.len();

    for i in 0..len {
        for j in (i + 1)..len {
            if (hours[i] + hours[j]) % 24 == 0 {
                ret += 1;
            }
        }
    }
    ret
}

fn main() {
    let hours = vec![12, 12, 30, 24, 24];
    let ret = count_complete_day_pairs(hours);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let hours = vec![12, 12, 30, 24, 24];
        let ret = count_complete_day_pairs(hours);
        assert_eq!(ret, 2);
    }
    {
        let hours = vec![72, 48, 24, 3];
        let ret = count_complete_day_pairs(hours);
        assert_eq!(ret, 3);
    }
}
