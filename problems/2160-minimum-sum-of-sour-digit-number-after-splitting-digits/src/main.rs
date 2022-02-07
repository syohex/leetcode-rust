fn minimum_sum(num: i32) -> i32 {
    let mut nums = vec![];
    let mut num = num;

    while num > 0 {
        nums.push(num % 10);
        num /= 10;
    }

    nums.sort();

    (nums[0] * 10 + nums[3]) + (nums[1] * 10 + nums[2])
}

fn main() {
    println!("ret={}", minimum_sum(2932));
}

#[test]
fn test_minimum_sum() {
    assert_eq!(minimum_sum(2932), 52);
    assert_eq!(minimum_sum(4009), 13);
}
