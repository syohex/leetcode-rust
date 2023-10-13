fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut p2 = cost[0];
    let mut p1 = cost[1];

    for i in 2..cost.len() {
        let v2 = cost[i] + p2;
        let v1 = cost[i] + p1;

        p2 = p1;
        p1 = std::cmp::min(v1, v2);
    }

    std::cmp::min(p1, p2)
}

fn main() {
    let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
    let ret = min_cost_climbing_stairs(cost);
    println!("ret={ret}");
}

#[test]
fn test_min_cost_climing_stairs() {
    {
        let cost = vec![10, 15, 20];
        let ret = min_cost_climbing_stairs(cost);
        assert_eq!(ret, 15);
    }
    {
        let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        let ret = min_cost_climbing_stairs(cost);
        assert_eq!(ret, 6);
    }
}
