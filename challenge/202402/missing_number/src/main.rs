fn missing_number(nums: Vec<i32>) -> i32 {
    let n = nums.len() as i32;
    let mut ret = n;

    for i in 0..n {
        ret ^= i;
    }

    for num in nums {
        ret ^= num;
    }

    ret
}

fn main() {
    let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
    let ret = missing_number(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![3, 0, 1];
        let ret = missing_number(nums);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![0, 1];
        let ret = missing_number(nums);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        let ret = missing_number(nums);
        assert_eq!(ret, 8);
    }
}
