fn peak_valley_difference(nums: Vec<i32>) -> i32 {
    let mut peaks = 0;
    let mut valleies = 0;
    for i in 1..(nums.len() - 1) {
        if nums[i - 1] < nums[i] && nums[i] > nums[i + 1] {
            peaks += nums[i];
        } else if nums[i - 1] > nums[i] && nums[i] < nums[i + 1] {
            valleies += nums[i];
        }
    }

    peaks - valleies
}

fn main() {
    let ret = peak_valley_difference(vec![1, 5, 2, 4, 1, 3, 0]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(peak_valley_difference(vec![1, 5, 2, 4, 1, 3, 0]), 9);
    assert_eq!(peak_valley_difference(vec![-5, -6, -1, -2, 5, 3, 2]), 12);
    assert_eq!(peak_valley_difference(vec![6, 5, 2, 5, 5, 3, 6]), -5);
}
