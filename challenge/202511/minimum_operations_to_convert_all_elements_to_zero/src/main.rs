fn min_operations(nums: Vec<i32>) -> i32 {
    let mut ret = 0;
    let mut stack = vec![];

    for n in nums {
        while !stack.is_empty() && *stack.last().unwrap() > n {
            stack.pop();
        }

        if n == 0 {
            continue;
        }

        if stack.is_empty() || *stack.last().unwrap() != n {
            stack.push(n);
            ret += 1;
        }
    }

    ret
}

fn main() {
    let nums = vec![1, 2, 1, 2, 1, 2];
    let ret = min_operations(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![0, 2];
        let ret = min_operations(nums);
        assert_eq!(ret, 1);
    }
    {
        let nums = vec![3, 1, 2, 1];
        let ret = min_operations(nums);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![1, 2, 1, 2, 1, 2];
        let ret = min_operations(nums);
        assert_eq!(ret, 4);
    }
}
