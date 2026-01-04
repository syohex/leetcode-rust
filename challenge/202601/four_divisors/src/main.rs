fn sum_four_divisors(nums: Vec<i32>) -> i32 {
    let mut ret = 0;
    for num in nums {
        let limit = (num as f64).sqrt() as i32;
        let mut tmp = 1 + num;
        let mut divisors = 2;
        for i in 2..=limit {
            if num % i == 0 {
                divisors += 1;
                let j = num / i;
                if i != j {
                    divisors += 1;
                    tmp += i + j;
                }
            }
        }
        if divisors == 4 {
            ret += tmp;
        }
    }

    ret
}

fn main() {
    let ret = sum_four_divisors(vec![21, 4, 7]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(sum_four_divisors(vec![21, 4, 7]), 32);
    assert_eq!(sum_four_divisors(vec![21, 21]), 64);
    assert_eq!(sum_four_divisors(vec![1, 2, 3, 4, 5]), 0);
}
