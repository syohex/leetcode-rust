fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start: i32, end: i32) -> f64 {
    use std::collections::{HashMap, VecDeque};

    let (start, end) = (start as usize, end as usize);
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut score: HashMap<(usize, usize), f64> = HashMap::new();
    for i in 0..edges.len() {
        let (a, b) = (edges[i][0] as usize, edges[i][1] as usize);

        graph.entry(a).or_insert(vec![]).push(b);
        graph.entry(b).or_insert(vec![]).push(a);

        *score.entry((a, b)).or_insert(0.0) = succ_prob[i];
        *score.entry((b, a)).or_insert(0.0) = succ_prob[i];
    }

    let mut q = VecDeque::new();
    q.push_back((start, 1.0));

    let mut maxs = vec![0.0f64; n as usize];
    while let Some((p, point)) = q.pop_front() {
        if let Some(v) = graph.get(&p) {
            for next in v {
                if let Some(score) = score.get(&(p, *next)) {
                    let new_score = point * score;
                    if new_score > maxs[*next] {
                        maxs[*next] = new_score;
                        q.push_back((*next, new_score));
                    }
                }
            }
        }
    }

    maxs[end]
}

fn main() {
    let n = 3;
    let edges = vec![vec![0, 1], vec![1, 2], vec![0, 2]];
    let succ_prob = vec![0.5, 0.5, 0.2];
    let (start, end) = (0, 2);
    let ret = max_probability(n, edges, succ_prob, start, end);
    println!("ret={ret}");
}

#[test]
fn test_max_probability() {
    {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2], vec![0, 2]];
        let succ_prob = vec![0.5, 0.5, 0.2];
        let (start, end) = (0, 2);
        let ret = max_probability(n, edges, succ_prob, start, end);
        assert_eq!(ret, 0.25);
    }
    {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2], vec![0, 2]];
        let succ_prob = vec![0.5, 0.5, 0.3];
        let (start, end) = (0, 2);
        let ret = max_probability(n, edges, succ_prob, start, end);
        assert_eq!(ret, 0.3);
    }
    {
        let n = 3;
        let edges = vec![vec![0, 1]];
        let succ_prob = vec![0.5];
        let (start, end) = (0, 2);
        let ret = max_probability(n, edges, succ_prob, start, end);
        assert_eq!(ret, 0.0);
    }
}
