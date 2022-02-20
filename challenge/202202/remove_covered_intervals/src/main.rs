fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    let mut intervals = intervals;

    intervals.sort_by(|a, b| {
        if a[0] == b[0] {
            return b[1].cmp(&a[1]);
        }
        a[0].cmp(&b[0])
    });

    let mut ret = 0;
    let mut prev = -1;
    for interval in &intervals {
        if prev < interval[1] {
            ret += 1;
            prev = interval[1];
        }
    }

    ret
}

fn main() {
    let intervals = vec![vec![1, 4], vec![3, 6], vec![2, 8]];
    let ret = remove_covered_intervals(intervals);
    println!("ret={ret}");
}

#[test]
fn test_remove_covered_intervals() {
    {
        let intervals = vec![vec![1, 4], vec![3, 6], vec![2, 8]];
        assert_eq!(remove_covered_intervals(intervals), 2);
    }
    {
        let intervals = vec![vec![1, 4], vec![2, 3]];
        assert_eq!(remove_covered_intervals(intervals), 1);
    }
    {
        let intervals = vec![vec![1, 2], vec![1, 4], vec![3, 4]];
        assert_eq!(remove_covered_intervals(intervals), 1);
    }
}
