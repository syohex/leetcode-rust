fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;

    #[derive(Copy, Clone, Eq, PartialEq)]
    struct Data {
        value: i32,
        row: usize,
        col: usize,
    }

    impl Ord for Data {
        fn cmp(&self, other: &Self) -> Ordering {
            other.value.cmp(&self.value)
        }
    }

    impl PartialOrd for Data {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut pq: BinaryHeap<Data> = BinaryHeap::new();
    for (i, m) in matrix.iter().enumerate() {
        pq.push(Data {
            value: m[0],
            row: i,
            col: 0,
        });
    }

    let mut ret = 0;
    for _ in 0..k {
        if let Some(d) = pq.pop() {
            ret = d.value;
            if d.col != matrix[0].len() - 1 {
                pq.push(Data {
                    value: matrix[d.row][d.col + 1],
                    row: d.row,
                    col: d.col + 1,
                });
            }
        }
    }

    ret
}

fn main() {
    let matrix = vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]];
    let ret = kth_smallest(matrix, 8);
    println!("ret={ret}");
}

#[test]
fn test_kth_smallest() {
    {
        let matrix = vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]];
        let ret = kth_smallest(matrix, 8);
        assert_eq!(ret, 13);
    }
    {
        let matrix = vec![vec![-5]];
        let ret = kth_smallest(matrix, 1);
        assert_eq!(ret, -5);
    }
}
