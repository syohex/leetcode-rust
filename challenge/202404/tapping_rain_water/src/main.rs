fn trap(height: Vec<i32>) -> i32 {
    let len = height.len();
    let mut left = vec![0; len];
    let mut right = vec![0; len];

    left[0] = height[0];
    for i in 1..len {
        left[i] = std::cmp::max(height[i], left[i - 1]);
    }

    right[len - 1] = height[len - 1];
    for i in (0..(len - 1)).rev() {
        right[i] = std::cmp::max(height[i], right[i + 1]);
    }

    let mut ret = 0;
    for i in 1..(len - 1) {
        ret += std::cmp::min(left[i], right[i]) - height[i];
    }

    ret
}

fn main() {
    let height = vec![4, 2, 0, 3, 2, 5];
    let ret = trap(height);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let ret = trap(height);
        assert_eq!(ret, 6);
    }
    {
        let height = vec![4, 2, 0, 3, 2, 5];
        let ret = trap(height);
        assert_eq!(ret, 9);
    }
    {
        let height = vec![0];
        let ret = trap(height);
        assert_eq!(ret, 0);
    }
}
