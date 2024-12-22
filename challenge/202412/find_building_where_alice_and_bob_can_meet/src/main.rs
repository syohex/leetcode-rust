fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    use std::cmp::{max, Reverse};
    use std::collections::BinaryHeap;

    let mut ret = vec![-1; queries.len()];
    let mut unresolved = vec![vec![]; heights.len()];
    for (i, q) in queries.into_iter().enumerate() {
        let (a, b) = if q[0] < q[1] {
            (q[0], q[1])
        } else {
            (q[1], q[0])
        };

        if a == b {
            ret[i] = a;
        } else if heights[a as usize] < heights[b as usize] {
            ret[i] = b;
        } else {
            unresolved[b as usize].push((max(heights[a as usize], heights[b as usize]), i));
        }
    }

    let mut q: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
    for (i, height) in heights.into_iter().enumerate() {
        while !q.is_empty() {
            let (h, query_index) = q.peek().unwrap().0;
            if h >= height {
                break;
            }

            ret[query_index] = i as i32;
            q.pop();
        }

        for (h, query_index) in unresolved[i].iter() {
            q.push(Reverse((*h, *query_index)));
        }
    }

    ret
}

fn main() {
    let heights = vec![6, 4, 8, 5, 2, 7];
    let queries = vec![vec![0, 1], vec![0, 3], vec![2, 4], vec![3, 4], vec![2, 2]];
    let ret = leftmost_building_queries(heights, queries);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let heights = vec![6, 4, 8, 5, 2, 7];
        let queries = vec![vec![0, 1], vec![0, 3], vec![2, 4], vec![3, 4], vec![2, 2]];
        let expected = vec![2, 5, -1, 5, 2];
        let ret = leftmost_building_queries(heights, queries);
        assert_eq!(ret, expected);
    }
    {
        let heights = vec![5, 3, 8, 2, 6, 1, 4, 6];
        let queries = vec![vec![0, 7], vec![3, 5], vec![5, 2], vec![3, 0], vec![1, 6]];
        let expected = vec![7, 6, -1, 4, 6];
        let ret = leftmost_building_queries(heights, queries);
        assert_eq!(ret, expected);
    }
}
