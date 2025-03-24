fn count_days(days: i32, meetings: Vec<Vec<i32>>) -> i32 {
    let mut meetings = meetings;
    meetings.sort_unstable_by(|a, b| {
        if a[0] == b[0] {
            b[1].cmp(&a[1])
        } else {
            a[0].cmp(&b[0])
        }
    });

    let mut merged = vec![];
    let len = meetings.len();
    let mut start = meetings[0][0];
    let mut end = meetings[0][1];
    for i in 1..len {
        let s = meetings[i][0];
        let e = std::cmp::min(end, meetings[i][1]);

        if e >= s {
            end = std::cmp::max(end, meetings[i][1]);
        } else {
            merged.push((start, end));
            start = meetings[i][0];
            end = meetings[i][1];
        }
    }

    merged.push((start, end));
    merged.push((days + 1, days + 1));

    let mut ret = merged[0].0 - 1;
    for i in 1..merged.len() {
        ret += merged[i].0 - merged[i - 1].1 - 1;
    }

    ret
}

fn main() {
    let days = 14;
    let meetings = vec![
        vec![6, 11],
        vec![7, 13],
        vec![8, 9],
        vec![5, 8],
        vec![3, 13],
        vec![11, 13],
        vec![1, 3],
        vec![5, 10],
        vec![8, 13],
        vec![3, 9],
    ];
    let ret = count_days(days, meetings);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let days = 14;
        let meetings = vec![
            vec![6, 11],
            vec![7, 13],
            vec![8, 9],
            vec![5, 8],
            vec![3, 13],
            vec![11, 13],
            vec![1, 3],
            vec![5, 10],
            vec![8, 13],
            vec![3, 9],
        ];
        let ret = count_days(days, meetings);
        assert_eq!(ret, 1);
    }
    {
        let days = 6;
        let meetings = vec![vec![1, 6]];
        let ret = count_days(days, meetings);
        assert_eq!(ret, 0);
    }
    {
        let days = 8;
        let meetings = vec![vec![3, 4], vec![4, 8], vec![2, 5], vec![3, 8]];
        let ret = count_days(days, meetings);
        assert_eq!(ret, 1);
    }
    {
        let days = 10;
        let meetings = vec![vec![5, 7], vec![1, 3], vec![9, 10]];
        let ret = count_days(days, meetings);
        assert_eq!(ret, 2);
    }
    {
        let days = 5;
        let meetings = vec![vec![2, 4], vec![1, 3]];
        let ret = count_days(days, meetings);
        assert_eq!(ret, 1);
    }
}
