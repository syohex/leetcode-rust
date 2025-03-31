fn min_costs(cost: Vec<i32>) -> Vec<i32> {
    let mut ret = vec![];
    let mut min_cost = cost[0];
    for c in cost {
        min_cost = std::cmp::min(min_cost, c);
        ret.push(min_cost);
    }
    ret
}

fn main() {
    let cost = vec![5, 3, 4, 1, 3, 2];
    let ret = min_costs(cost);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let cost = vec![5, 3, 4, 1, 3, 2];
        let expected = vec![5, 3, 3, 1, 1, 1];
        let ret = min_costs(cost);
        assert_eq!(ret, expected);
    }
    {
        let cost = vec![1, 2, 4, 6, 7];
        let expected = vec![1, 1, 1, 1, 1];
        let ret = min_costs(cost);
        assert_eq!(ret, expected);
    }
}
