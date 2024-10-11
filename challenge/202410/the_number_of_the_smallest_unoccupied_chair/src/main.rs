fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let len = times.len();
    let mut v = times
        .into_iter()
        .enumerate()
        .fold(vec![], |mut acc, (i, v)| {
            acc.push((v[0], 1, i));
            acc.push((v[1], -1, i));
            acc
        });
    v.sort_unstable();

    let mut h = BinaryHeap::new();
    for i in 0..len {
        h.push(Reverse(i));
    }

    let mut seats = vec![len + 1; len];
    for (_, j, friend) in v {
        if j == -1 {
            h.push(Reverse(seats[friend]));
        } else {
            if let Some(Reverse(t)) = h.pop() {
                seats[friend] = t;
            } else {
                panic!("never reach here")
            }
        }
    }

    seats[target_friend as usize] as i32
}

fn main() {
    let times = vec![vec![3, 10], vec![1, 5], vec![2, 6]];
    let ret = smallest_chair(times, 1);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let times = vec![vec![1, 4], vec![2, 3], vec![4, 6]];
        let ret = smallest_chair(times, 1);
        assert_eq!(ret, 1);
    }
    {
        let times = vec![vec![3, 10], vec![1, 5], vec![2, 6]];
        let ret = smallest_chair(times, 0);
        assert_eq!(ret, 2);
    }
}
