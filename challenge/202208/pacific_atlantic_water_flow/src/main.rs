fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::collections::HashSet;
    // use std::iter::FromIterator; for older Rust such as leetcode

    fn can_reachable(
        start_points: &Vec<(i32, i32)>,
        heights: &Vec<Vec<i32>>,
        visited: &mut HashSet<(i32, i32)>,
    ) -> HashSet<(i32, i32)> {
        let mut ret = HashSet::from_iter(start_points.clone());
        let mut v = start_points.clone();
        let steps = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];

        let rows = heights.len() as i32;
        let cols = heights[0].len() as i32;

        while let Some((x, y)) = v.pop() {
            let current = heights[x as usize][y as usize];

            visited.insert((x, y));

            for (i, j) in &steps {
                let new_x = x + *i;
                let new_y = y + *j;
                if !(new_x >= 0 && new_x < rows && new_y >= 0 && new_y < cols) {
                    continue;
                }

                if visited.contains(&(new_x, new_y)) {
                    continue;
                }

                let next = heights[new_x as usize][new_y as usize];
                if current <= next {
                    v.push((new_x, new_y));
                    ret.insert((new_x, new_y));
                }
            }
        }

        ret
    }

    let rows = heights.len() as i32;
    let cols = heights[0].len() as i32;
    let mut pacific_points = vec![];
    let mut atlantic_points = vec![];
    for i in 0..cols {
        pacific_points.push((0, i as i32));
        atlantic_points.push(((rows - 1) as i32, i as i32));
    }

    for i in 0..rows {
        pacific_points.push((i as i32, 0));
        atlantic_points.push((i as i32, (cols - 1) as i32));
    }

    let mut visited = HashSet::new();
    let s1 = can_reachable(&pacific_points, &heights, &mut visited);

    visited.clear();
    let s2 = can_reachable(&atlantic_points, &heights, &mut visited);

    let mut ret = vec![];
    for i in 0..rows {
        for j in 0..cols {
            let key = (i, j);
            if s1.contains(&key) && s2.contains(&key) {
                ret.push(vec![i, j]);
            }
        }
    }

    ret
}

fn main() {
    let heights = vec![
        vec![1, 2, 2, 3, 5],
        vec![3, 2, 3, 4, 4],
        vec![2, 4, 5, 3, 1],
        vec![6, 7, 1, 4, 5],
        vec![5, 1, 1, 2, 4],
    ];
    let ret = pacific_atlantic(heights);
    println!("ret={:?}", ret);
}

#[test]
fn test_pacific_atlantic() {
    {
        let heights = vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ];
        let expected = vec![
            vec![0, 4],
            vec![1, 3],
            vec![1, 4],
            vec![2, 2],
            vec![3, 0],
            vec![3, 1],
            vec![4, 0],
        ];
        let ret = pacific_atlantic(heights);
        assert_eq!(ret, expected);
    }
    {
        let heights = vec![vec![1]];
        let expected = vec![vec![0, 0]];
        let ret = pacific_atlantic(heights);
        assert_eq!(ret, expected);
    }
}
