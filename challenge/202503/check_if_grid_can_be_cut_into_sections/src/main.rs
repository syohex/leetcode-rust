fn check_valid_cuts(_n: i32, rectangles: Vec<Vec<i32>>) -> bool {
    fn f(rectangles: &mut [Vec<i32>], index: usize) -> bool {
        rectangles.sort_unstable_by(|a, b| a[index].cmp(&b[index]));

        let mut max_end = rectangles[0][index + 2];
        let mut cuts = 0;
        for rect in rectangles.iter().skip(1) {
            if rect[index] >= max_end {
                cuts += 1;
            }

            max_end = std::cmp::max(max_end, rect[index + 2]);
        }

        cuts >= 2
    }

    let mut rectangles = rectangles;
    f(&mut rectangles, 0) || f(&mut rectangles, 1)
}
fn main() {
    let n = 5;
    let rectangles = vec![
        vec![1, 0, 5, 2],
        vec![0, 2, 2, 4],
        vec![3, 2, 5, 3],
        vec![0, 4, 4, 5],
    ];
    let ret = check_valid_cuts(n, rectangles);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let n = 5;
        let rectangles = vec![
            vec![1, 0, 5, 2],
            vec![0, 2, 2, 4],
            vec![3, 2, 5, 3],
            vec![0, 4, 4, 5],
        ];
        let ret = check_valid_cuts(n, rectangles);
        assert!(ret);
    }
    {
        let n = 4;
        let rectangles = vec![
            vec![0, 0, 1, 1],
            vec![2, 0, 3, 4],
            vec![0, 2, 2, 3],
            vec![3, 0, 4, 3],
        ];
        let ret = check_valid_cuts(n, rectangles);
        assert!(ret);
    }
    {
        let n = 4;
        let rectangles = vec![
            vec![0, 2, 2, 4],
            vec![1, 0, 3, 2],
            vec![2, 2, 3, 4],
            vec![3, 0, 4, 2],
            vec![3, 2, 4, 4],
        ];
        let ret = check_valid_cuts(n, rectangles);
        assert!(!ret);
    }
}
