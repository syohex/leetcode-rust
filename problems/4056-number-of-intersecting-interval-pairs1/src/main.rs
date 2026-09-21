fn count_intersecting_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    let len = intervals.len();
    let mut ret = 0;

    for i in 0..len {
        for j in (i + 1)..len {
            let start = std::cmp::max(intervals[i][0], intervals[j][0]);
            let end = std::cmp::min(intervals[i][1], intervals[j][1]);
            if start <= end {
                ret += 1;
            }
        }
    }

    ret
}

fn main() {
    let ret = count_intersecting_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4]]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(
        count_intersecting_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4]]),
        2
    );
    assert_eq!(
        count_intersecting_intervals(vec![vec![1, 5], vec![2, 4], vec![3, 6]]),
        3
    );
    assert_eq!(
        count_intersecting_intervals(vec![vec![1, 2], vec![3, 4], vec![5, 6]]),
        0
    );
}
