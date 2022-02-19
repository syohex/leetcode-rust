fn minimum_deviation(nums: Vec<i32>) -> i32 {
    use std::collections::BinaryHeap;

    let mut min = std::i32::MAX;
    let mut q = BinaryHeap::new();
    for num in &nums {
        if num % 2 == 0 {
            q.push(*num);
            min = std::cmp::min(min, *num);
        } else {
            q.push(num * 2);
            min = std::cmp::min(min, num * 2);
        }
    }

    let mut ret = std::i32::MAX;
    while let Some(n) = q.pop() {
        ret = std::cmp::min(ret, n - min);
        if n % 2 == 1 {
            break;
        }

        q.push(n / 2);
        min = std::cmp::min(min, n / 2);
    }

    ret
}

fn main() {
    let nums = vec![4, 1, 5, 20, 3];
    let ret = minimum_deviation(nums);
    println!("ret={ret}");
}

#[test]
fn test_minimum_deviation() {
    {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(minimum_deviation(nums), 1);
    }
    {
        let nums = vec![4, 1, 5, 20, 3];
        assert_eq!(minimum_deviation(nums), 3);
    }
    {
        let nums = vec![2, 10, 8];
        assert_eq!(minimum_deviation(nums), 3);
    }
}
