fn smallest_number(num: i64) -> i64 {
    fn to_nums(num: i64) -> Vec<i64> {
        if num == 0 {
            return vec![0];
        }

        let mut nums = vec![];
        let mut num = num;

        while num > 0 {
            nums.push(num % 10);
            num /= 10;
        }

        nums
    }

    if num < 0 {
        let num = num * -1;
        let mut nums = to_nums(num);
        nums.sort_unstable();

        let ret = nums.into_iter().rev().fold(0, |acc, n| (acc * 10) + n);
        -ret
    } else {
        let mut nums = to_nums(num);
        nums.sort_unstable();

        if let Some(index) = nums.iter().position(|&n| n != 0) {
            let init = nums[index];
            nums.into_iter().enumerate().fold(
                init,
                |acc, (i, n)| {
                    if i == index {
                        acc
                    } else {
                        acc * 10 + n
                    }
                },
            )
        } else {
            0
        }
    }
}

fn main() {
    let ret1 = smallest_number(310);
    let ret2 = smallest_number(-7605);
    println!("ret1={ret1}, ret2={ret2}");
}

#[test]
fn test_smallest_number() {
    assert_eq!(smallest_number(310), 103);
    assert_eq!(smallest_number(-7605), -7650);
    assert_eq!(smallest_number(0), 0);
    assert_eq!(smallest_number(-1), -1);
}
