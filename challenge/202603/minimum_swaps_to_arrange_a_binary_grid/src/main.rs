fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
    let len = grid.len();
    let mut trailing_zeros = vec![];
    for v in &grid {
        let mut zeros = 0;
        for n in v.iter().rev() {
            if *n != 0 {
                break;
            }
            zeros += 1;
        }
        trailing_zeros.push(zeros);
    }

    let mut ret = 0;
    for i in 0..len {
        let zero_needed = len - 1 - i;
        if trailing_zeros[i] >= zero_needed {
            continue;
        }

        let mut j = i;
        while j < len {
            if trailing_zeros[j] >= zero_needed {
                break;
            }
            j += 1;
        }

        if j >= len {
            return -1;
        }

        while i < j {
            trailing_zeros.swap(j, j - 1);
            ret += 1;
            j -= 1;
        }
    }

    ret
}

fn main() {
    let grid = vec![vec![0, 0, 1], vec![1, 1, 0], vec![1, 0, 0]];
    let ret = min_swaps(grid);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![vec![0, 0, 1], vec![1, 1, 0], vec![1, 0, 0]];
        let ret = min_swaps(grid);
        assert_eq!(ret, 3);
    }
    {
        let grid = vec![
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
        ];
        let ret = min_swaps(grid);
        assert_eq!(ret, -1);
    }
    {
        let grid = vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 1]];
        let ret = min_swaps(grid);
        assert_eq!(ret, 0);
    }
}
