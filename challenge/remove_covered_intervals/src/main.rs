fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    let mut intervals = intervals;
    intervals.sort_unstable_by(|a, b| {
        if a[0] != b[0] {
            a[0].cmp(&b[0])
        } else {
            b[1].cmp(&a[1])
        }
    });

    let mut start = intervals[0][0];
    let mut end = intervals[0][1];
    let len = intervals.len();

    let mut removed = 0;
    for v in intervals.into_iter().skip(1) {
        if start <= v[0] && v[1] <= end {
            removed += 1;
        }

        if v[1] > end {
            start = v[0];
            end = v[1];
        }
    }

    (len - removed) as i32
}

fn main() {
    let intervals = vec![vec![1, 4], vec![3, 6], vec![2, 8]];
    let ret = remove_covered_intervals(intervals);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let intervals = vec![vec![1, 2], vec![1, 4], vec![3, 4]];
        let ret = remove_covered_intervals(intervals);
        assert_eq!(ret, 1);
    }
    {
        let intervals = vec![vec![1, 4], vec![3, 6], vec![2, 8]];
        let ret = remove_covered_intervals(intervals);
        assert_eq!(ret, 2);
    }
    {
        let intervals = vec![vec![1, 4], vec![2, 3]];
        let ret = remove_covered_intervals(intervals);
        assert_eq!(ret, 1);
    }
}
