fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
    fn f(i: usize, graph: &Vec<Vec<usize>>) -> i32 {
        use std::collections::HashSet;

        let mut visited = HashSet::new();
        let mut q = vec![i];

        while !q.is_empty() {
            let pos = q.pop().unwrap();
            visited.insert(pos);

            for next in &graph[pos] {
                if !visited.contains(next) {
                    q.push(*next);
                }
            }
        }

        visited.len() as i32
    }

    let n = bombs.len();
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            let x1 = bombs[i][0];
            let y1 = bombs[i][1];
            let x2 = bombs[j][0];
            let y2 = bombs[j][1];
            let r = bombs[i][2];

            let c = (r as i64).pow(2);
            let a = ((x1 - x2) as i64).pow(2);
            let b = ((y1 - y2) as i64).pow(2);
            if c >= a + b {
                graph[i].push(j);
            }
        }
    }

    let mut ret = 0;
    for i in 0..n {
        ret = std::cmp::max(ret, f(i, &graph));
    }

    ret
}

fn main() {
    let bombs = vec![vec![2, 1, 3], vec![6, 1, 4]];
    let ret = maximum_detonation(bombs);
    println!("ret={ret}");
}

#[test]
fn test_maximum_detonation() {
    {
        let bombs = vec![vec![2, 1, 3], vec![6, 1, 4]];
        let ret = maximum_detonation(bombs);
        assert_eq!(ret, 2);
    }
    {
        let bombs = vec![vec![1, 1, 5], vec![10, 10, 5]];
        let ret = maximum_detonation(bombs);
        assert_eq!(ret, 1);
    }
    {
        let bombs = vec![
            vec![1, 2, 3],
            vec![2, 3, 1],
            vec![3, 4, 2],
            vec![4, 5, 3],
            vec![5, 6, 4],
        ];
        let ret = maximum_detonation(bombs);
        assert_eq!(ret, 5);
    }
}
