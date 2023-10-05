fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    let mut c1 = 0;
    let mut c2 = 0;
    let mut n1 = std::i32::MIN;
    let mut n2 = std::i32::MIN;

    for &num in &nums {
        if n1 == num {
            c1 += 1;
        } else if n2 == num {
            c2 += 1;
        } else if c1 == 0 {
            n1 = num;
            c1 = 1;
        } else if c2 == 0 {
            n2 = num;
            c2 = 1;
        } else {
            c1 -= 1;
            c2 -= 1;
        }
    }

    c1 = 0;
    c2 = 0;
    for &num in &nums {
        if num == n1 {
            c1 += 1;
        } else if num == n2 {
            c2 += 1;
        }
    }

    let mut ret = vec![];
    if c1 > nums.len() / 3 {
        ret.push(n1);
    }
    if c2 > nums.len() / 3 {
        ret.push(n2);
    }

    ret
}

fn main() {
    let nums = vec![3, 2, 3];
    let ret = majority_element(nums);
    println!("ret={ret:?}");
}

#[test]
fn test_majority_element() {
    {
        let nums = vec![3, 2, 3];
        let expected = vec![3];
        let ret = majority_element(nums);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![1];
        let expected = vec![1];
        let ret = majority_element(nums);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![1, 2];
        let expected = vec![1, 2];
        let ret = majority_element(nums);
        assert_eq!(ret, expected);
    }
}
