fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
    fn to_graph(edges: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ret = vec![vec![]; edges.len() + 1];
        for edge in edges {
            ret[edge[0] as usize].push(edge[1]);
            ret[edge[1] as usize].push(edge[0]);
        }

        ret
    }

    fn get_diameter(graph: &Vec<Vec<i32>>, init_node: i32, nodes: usize) -> (i32, i32) {
        use std::collections::VecDeque;

        let mut visited = vec![false; nodes];
        let mut q = VecDeque::new();
        q.push_back(init_node);
        visited[init_node as usize] = true;

        let mut diameter = 0;
        let mut furtherest_node = init_node;
        while !q.is_empty() {
            let len = q.len();
            for _ in 0..len {
                if let Some(node) = q.pop_front() {
                    for &next in &graph[node as usize] {
                        if !visited[next as usize] {
                            q.push_back(next);
                            visited[next as usize] = true;
                            furtherest_node = next;
                        }
                    }
                }
            }

            if !q.is_empty() {
                diameter += 1;
            }
        }

        (diameter, furtherest_node)
    }

    fn longest_diameter(graph: &Vec<Vec<i32>>, nodes: usize) -> i32 {
        let (_, furtherest_node) = get_diameter(graph, 0, nodes);
        let (diameter, _) = get_diameter(graph, furtherest_node, nodes);
        diameter
    }

    fn half_diameter(diameter: i32) -> i32 {
        (diameter as f64 / 2.0).ceil() as i32
    }

    let nodes1 = edges1.len() + 1;
    let nodes2 = edges2.len() + 1;
    let graph1 = to_graph(&edges1);
    let graph2 = to_graph(&edges2);

    let diameter1 = longest_diameter(&graph1, nodes1);
    let diameter2 = longest_diameter(&graph2, nodes2);
    let longest = std::cmp::max(diameter1, diameter2);

    let merged_diameter = half_diameter(diameter1) + half_diameter(diameter2) + 1;
    std::cmp::max(longest, merged_diameter)
}

fn main() {
    let edges1 = vec![vec![0, 1], vec![0, 2], vec![0, 3]];
    let edges2 = vec![vec![0, 1]];
    let ret = minimum_diameter_after_merge(edges1, edges2);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let edges1 = vec![vec![0, 1], vec![0, 2], vec![0, 3]];
        let edges2 = vec![vec![0, 1]];
        let ret = minimum_diameter_after_merge(edges1, edges2);
        assert_eq!(ret, 3);
    }
    {
        let edges1 = vec![
            vec![0, 1],
            vec![0, 2],
            vec![0, 3],
            vec![2, 4],
            vec![2, 5],
            vec![3, 6],
            vec![2, 7],
        ];
        let edges2 = vec![
            vec![0, 1],
            vec![0, 2],
            vec![0, 3],
            vec![2, 4],
            vec![2, 5],
            vec![3, 6],
            vec![2, 7],
        ];
        let ret = minimum_diameter_after_merge(edges1, edges2);
        assert_eq!(ret, 5);
    }
}
