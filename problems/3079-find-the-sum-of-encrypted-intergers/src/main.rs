fn sum_of_encrypted_int(nums: Vec<i32>) -> i32 {
    fn encrypt(num: i32) -> i32 {
        let mut largest = 0;
        let mut n = num;
        while n > 0 {
            largest = std::cmp::max(largest, n % 10);
            n /= 10;
        }

        let mut ret = 0;
        n = num;
        let mut base = 1;
        while n > 0 {
            ret += base * (largest % 10);
            base *= 10;
            n /= 10;
        }

        ret
    }

    nums.into_iter().map(encrypt).sum()
}

fn main() {
    let nums = vec![10, 21, 31];
    let ret = sum_of_encrypted_int(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 2, 3];
        let ret = sum_of_encrypted_int(nums);
        assert_eq!(ret, 6);
    }
    {
        let nums = vec![10, 21, 31];
        let ret = sum_of_encrypted_int(nums);
        assert_eq!(ret, 66);
    }
}
