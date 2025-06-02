fn candy(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    let mut lefts = vec![1; n];

    for i in 1..n {
        if ratings[i] > ratings[i - 1] {
            lefts[i] = lefts[i - 1] + 1
        }
    }

    let mut rights = vec![1; n];
    for i in (0..(n-1)).rev() {
        if ratings[i] > ratings[i + 1] {
            rights[i] = rights[i + 1] + 1;
        }
    }

    let mut ret = 0;
    for i in 0..n {
        ret += std::cmp::max(lefts[i], rights[i]);
    }

    ret
}

fn main() {
    let ratings = vec![1, 0, 2];
    let ret = candy(ratings);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let ratings = vec![1, 0, 2];
        let ret = candy(ratings);
        assert_eq!(ret, 5);
    }
    {
        let ratings = vec![1, 2, 2];
        let ret = candy(ratings);
        assert_eq!(ret, 4);
    }
}
