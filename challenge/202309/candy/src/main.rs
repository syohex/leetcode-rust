fn candy(ratings: Vec<i32>) -> i32 {
    if ratings.len() == 1 {
        return 1;
    }

    let len = ratings.len();
    let mut lefts = vec![1; len];
    let mut rights = vec![1; len];

    for i in 1..len {
        if ratings[i - 1] < ratings[i] {
            lefts[i] = lefts[i - 1] + 1;
        }
    }

    for i in (0..(len - 1)).rev() {
        if ratings[i] > ratings[i + 1] {
            rights[i] = rights[i + 1] + 1;
        }
    }

    let mut ret = 0;
    for i in 0..len {
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
}
