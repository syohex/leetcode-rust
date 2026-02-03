fn is_trionic(nums: Vec<i32>) -> bool {
    let len = nums.len();
    let mut i = 1;

    let mut p = 0;
    while i < len {
        if nums[i - 1] >= nums[i] {
            p = i - 1;
            break;
        }

        i += 1;
    }

    if p == 0 {
        return false;
    }

    let mut q = len;
    while i < len {
        if nums[i - 1] <= nums[i] {
            q = i - 1;
            break;
        }

        i += 1;
    }

    if q == len {
        return false;
    }

    while i < len {
        if nums[i - 1] >= nums[i] {
            return false;
        }

        i += 1;
    }

    true
}

fn main() {
    let ret = is_trionic(vec![1, 3, 5, 4, 2, 6]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(!is_trionic(vec![3, 7, 1]));
    assert!(is_trionic(vec![1, 3, 5, 4, 2, 6]));
    assert!(!is_trionic(vec![2, 1, 3]));
}
