fn tuple_same_product(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut products = HashMap::new();
    let len = nums.len();
    for i in 0..len {
        for j in (i + 1)..len {
            *products.entry(nums[i] * nums[j]).or_insert(0) += 1;
        }
    }

    products.values().fold(0, |acc, &n| {
        if n == 1 {
            acc
        } else {
            let combinations = (n * (n - 1)) / 2;
            acc + combinations * 8
        }
    })
}

fn main() {
    let nums = vec![2, 3, 4, 6];
    let ret = tuple_same_product(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![2, 3, 4, 6];
        let ret = tuple_same_product(nums);
        assert_eq!(ret, 8);
    }
    {
        let nums = vec![1, 2, 4, 5, 10];
        let ret = tuple_same_product(nums);
        assert_eq!(ret, 16);
    }
}
