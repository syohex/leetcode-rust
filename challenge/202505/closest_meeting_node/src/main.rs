fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
    use std::collections::VecDeque;

    fn get_distance(node: usize, edges: &[i32]) -> Vec<i32> {
        let mut q = VecDeque::new();
        q.push_back(node);

        let mut ret = vec![i32::MAX; edges.len()];
        ret[node] = 0;
        let mut visited = vec![false; edges.len()];

        while !q.is_empty() {
            let n = q.pop_front().unwrap();
            visited[n] = true;

            let next = edges[n];
            if next != -1 && !visited[next as usize] {
                ret[next as usize] = ret[n] + 1;
                q.push_back(next as usize);
            }
        }

        ret
    }

    let (node1, node2) = (node1 as usize, node2 as usize);
    let dist1 = get_distance(node1, &edges);
    let dist2 = get_distance(node2, &edges);

    let mut ret = -1;
    let mut min = i32::MAX;
    for i in 0..edges.len() {
        let max_dist = std::cmp::max(dist1[i], dist2[i]);
        if min > max_dist {
            min = max_dist;
            ret = i as i32;
        }
    }

    ret
}
fn main() {
    let edges = vec![2, 2, 3, -1];
    let ret = closest_meeting_node(edges, 0, 1);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let edges = vec![2, 2, 3, -1];
        let ret = closest_meeting_node(edges, 0, 1);
        assert_eq!(ret, 2);
    }
    {
        let edges = vec![1, 2, -1];
        let ret = closest_meeting_node(edges, 0, 2);
        assert_eq!(ret, 2);
    }
}
