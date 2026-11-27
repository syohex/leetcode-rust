fn count_dominant_elements(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    if len == 1 {
        1
    } else {
        let mut ret = 0;
        for i in 0..len {
            if i == 0 {
                if nums[0] >= nums[1] {
                    ret += 1;
                }
            } else if i == len - 1 {
                if nums[len - 1] >= nums[len - 2] {
                    ret += 1;
                }
            } else {
                let avg = (nums[i - 1] + nums[i + 1]) as f64 / 2.0;
                if nums[i] as f64 >= avg {
                    ret += 1;
                }
            }
        }
        ret
    }
}

fn main() {
    let ret = count_dominant_elements(vec![1, 10, 2]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(count_dominant_elements(vec![96, 75, 55]), 1);
    assert_eq!(count_dominant_elements(vec![1, 10, 2]), 1);
    assert_eq!(count_dominant_elements(vec![6, 4, 2]), 2);
    assert_eq!(count_dominant_elements(vec![7]), 1);
}
