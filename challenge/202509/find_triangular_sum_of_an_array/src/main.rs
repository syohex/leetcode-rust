fn triangular_sum(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let len = nums.len();

    for i in (1..len).rev() {
        let mut tmp = vec![0; i];
        for j in 0..i {
            tmp[j] = (nums[j] + nums[j + 1]) % 10;
        }

        nums = tmp;
    }

    nums[0]
}

fn main() {
    let ret = triangular_sum(vec![1, 2, 3, 4, 5]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(triangular_sum(vec![1, 2, 3, 4, 5]), 8);
    assert_eq!(triangular_sum(vec![5]), 5);
}
