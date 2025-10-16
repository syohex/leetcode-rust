fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
    let mut v = vec![0; value as usize];
    for n in nums {
        let n = (n % value + value) % value;
        v[n as usize] += 1;
    }

    let mut ret = 0;
    while v[ret % value as usize] != 0 {
        v[ret % value as usize] -= 1;
        ret += 1;
    }

    ret as i32
}

fn main() {
    let nums = vec![1, -10, 7, 13, 6, 8];
    let ret = find_smallest_integer(nums, 5);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![3, 0, 3, 2, 4, 2, 1, 1, 0, 4];
        let ret = find_smallest_integer(nums, 5);
        assert_eq!(ret, 10);
    }
    {
        let nums = vec![1, -10, 7, 13, 6, 8];
        let ret = find_smallest_integer(nums, 5);
        assert_eq!(ret, 4);
    }
    {
        let nums = vec![1, -10, 7, 13, 6, 8];
        let ret = find_smallest_integer(nums, 7);
        assert_eq!(ret, 2);
    }
}
