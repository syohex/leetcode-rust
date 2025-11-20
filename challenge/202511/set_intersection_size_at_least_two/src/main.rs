fn intersection_size_two(intervals: Vec<Vec<i32>>) -> i32 {
    let mut intervals = intervals;
    intervals.sort_unstable_by(|a, b| {
        if a[1] == b[1] {
            a[0].cmp(&b[0])
        } else {
            a[1].cmp(&b[1])
        }
    });

    let mut times = vec![];
    times.push(intervals[0][1] - 1);
    times.push(intervals[0][1]);

    for v in intervals.into_iter().skip(1) {
        let len = times.len();
        if v[0] > times[len - 1] {
            times.push(v[1] - 1);
            times.push(v[1]);
        } else if v[0] == times[len - 1] {
            times.push(v[1]);
        } else if v[0] > times[len - 2] {
            times.push(v[1]);
        }

    }

    times.len() as i32
}

fn main() {
    let intervals = vec![vec![1, 3], vec![3, 7], vec![8, 9]];
    let ret = intersection_size_two(intervals);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let intervals = vec![vec![1, 3], vec![3, 7], vec![8, 9]];
        let ret = intersection_size_two(intervals);
        assert_eq!(ret, 5);
    }
    {
        let intervals = vec![vec![1, 3], vec![1, 4], vec![2, 5], vec![3, 5]];
        let ret = intersection_size_two(intervals);
        assert_eq!(ret, 3);
    }
    {
        let intervals = vec![vec![1, 2], vec![2, 3], vec![2, 4], vec![4, 5]];
        let ret = intersection_size_two(intervals);
        assert_eq!(ret, 5);
    }
}
