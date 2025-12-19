fn total_waiviness(num1: i32, num2: i32) -> i32 {
    fn count_peak_and_valley(n: i32) -> i32 {
        let s : Vec<_> = n.to_string().bytes().collect();
        let len = s.len();
        let mut ret = 0;
        for i in 1..(len - 1) {
            if (s[i-1] < s[i] && s[i] > s[i + 1]) || (s[i - 1] > s[i] && s[i] < s[i + 1]) {
                ret += 1;
            }
        }

        ret
    }

    (num1..=num2)
    .fold(0, |acc, n| acc + count_peak_and_valley(n))
}

fn main() {
    let ret = total_waiviness(1, 10000);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(total_waiviness(120, 130), 3);
    assert_eq!(total_waiviness(198, 202), 3);
    assert_eq!(total_waiviness(4848, 4848), 2);
}
