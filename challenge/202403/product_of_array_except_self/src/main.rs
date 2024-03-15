fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut left_acc = vec![0; len + 1];
    left_acc[0] = 1;

    for (i, &num) in nums.iter().enumerate() {
        left_acc[i + 1] = left_acc[i] * num;
    }

    let mut right_acc = vec![0; len + 1];
    right_acc[len] = 1;

    for (i, &num) in nums.iter().enumerate().rev() {
        right_acc[i] = right_acc[i + 1] * num;
    }

    let mut ret =vec![];
    for i in 0..len {
        ret.push(left_acc[i] * right_acc[i+1]);
    }

    ret
}

fn main() {
    let nums = vec![1, 2, 3, 4];
    let ret = product_except_self(nums);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 2, 3, 4];
        let expected = vec![24, 12, 8, 6];
        let ret = product_except_self(nums);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![-1, 1, 0, -3, 3];
        let expected = vec![0, 0, 9, 0, 0];
        let ret = product_except_self(nums);
        assert_eq!(ret, expected);
    }
}
