fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
    fn f(num: i32, i: i32, n: i32, k: i32, acc: &mut Vec<i32>) {
        if i == n {
            acc.push(num);
            return;
        }

        let last_digit = num % 10;
        if last_digit - k >= 0 {
            let new_num = (10 * num) + (last_digit - k);
            f(new_num, i + 1, n, k, acc);
        }

        if k != 0 && last_digit + k <= 9 {
            let new_num = (10 * num) + (last_digit + k);
            f(new_num, i + 1, n, k, acc);
        }
    }

    let mut ret = vec![];
    for i in 1..=9 {
        f(i, 1, n, k, &mut ret);
    }

    ret
}
fn main() {
    let ret = nums_same_consec_diff(3, 7);
    println!("ret={:?}", ret);
}

#[test]
fn test_nums_same_consec_diff() {
    {
        let expected = vec![181, 292, 707, 818, 929];
        let ret = nums_same_consec_diff(3, 7);
        assert_eq!(ret, expected);
    }
    {
        let expected = vec![
            10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98,
        ];
        let ret = nums_same_consec_diff(2, 1);
        assert_eq!(ret, expected);
    }
}
