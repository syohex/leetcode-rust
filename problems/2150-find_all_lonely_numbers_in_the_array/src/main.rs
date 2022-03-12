fn find_lonely(nums: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;

    let mut h = HashMap::new();
    for n in nums {
        *h.entry(n).or_insert(0) += 1;
    }

    h.iter().fold(vec![], |mut acc, (k, v)| {
        if *v == 1 {
            if !h.contains_key(&(*k + 1)) && !h.contains_key(&(*k - 1)) {
                acc.push(*k);
                acc
            } else {
                acc
            }
        } else {
            acc
        }
    })
}

fn main() {
    let nums = vec![10, 6, 5, 8];
    let ret = find_lonely(nums);
    println!("ret={:?}", ret);
}

#[cfg(test)]
mod test {
    use super::*;

    fn check(got: Vec<i32>, expected: Vec<i32>) {
        use std::collections::HashSet;

        assert_eq!(got.len(), expected.len());

        let gs: HashSet<i32> = got.into_iter().collect();
        let es: HashSet<i32> = expected.into_iter().collect();

        assert_eq!(gs, es);
    }

    #[test]
    fn test_find_lonely() {
        {
            let nums = vec![10, 6, 5, 8];
            let expected = vec![10, 8];
            check(find_lonely(nums), expected);
        }
        {
            let nums = vec![1, 3, 5, 3];
            let expected = vec![1, 5];
            check(find_lonely(nums), expected);
        }
    }
}
