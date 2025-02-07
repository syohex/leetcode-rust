fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::HashMap;

    let mut ret = vec![];
    let mut ball_color = HashMap::new();
    let mut colors = HashMap::new();
    for q in queries {
        if let Some(old_color) = ball_color.get(&q[0]) {
            let mut should_remove = false;
            if let Some(num) = colors.get_mut(old_color) {
                if *num == 1 {
                    should_remove = true;
                } else {
                    *num -= 1;
                }
            }
            if should_remove {
                colors.remove(old_color);
            }
        } else {
            ball_color.insert(q[0], q[1]);
        }

        ball_color.insert(q[0], q[1]);
        *colors.entry(q[1]).or_insert(0) += 1;

        ret.push(colors.len() as i32);
    }

    ret
}

fn main() {
    let queries = vec![vec![0, 1], vec![1, 2], vec![2, 2], vec![3, 4], vec![4, 5]];
    let ret = query_results(4, queries);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let queries = vec![vec![0, 1], vec![1, 2], vec![2, 2], vec![3, 4], vec![4, 5]];
        let expected = vec![1, 2, 2, 3, 4];
        let ret = query_results(4, queries);
        assert_eq!(ret, expected);
    }
    {
        let queries = vec![vec![1, 4], vec![2, 5], vec![1, 3], vec![3, 4]];
        let expected = vec![1, 2, 2, 3];
        let ret = query_results(4, queries);
        assert_eq!(ret, expected);
    }
}
