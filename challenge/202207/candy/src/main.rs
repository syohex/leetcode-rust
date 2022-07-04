fn candy(ratings: Vec<i32>) -> i32 {
    let len = ratings.len();
    if len == 1 {
        return 1;
    }

    let mut lefts = vec![1; len];
    for i in 1..len {
        if ratings[i] > ratings[i - 1] {
            lefts[i] = lefts[i - 1] + 1;
        }
    }

    let mut rights = vec![1; len];
    for i in (0..=len - 2).rev() {
        if ratings[i] > ratings[i + 1] {
            rights[i] = rights[i + 1] + 1;
        }
    }

    let mut sum = 0;
    for i in 0..len {
        sum += std::cmp::max(lefts[i], rights[i]);
    }

    sum
}

fn main() {
    let ratings = vec![1, 0, 2];
    let ret = candy(ratings);
    println!("ret={ret}");
}

#[test]
fn test_candy() {
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
    {
        let ratings = vec![0];
        let ret = candy(ratings);
        assert_eq!(ret, 1);
    }
    {
        let ratings = vec![0, 0];
        let ret = candy(ratings);
        assert_eq!(ret, 2);
    }
    {
        let ratings = vec![0, 1];
        let ret = candy(ratings);
        assert_eq!(ret, 3);
    }
}
