fn min_score_triangulation(values: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    fn f(
        left: usize,
        right: usize,
        values: &[i32],
        cache: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if left + 2 > right {
            return 0;
        }

        if left + 2 == right {
            return values[left] * values[left + 1] * values[right];
        }

        if let Some(v) = cache.get(&(left, right)) {
            *v
        } else {
            let mut min = i32::MAX;
            for i in (left + 1)..right {
                let p = values[left] * values[i] * values[right];
                let v = p + f(left, i, values, cache) + f(i, right, values, cache);
                min = std::cmp::min(min, v);
            }

            cache.insert((left, right), min);
            min
        }
    }

    let mut cache = HashMap::new();
    f(0, values.len() - 1, &values, &mut cache)
}

fn main() {
    let ret = min_score_triangulation(vec![3, 7, 4, 5]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_score_triangulation(vec![1, 2, 3]), 6);
    assert_eq!(min_score_triangulation(vec![3, 7, 4, 5]), 144);
    assert_eq!(min_score_triangulation(vec![1, 3, 1, 4, 1, 5]), 13);
}
