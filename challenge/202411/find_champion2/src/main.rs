fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    fn f(node: usize, graph: &Vec<Vec<usize>>, cache: &mut Vec<i32>) -> i32 {
        if cache[node] != -1 {
            return cache[node];
        }

        let mut ret = 1;
        for next in graph[node].iter() {
            if cache[*next] == -1 {
                ret += f(*next, graph, cache);
            }
        }

        cache[node] = ret;
        ret
    }

    let n = n as usize;
    let mut graph = vec![vec![]; n];
    for v in edges {
        graph[v[0] as usize].push(v[1] as usize);
    }

    for i in 0..n {
        let mut cache = vec![-1; n];
        let ret = f(i, &graph, &mut cache);
        if ret == n as i32 {
            return i as i32;
        }
    }

    -1
}

fn main() {
    let n = 3;
    let edges = vec![vec![0, 1], vec![1, 2]];
    let ret = find_champion(n, edges);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2]];
        let ret = find_champion(n, edges);
        assert_eq!(ret, 0);
    }
    {
        let n = 4;
        let edges = vec![vec![0, 2], vec![1, 3], vec![1, 2]];
        let ret = find_champion(n, edges);
        assert_eq!(ret, -1);
    }
    {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2], vec![0, 2]];
        let ret = find_champion(n, edges);
        assert_eq!(ret, 0);
    }
    {
        let n = 4;
        let edges = vec![vec![0, 1], vec![2, 0], vec![2, 1]];
        let ret = find_champion(n, edges);
        assert_eq!(ret, -1);
    }
    {
        let n = 2;
        let edges = vec![vec![1, 0]];
        let ret = find_champion(n, edges);
        assert_eq!(ret, 1);
    }
}
