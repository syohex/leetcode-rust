fn largest_good_integer(num: String) -> String {
    let (mut candidates, _, _) = num
        .chars()
        .fold((vec![], ' ', 0), |(mut acc, prev, count), c| {
            if prev == c {
                if count == 2 {
                    acc.push(c);
                    (acc, ' ', 0)
                } else {
                    (acc, prev, count + 1)
                }
            } else {
                (acc, c, 1)
            }
        });

    candidates.sort_unstable_by(|a, b| b.cmp(&a));

    if candidates.is_empty() {
        "".to_string()
    } else {
        let mut ret = String::new();
        for _ in 0..3 {
            ret.push(candidates[0]);
        }
        ret
    }
}

fn main() {
    let num = "6777133339".to_string();
    let ret = largest_good_integer(num);
    println!("ret={ret}");
}

#[test]
fn test_largest_good_integer() {
    {
        let num = "6777133339".to_string();
        let expected = "777".to_string();
        let ret = largest_good_integer(num);
        assert_eq!(ret, expected);
    }
    {
        let num = "2300019".to_string();
        let expected = "000".to_string();
        let ret = largest_good_integer(num);
        assert_eq!(ret, expected);
    }
    {
        let num = "42352338".to_string();
        let expected = "".to_string();
        let ret = largest_good_integer(num);
        assert_eq!(ret, expected);
    }
}
