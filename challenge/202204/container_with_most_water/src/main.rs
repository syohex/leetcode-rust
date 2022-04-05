fn max_area(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;

    let mut ret = std::i32::MIN;
    while left < right {
        let h = std::cmp::min(height[left], height[right]);
        let w = (right - left) as i32;

        ret = std::cmp::max(ret, h * w);

        if height[left] < height[right] {
            left += 1;
        } else if height[left] > height[right] {
            right -= 1;
        } else {
            left += 1;
            right -= 1;
        }
    }

    ret
}

fn main() {
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let ret = max_area(height);
    println!("ret={ret}");
}

#[test]
fn test_max_area() {
    {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(max_area(height), 49);
    }
    {
        let height = vec![1, 1];
        assert_eq!(max_area(height), 1);
    }
}
