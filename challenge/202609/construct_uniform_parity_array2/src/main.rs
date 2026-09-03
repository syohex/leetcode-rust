fn uniform_array(nums: Vec<i32>) -> bool {
    let mut min = i32::MAX;
    let mut has_odd = false;

    for n in nums {
        min = std::cmp::min(min, n);
        if n % 2 == 1 {
            has_odd = true;
        }
    }

    if min % 2 == 1 {
        true
    } else {
        !has_odd
    }
}

fn main() {
    let ret = uniform_array(vec![1,4,7]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(uniform_array(vec![1,4,7]));
    assert!(!uniform_array(vec![2,3]));
    assert!(uniform_array(vec![4,6]));
}
