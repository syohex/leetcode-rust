fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashSet;

    let n = n as usize;
    let mut graph = vec![HashSet::<usize>::new(); n];
    for road in roads {
        graph[road[0] as usize].insert(road[1] as usize);
        graph[road[1] as usize].insert(road[0] as usize);
    }

    let mut ret = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            let mut sum = (graph[i].len() + graph[j].len()) as i32;
            if graph[i].contains(&j) {
                sum -= 1;
            }

            ret = std::cmp::max(ret, sum);
        }
    }

    ret
}

fn main() {
    let n = 5;
    let roads = vec![
        vec![0, 1],
        vec![0, 3],
        vec![1, 2],
        vec![1, 3],
        vec![2, 3],
        vec![2, 4],
    ];
    let ret = maximal_network_rank(n, roads);
    println!("ret={ret}");
}

#[test]
fn test_maximal_network_rank() {
    {
        let n = 4;
        let roads = vec![vec![0, 1], vec![0, 3], vec![1, 2], vec![1, 3]];
        let ret = maximal_network_rank(n, roads);
        assert_eq!(ret, 4);
    }
    {
        let n = 5;
        let roads = vec![
            vec![0, 1],
            vec![0, 3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
            vec![2, 4],
        ];
        let ret = maximal_network_rank(n, roads);
        assert_eq!(ret, 5);
    }
    {
        let n = 8;
        let roads = vec![
            vec![0, 1],
            vec![1, 2],
            vec![2, 3],
            vec![2, 4],
            vec![5, 6],
            vec![5, 7],
        ];
        let ret = maximal_network_rank(n, roads);
        assert_eq!(ret, 5);
    }
}
