fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;
    use std::collections::HashMap;

    #[derive(Debug, Eq, PartialEq)]
    struct Data {
        num: i32,
        count: i32,
    }

    impl Ord for Data {
        fn cmp(&self, other: &Self) -> Ordering {
            if self.count != other.count {
                self.count.cmp(&other.count)
            } else {
                other.num.cmp(&self.num)
            }
        }
    }

    impl PartialOrd for Data {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut h = HashMap::new();
    for num in nums {
        *h.entry(num).or_insert(0) += 1;
    }

    let mut q = BinaryHeap::new();
    for (k, v) in h.iter() {
        q.push(Data { num: *k, count: *v });
    }

    let mut ret = vec![];
    for _ in 0..k {
        ret.push(q.pop().unwrap().num);
    }

    ret
}

fn main() {
    let nums = vec![1, 1, 1, 2, 2, 3];
    let ret = top_k_frequent(nums, 2);
    println!("ret={:?}", ret);
}

#[test]
fn test_top_k_frequent() {
    {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let expected = vec![1, 2];
        let ret = top_k_frequent(nums, 2);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![1];
        let expected = vec![1];
        let ret = top_k_frequent(nums, 1);
        assert_eq!(ret, expected);
    }
}
