fn smallest_common_element(mat: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;

    let mut start = std::i32::MIN;
    let mut end = std::i32::MAX;

    for v in mat.iter() {
        start = std::cmp::max(start, v[0]);
        end = std::cmp::min(end, v[v.len() - 1]);
    }

    let mut h: HashMap<i32, i32> = HashMap::new();
    for v in mat.iter() {
        for n in v {
            if *n < start {
                continue;
            }
            if *n > end {
                break;
            }

            *h.entry(*n).or_insert(0) += 1;
        }
    }

    for (k, v) in h {
        if v as usize == mat.len() {
            return k;
        }
    }

    -1
}

fn main() {
    let mat = vec![
        vec![1, 2, 3, 4, 5],
        vec![2, 4, 5, 8, 10],
        vec![3, 5, 7, 9, 11],
        vec![1, 3, 5, 7, 9],
    ];
    let ret = smallest_common_element(mat);
    println!("ret={}", ret);
}

#[test]
fn test_smallest_common_element() {
    {
        let mat = vec![
            vec![1, 2, 3, 4, 5],
            vec![2, 4, 5, 8, 10],
            vec![3, 5, 7, 9, 11],
            vec![1, 3, 5, 7, 9],
        ];
        assert_eq!(smallest_common_element(mat), 5);
    }
    {
        let mat = vec![vec![1, 2, 3], vec![2, 3, 4], vec![2, 3, 5]];
        assert_eq!(smallest_common_element(mat), 2);
    }
}
