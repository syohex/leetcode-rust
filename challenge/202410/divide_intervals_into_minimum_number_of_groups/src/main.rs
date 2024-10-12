fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
    let mut v = vec![];
    for interval in intervals {
        v.push((interval[0], 1));
        v.push((interval[1] + 1, -1));
    }

    v.sort_unstable();

    let mut ret = 0;
    let mut duplicates = 0;
    for (_, n) in v {
        duplicates += n;
        ret = std::cmp::max(ret, duplicates);
    }

    ret
}

fn main() {
    let intervals = vec![vec![5, 10], vec![6, 8], vec![1, 5], vec![2, 3], vec![1, 10]];
    let ret = min_groups(intervals);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let intervals = vec![vec![5, 10], vec![6, 8], vec![1, 5], vec![2, 3], vec![1, 10]];
        let ret = min_groups(intervals);
        assert_eq!(ret, 3);
    }
    {
        let intervals = vec![vec![1, 3], vec![5, 6], vec![8, 10], vec![11, 13]];
        let ret = min_groups(intervals);
        assert_eq!(ret, 1);
    }
}
