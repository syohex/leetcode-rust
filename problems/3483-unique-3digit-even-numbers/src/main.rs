fn total_numbers(digits: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let len = digits.len();
    let mut s = HashSet::new();
    for i in 0..len {
        if digits[i] == 0 {
            continue;
        }
        for j in 0..len {
            if i == j {
                continue;
            }

            for k in 0..len {
                if k == i || k == j || digits[k] % 2 == 1 {
                    continue;
                }
                s.insert(digits[i] * 100 + digits[j] * 10 + digits[k]);
            }
        }
    }
    s.len() as i32
}

fn main() {
    let digits = vec![6, 6, 6];
    let ret = total_numbers(digits);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let digits = vec![1, 2, 3, 4];
        let ret = total_numbers(digits);
        assert_eq!(ret, 12);
    }
    {
        let digits = vec![0, 2, 2];
        let ret = total_numbers(digits);
        assert_eq!(ret, 2);
    }
    {
        let digits = vec![6, 6, 6];
        let ret = total_numbers(digits);
        assert_eq!(ret, 1);
    }
    {
        let digits = vec![1, 3, 5];
        let ret = total_numbers(digits);
        assert_eq!(ret, 0);
    }
}
