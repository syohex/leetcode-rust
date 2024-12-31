fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
    use std::cmp::min;
    let limit = *days.last().unwrap() as usize;
    let mut dp = vec![0; limit + 1];

    let mut j = 0;
    for i in 1..=limit {
        if (i as i32) < days[j] {
            dp[i] = dp[i - 1];
        } else {
            j += 1;
            let ret1 = dp[i - 1] + costs[0];
            let ret7 = if i >= 7 {
                dp[i - 7] + costs[1]
            } else {
                dp[0] + costs[1]
            };
            let ret30 = if i >= 30 {
                dp[i - 30] + costs[2]
            } else {
                dp[0] + costs[2]
            };
            dp[i] = min(ret1, min(ret7, ret30));
        }
    }

    dp[limit]
}

fn main() {
    let days = vec![1, 4, 6, 7, 8, 20];
    let costs = vec![2, 7, 15];
    let ret = mincost_tickets(days, costs);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let days = vec![1, 4, 6, 7, 8, 20];
        let costs = vec![2, 7, 15];
        let ret = mincost_tickets(days, costs);
        assert_eq!(ret, 11);
    }
    {
        let days = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31];
        let costs = vec![2, 7, 15];
        let ret = mincost_tickets(days, costs);
        assert_eq!(ret, 17);
    }
}
