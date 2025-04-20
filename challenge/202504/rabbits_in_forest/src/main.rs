fn num_rabitts(answers: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let h = answers.into_iter().fold(HashMap::new(), |mut acc, n| {
        *acc.entry(n).or_insert(0) += 1;
        acc
    });

    h.into_iter().fold(0, |acc, (k, v)| {
        if v % (k + 1) == 0 {
            acc + v
        } else {
            acc + v + (k + 1) - v % (k + 1)
        }
    })
}

fn main() {
    let answers = vec![10, 10, 10];
    let ret = num_rabitts(answers);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let answers = vec![1, 1, 2];
        let ret = num_rabitts(answers);
        assert_eq!(ret, 5);
    }
    {
        let answers = vec![10, 10, 10];
        let ret = num_rabitts(answers);
        assert_eq!(ret, 11);
    }
}
