fn max_area(height: Vec<i32>) -> i32 {
    let mut ret = 0;
    let mut left = 0;
    let mut right = height.len() - 1;

    while left < right {
        let width = right - left;
        ret = std::cmp::max(
            ret,
            std::cmp::min(height[left], height[right]) * width as i32,
        );
        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    ret
}

fn main() {
    let ret = max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(max_area(vec![1, 1]), 1);
}
