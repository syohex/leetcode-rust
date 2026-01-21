fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
    nums.into_iter()
        .map(|num| {
            if num % 2 == 0 {
                -1
            } else {
                let mut mask = 1;
                for _ in 0..32 {
                    if (num & mask) == 0 {
                        return num & !(mask >> 1);
                    }
                    mask <<= 1;
                }

                -1
            }
        })
        .collect()
}

fn main() {
    let ret = min_bitwise_array(vec![2, 3, 5, 7]);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let ret = min_bitwise_array(vec![2, 3, 5, 7]);
        assert_eq!(ret, vec![-1, 1, 4, 3]);
    }
    {
        let ret = min_bitwise_array(vec![11, 13, 31]);
        assert_eq!(ret, vec![9, 12, 15]);
    }
}
