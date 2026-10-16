fn max_product_pair(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let len = nums.len();
    let mut ret = vec![-1; 2];
    let mut max = i32::MIN;

    for i in 0..len {
        for j in 0..len {
            if (i == j) || (nums[i] <= nums[j]) || (nums[i] + nums[j] != target) {
                continue;
            }

            let m = nums[i] * nums[j];
            if m > max {
                ret[0] = i as i32;
                ret[1] = j as i32;
                max = m;
            }
        }
    }

    ret
}

fn main() {
    let ret = max_product_pair(vec![1, 2, 3, 4], 5);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(max_product_pair(vec![1, 2, 3, 4], 5), vec![2, 1]);
    assert_eq!(max_product_pair(vec![-3, -1, 4, 2], 1), vec![3, 1]);
    assert_eq!(max_product_pair(vec![3, 3, 5], 6), vec![-1, -1]);
}
