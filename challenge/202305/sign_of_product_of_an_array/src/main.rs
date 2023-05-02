fn array_sign(nums: Vec<i32>) -> i32 {
    let mut ret = 1;
    for num in nums {
        if num == 0 {
            return 0;
        } else if num < 0 {
            ret *= -1;
        }
    }

    ret
}

fn main() {
    let nums = vec![-1, -2, -3, -4, 3, 2, 1];
    let ret = array_sign(nums);
    println!("ret={ret}");
}

#[test]
fn test_array_sign() {
    {
        let nums = vec![-1, -2, -3, -4, 3, 2, 1];
        let ret = array_sign(nums);
        assert_eq!(ret, 1);
    }
    {
        let nums = vec![1, 5, 0, 2, -3];
        let ret = array_sign(nums);
        assert_eq!(ret, 0);
    }
    {
        let nums = vec![-1, 1, -1, 1, -1];
        let ret = array_sign(nums);
        assert_eq!(ret, -1);
    }
}
