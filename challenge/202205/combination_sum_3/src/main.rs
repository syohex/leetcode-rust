fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    fn f(num: i32, k: usize, n: i32, acc: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
        let sum: i32 = acc.iter().sum();
        if acc.len() == k {
            if sum == n {
                ret.push(acc.clone());
            }
            return;
        }

        if n - sum < num {
            return;
        }

        let limit = std::cmp::min(10, n);
        for j in num..limit {
            acc.push(j);
            f(j + 1, k, n, acc, ret);
            acc.pop();
        }
    }

    let mut ret = vec![];
    let mut acc = vec![];
    let k = k as usize;

    f(1, k, n, &mut acc, &mut ret);

    ret
}

fn main() {
    let ret = combination_sum3(3, 9);
    println!("ret={:?}", ret);
}

#[test]
fn test_combination_sum3() {
    {
        let expected = vec![vec![1, 2, 4]];
        let ret = combination_sum3(3, 7);
        assert_eq!(ret, expected);
    }
    {
        let expected = vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]];
        let ret = combination_sum3(3, 9);
        assert_eq!(ret, expected);
    }
    {
        let ret = combination_sum3(4, 1);
        assert!(ret.is_empty());
    }
    {
        let ret = combination_sum3(2, 18);
        assert!(ret.is_empty());
    }
}
