fn maximum_possible_size(nums: Vec<i32>) -> i32 {
    let mut ret = 0;
    let mut prev = 0;

    for num in nums {
        if num >= prev {
            prev = num;
            ret += 1;
        }
    }

    ret
}

fn main() {
    let nums = vec![4, 2, 5, 3, 5];
    let ret = maximum_possible_size(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![4, 2, 5, 3, 5];
        let ret = maximum_possible_size(nums);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![1, 2, 3];
        let ret = maximum_possible_size(nums);
        assert_eq!(ret, 3);
    }
}
