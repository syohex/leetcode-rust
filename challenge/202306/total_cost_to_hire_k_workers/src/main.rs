fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut left = BinaryHeap::new();
    for i in 0..candidates {
        left.push(Reverse(costs[i as usize] as i64));
    }

    let mut right = BinaryHeap::new();
    let start = std::cmp::max(candidates, costs.len() as i32 - candidates);
    for i in start..costs.len() as i32 {
        right.push(Reverse(costs[i as usize] as i64));
    }

    let mut ret = 0i64;
    let mut next_left = candidates;
    let mut next_right = costs.len() as i32 - 1 - candidates;

    for _ in 0..k {
        if right.is_empty()
            || (!left.is_empty() && left.peek().unwrap().0 <= right.peek().unwrap().0)
        {
            ret += left.pop().unwrap().0;
            if next_left <= next_right {
                left.push(Reverse(costs[next_left as usize] as i64));
                next_left += 1;
            }
        } else {
            ret += right.pop().unwrap().0;
            if next_left <= next_right {
                right.push(Reverse(costs[next_right as usize] as i64));
                next_right -= 1;
            }
        }
    }

    ret
}

fn main() {
    let costs = vec![17, 12, 10, 2, 7, 2, 11, 20, 8];
    let ret = total_cost(costs, 3, 4);
    println!("ret={ret}");
}

#[test]
fn test_total_cost() {
    {
        let costs = vec![17, 12, 10, 2, 7, 2, 11, 20, 8];
        let ret = total_cost(costs, 3, 4);
        assert_eq!(ret, 11);
    }
    {
        let costs = vec![1, 2, 4, 1];
        let ret = total_cost(costs, 3, 3);
        assert_eq!(ret, 4);
    }
}
