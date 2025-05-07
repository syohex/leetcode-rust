fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let rows = move_time.len() as i32;
    let cols = move_time[0].len() as i32;
    let steps = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    let mut min_times = vec![vec![1_100_000_000; cols as usize]; rows as usize];
    min_times[0][0] = 0;

    let mut q = BinaryHeap::new();
    q.push((Reverse(0), 0, 0));

    while !q.is_empty() {
        let (Reverse(v), row, col) = q.pop().unwrap();

        for &(x, y) in &steps {
            let r = row as i32 + x;
            let c = col as i32 + y;
            if r >= 0 && r < rows && c >= 0 && c < cols {
                let (r, c) = (r as usize, c as usize);
                let time = std::cmp::max(v, move_time[r][c]) + 1;
                if time < min_times[r][c] {
                    min_times[r][c] = time;
                    q.push((Reverse(time), r, c));
                }
            }
        }
    }

    min_times[rows as usize - 1][cols as usize - 1]
}

fn main() {
    let move_times = vec![vec![0, 4], vec![4, 4]];
    let ret = min_time_to_reach(move_times);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let move_times = vec![vec![0, 4], vec![4, 4]];
        let ret = min_time_to_reach(move_times);
        assert_eq!(ret, 6);
    }
    {
        let move_times = vec![vec![0, 0, 0], vec![0, 0, 0]];
        let ret = min_time_to_reach(move_times);
        assert_eq!(ret, 3);
    }
}
