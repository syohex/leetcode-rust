fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
    let mut ret = 0i64;
    let mut tmp = 0i64;
    for n in nums {
        if n == 0 {
            tmp += 1;
        } else {
            tmp = 0;
        }

        ret += tmp;
    }

    ret
}

fn main() {
        let nums = vec![0, 0, 0, 2, 0, 0];
        let ret = zero_filled_subarray(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 3, 0, 0, 2, 0, 0, 4];
        let ret = zero_filled_subarray(nums);
        assert_eq!(ret, 6);
    }
    {
        let nums = vec![0, 0, 0, 2, 0, 0];
        let ret = zero_filled_subarray(nums);
        assert_eq!(ret, 9);
    }
    {
        let nums = vec![2, 10, 2019];
        let ret = zero_filled_subarray(nums);
        assert_eq!(ret, 0);
    }
}
