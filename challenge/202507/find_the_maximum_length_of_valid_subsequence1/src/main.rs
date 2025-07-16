fn maximum_length(nums: Vec<i32>) -> i32 {
    let odds = nums.iter().filter(|&n| n % 2 == 1).count() as i32;
    let evens = nums.iter().filter(|&n| n % 2 == 0).count() as i32;

    let mut state = nums[0] % 2 == 0;
    let mut even_odd = 1;
    for &n in nums.iter().skip(1) {
        let is_even = n % 2 == 0;
        if is_even != state {
            even_odd += 1;
            state = !state;
        }
    }

    std::cmp::max(odds, std::cmp::max(evens, even_odd))
}

fn main() {
    let nums = vec![1, 2, 3, 4];
    let ret = maximum_length(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 2, 3, 4];
        let ret = maximum_length(nums);
        assert_eq!(ret, 4);
    }
    {
        let nums = vec![1, 2, 1, 1, 2, 1, 2];
        let ret = maximum_length(nums);
        assert_eq!(ret, 6);
    }
    {
        let nums = vec![1, 3];
        let ret = maximum_length(nums);
        assert_eq!(ret, 2);
    }
}
