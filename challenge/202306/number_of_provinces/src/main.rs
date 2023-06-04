fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    let n = is_connected.len();
    let mut visited = vec![false; n];

    let mut ret = 0;
    for i in 0..n {
        if visited[i] {
            continue;
        }

        visited[i] = true;

        let mut q = vec![i];
        while !q.is_empty() {
            let node = q.pop().unwrap();

            dbg!(&is_connected[node]);
            for (j, v) in is_connected[node].iter().enumerate() {
                if *v == 1 && !visited[j] {
                    visited[j] = true;
                    q.push(j);
                }
            }
        }

        ret += 1;
    }

    ret
}

fn main() {
    let is_connected = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
    let ret = find_circle_num(is_connected);
    println!("ret={ret}");
}

#[test]
fn test_find_circle_num() {
    {
        let is_connected = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
        let ret = find_circle_num(is_connected);
        assert_eq!(ret, 2);
    }
    {
        let is_connected = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        let ret = find_circle_num(is_connected);
        assert_eq!(ret, 3);
    }
}
