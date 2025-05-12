fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
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
                if i == k || j == k || digits[k] % 2 != 0 {
                    continue;
                }

                s.insert(digits[i] * 100 + digits[j] * 10 + digits[k]);
            }
        }
    }

    let mut ret: Vec<_> = s.into_iter().collect();
    ret.sort_unstable();
    ret
}

fn main() {
    let digits = vec![2, 1, 3, 0];
    let ret = find_even_numbers(digits);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let digits = vec![2, 1, 3, 0];
        let expected = vec![102, 120, 130, 132, 210, 230, 302, 310, 312, 320];
        let ret = find_even_numbers(digits);
        assert_eq!(ret, expected);
    }
    {
        let digits = vec![2, 2, 8, 8, 2];
        let expected = vec![222, 228, 282, 288, 822, 828, 882];
        let ret = find_even_numbers(digits);
        assert_eq!(ret, expected);
    }
    {
        let digits = vec![3, 7, 5];
        let expected = vec![];
        let ret = find_even_numbers(digits);
        assert_eq!(ret, expected);
    }
}
