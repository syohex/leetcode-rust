fn max_area(height: Vec<i32>) -> i32 {
    let mut left = 0_usize;
    let mut right = height.len() - 1;

    let mut ret = 0;
    while left < right {
        let h = std::cmp::min(height[left], height[right]);
        let area = h * (right - left) as i32;
        ret = std::cmp::max(ret, area);
        if height[left] > height[right] {
            right -= 1;
        } else {
            left += 1;
        }
    }

    ret
}

fn main() {
    println!(
        "max_area([1,8,6,2,5,4,8,3,7])={}",
        max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7])
    );
}

#[test]
fn test_max_area() {
    assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(max_area(vec![1, 1]), 1);
    assert_eq!(max_area(vec![4, 3, 2, 1, 4]), 16);
    assert_eq!(max_area(vec![1, 2, 1]), 2);
}
