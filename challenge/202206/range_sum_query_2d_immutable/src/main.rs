struct NumMatrix {
    acc: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut acc = vec![vec![0; cols]; rows + 1];
        for i in 0..rows {
            for j in 0..cols {
                acc[i + 1][j] = acc[i][j] + matrix[i][j];
            }
        }

        Self { acc }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let mut sum = 0;
        for i in col1..=col2 {
            let i = i as usize;
            sum += self.acc[row2 as usize + 1][i] - self.acc[row1 as usize][i];
        }

        sum
    }
}

fn main() {
    let matrix = vec![
        vec![3, 0, 1, 4, 2],
        vec![5, 6, 3, 2, 1],
        vec![1, 2, 0, 1, 5],
        vec![4, 1, 0, 1, 7],
        vec![1, 0, 3, 0, 5],
    ];
    let nm = NumMatrix::new(matrix);
    let ret = nm.sum_region(1, 2, 2, 4);
    println!("ret={ret}");
}

#[test]
fn test_range_sum_query2d() {
    {
        let matrix = vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ];
        let nm = NumMatrix::new(matrix);
        assert_eq!(nm.sum_region(2, 1, 4, 3), 8);
        assert_eq!(nm.sum_region(1, 1, 2, 2), 11);
        assert_eq!(nm.sum_region(1, 2, 2, 4), 12);
    }
}
