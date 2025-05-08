fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
    use std::cmp::{Reverse, max};
    use std::collections::BinaryHeap;

    let rows = move_time.len() as i32;
    let cols = move_time[0].len() as i32;
    let steps = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    let mut min_times = vec![vec![1_100_000_000; cols as usize]; rows as usize];
    min_times[0][0] = 0;

    let mut q = BinaryHeap::new();
    q.push((Reverse(0), 0, 0, 1));

    while !q.is_empty() {
        let (Reverse(current_time), row, col, wait_time) = q.pop().unwrap();

        for &(x, y) in &steps {
            let r = row + x;
            let c = col + y;
            if r >= 0 && r < rows && c >= 0 && c < cols {
                let (r, c) = (r as usize, c as usize);
                let time = max(current_time, move_time[r][c]) + wait_time;
                if time < min_times[r][c] {
                    min_times[r][c] = time;
                    let next_wait = if wait_time == 1 { 2 } else { 1 };
                    q.push((Reverse(time), r as i32, c as i32, next_wait));
                }
            }
        }
    }

    min_times[rows as usize - 1][cols as usize - 1]
}

fn main() {
    let move_time = vec![vec![0, 4], vec![4, 4]];
    let ret = min_time_to_reach(move_time);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let move_time = vec![vec![0, 4], vec![4, 4]];
        let ret = min_time_to_reach(move_time);
        assert_eq!(ret, 7);
    }
    {
        let move_time = vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0]];
        let ret = min_time_to_reach(move_time);
        assert_eq!(ret, 6);
    }
}
