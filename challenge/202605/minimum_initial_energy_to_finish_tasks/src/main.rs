fn minimum_effort(tasks: Vec<Vec<i32>>) -> i32 {
    let mut tasks = tasks;
    tasks.sort_unstable_by(|a, b| (a[1] - a[0]).cmp(&(b[1] - b[0])));

    let mut ret = 0;
    for task in tasks {
        ret = std::cmp::max(ret + task[0], task[1]);
    }

    ret
}

fn main() {
    let tasks = vec![vec![1, 2], vec![2, 4], vec![4, 8]];
    let ret = minimum_effort(tasks);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let tasks = vec![
            vec![1, 3],
            vec![2, 4],
            vec![10, 11],
            vec![10, 12],
            vec![8, 9],
        ];
        let ret = minimum_effort(tasks);
        assert_eq!(ret, 32);
    }
    {
        let tasks = vec![vec![1, 2], vec![2, 4], vec![4, 8]];
        let ret = minimum_effort(tasks);
        assert_eq!(ret, 8);
    }
    {
        let tasks = vec![
            vec![1, 7],
            vec![2, 8],
            vec![3, 9],
            vec![4, 10],
            vec![5, 11],
            vec![6, 12],
        ];
        let ret = minimum_effort(tasks);
        assert_eq!(ret, 27);
    }
}
