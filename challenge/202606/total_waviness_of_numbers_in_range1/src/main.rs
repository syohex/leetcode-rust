fn total_waviness(num1: i32, num2: i32) -> i32 {
    (num1..=num2)
        .map(|n| n.to_string().chars().collect::<Vec<char>>())
        .filter(|cs| cs.len() >= 3)
        .fold(0, |acc, cs| {
            let mut ret = 0;
            for i in 1..(cs.len() - 1) {
                if (cs[i - 1] < cs[i] && cs[i] > cs[i + 1])
                    || (cs[i - 1] > cs[i] && cs[i] < cs[i + 1])
                {
                    ret += 1;
                }
            }
            acc + ret
        })
}

fn main() {
    let ret = total_waviness(4848, 4848);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(total_waviness(120, 130), 3);
    assert_eq!(total_waviness(198, 202), 3);
    assert_eq!(total_waviness(4848, 4848), 2);
}
