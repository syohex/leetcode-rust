fn predict_the_winner(nums: Vec<i32>) -> bool {
    fn f(nums: &[i32], left: usize, right: usize, cache: &mut Vec<Vec<i32>>) -> i32 {
        if cache[left][right] != i32::MIN {
            return cache[left][right];
        }

        if left == right {
            return nums[left];
        }

        let left_v = nums[left] - f(nums, left + 1, right, cache);
        let right_v = nums[right] - f(nums, left, right - 1, cache);
        let v = std::cmp::max(left_v, right_v);
        cache[left][right] = v;
        v
    }

    let len = nums.len();
    let mut cache = vec![vec![i32::MIN; len]; len];
    f(&nums, 0, len - 1, &mut cache) >= 0
}

fn main() {
    let ret = predict_the_winner(vec![1, 5, 233, 7]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(!predict_the_winner(vec![1, 5, 2]));
    assert!(predict_the_winner(vec![1, 5, 233, 7]));
}
