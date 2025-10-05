fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::collections::VecDeque;

    fn f(
        q: &mut VecDeque<(usize, usize)>,
        rows: usize,
        cols: usize,
        heights: &[Vec<i32>],
    ) -> Vec<Vec<bool>> {
        let mut ret = vec![vec![false; cols]; rows];
        let steps = [(-1, 0), (0, -1), (1, 0), (0, 1)];

        while let Some((r, c)) = q.pop_front() {
            ret[r][c] = true;
            for &(x, y) in &steps {
                let row = r as i32 + x;
                let col = c as i32 + y;

                if row >= 0
                    && row < rows as i32
                    && col >= 0
                    && col < cols as i32
                    && !ret[row as usize][col as usize]
                    && heights[row as usize][col as usize] >= heights[r][c]
                {
                    q.push_back((row as usize, col as usize));
                }
            }
        }

        ret
    }

    let rows = heights.len();
    let cols = heights[0].len();

    let mut pacifics = VecDeque::new();
    let mut atlantics = VecDeque::new();

    for i in 0..rows {
        pacifics.push_back((i, 0));
        atlantics.push_back((i, cols - 1));
    }
    for i in 0..cols {
        pacifics.push_back((0, i));
        atlantics.push_back((rows - 1, i));
    }

    let ret1 = f(&mut pacifics, rows, cols, &heights);
    let ret2 = f(&mut atlantics, rows, cols, &heights);

    let mut ret = vec![];
    for i in 0..rows {
        for j in 0..cols {
            if ret1[i][j] && ret2[i][j] {
                ret.push(vec![i as i32, j as i32]);
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
    println!("ret={ret:?}");
}

#[test]
fn test() {
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
        let mut ret = pacific_atlantic(heights);
        ret.sort_unstable();
        assert_eq!(ret, expected);
    }
    {
        let heights = vec![vec![1]];
        let expected = vec![vec![0, 0]];
        let ret = pacific_atlantic(heights);
        assert_eq!(ret, expected);
    }
}
