fn total_numbers(digits: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let mut s = HashSet::new();
    let len = digits.len();

    for i in 0..len {
        if digits[i] == 0 {
            continue;
        }

        for j in 0..len {
            if i == j {
                continue;
            }

            for k in 0..len {
                if i == k || j == k || digits[k] % 2 == 1 {
                    continue;
                }

                let num = 100 * digits[i] + 10 * digits[j] + digits[k];
                s.insert(num);
            }
        }
    }

    s.len() as i32
}
fn main() {
    let ret = total_numbers(vec![1, 2, 3, 4]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(total_numbers(vec![1, 2, 3, 4]), 12);
    assert_eq!(total_numbers(vec![6, 6, 6]), 1);
    assert_eq!(total_numbers(vec![1, 3, 5]), 0);
}
