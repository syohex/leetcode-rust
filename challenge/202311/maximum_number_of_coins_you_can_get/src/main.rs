fn max_coins(piles: Vec<i32>) -> i32 {
    let mut piles = piles;
    piles.sort_unstable();

    let mut left = 0i32;
    let mut right = piles.len() as i32 - 2;

    let mut ret = 0;
    while left < right {
        ret += piles[right as usize];

        left += 1;
        right -= 2;
    }

    ret
}

fn main() {
    let piles = vec![2, 4, 1, 2, 7, 8];
    let ret = max_coins(piles);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let piles = vec![2, 4, 1, 2, 7, 8];
        let ret = max_coins(piles);
        assert_eq!(ret, 9);
    }
    {
        let piles = vec![2, 4, 5];
        let ret = max_coins(piles);
        assert_eq!(ret, 4);
    }
    {
        let piles = vec![9, 8, 7, 6, 5, 1, 2, 3, 4];
        let ret = max_coins(piles);
        assert_eq!(ret, 18);
    }
}
