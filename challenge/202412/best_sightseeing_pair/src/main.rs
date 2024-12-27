fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
    let init = values[0];
    values
        .into_iter()
        .enumerate()
        .skip(1)
        .fold((0, init), |(acc, max), (i, n)| {
            let sum = max + n - i as i32;
            (std::cmp::max(acc, sum), std::cmp::max(max, n + i as i32))
        })
        .0
}

fn main() {
    let values = vec![8, 1, 5, 2, 6];
    let ret = max_score_sightseeing_pair(values);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let values = vec![8, 1, 5, 2, 6];
        let ret = max_score_sightseeing_pair(values);
        assert_eq!(ret, 11);
    }
    {
        let values = vec![1, 2];
        let ret = max_score_sightseeing_pair(values);
        assert_eq!(ret, 2);
    }
}
