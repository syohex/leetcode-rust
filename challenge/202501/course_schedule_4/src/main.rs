fn check_if_prerequisite(
    num_courses: i32,
    prerequisites: Vec<Vec<i32>>,
    queries: Vec<Vec<i32>>,
) -> Vec<bool> {
    use std::collections::VecDeque;

    let n = num_courses as usize;
    let mut graph: Vec<Vec<_>> = vec![vec![]; n];

    for v in &prerequisites {
        graph[v[0] as usize].push(v[1] as usize);
    }

    let mut is_reachable = vec![vec![false; n]; n];

    for i in 0..n {
        let mut q = VecDeque::new();
        q.push_back(i);
        is_reachable[i][i] = true;
        let mut visited = vec![false; n];

        while !q.is_empty() {
            let len = q.len();

            for _ in 0..len {
                let node = q.pop_front().unwrap();
                visited[node] = true;

                for &next in &graph[node] {
                    if !visited[next] {
                        is_reachable[i][next] = true;
                        q.push_back(next);
                    }
                }
            }
        }
    }

    queries.into_iter().fold(vec![], |mut acc, v| {
        acc.push(is_reachable[v[0] as usize][v[1] as usize]);
        acc
    })
}

fn main() {
    let num_courses = 3;
    let prerequisites = vec![vec![1, 2], vec![1, 0], vec![2, 0]];
    let queries = vec![vec![1, 0], vec![1, 2]];
    let ret = check_if_prerequisite(num_courses, prerequisites, queries);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0]];
        let queries = vec![vec![0, 1], vec![1, 0]];
        let expected = vec![false, true];
        let ret = check_if_prerequisite(num_courses, prerequisites, queries);
        assert_eq!(ret, expected);
    }
    {
        let num_courses = 2;
        let prerequisites = vec![];
        let queries = vec![vec![1, 0], vec![0, 1]];
        let expected = vec![false, false];
        let ret = check_if_prerequisite(num_courses, prerequisites, queries);
        assert_eq!(ret, expected);
    }
    {
        let num_courses = 3;
        let prerequisites = vec![vec![1, 2], vec![1, 0], vec![2, 0]];
        let queries = vec![vec![1, 0], vec![1, 2]];
        let expected = vec![true, true];
        let ret = check_if_prerequisite(num_courses, prerequisites, queries);
        assert_eq!(ret, expected);
    }
}
