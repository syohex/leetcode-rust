fn minimum_cost(costs: Vec<i32>) -> i32 {
    let mut costs = costs;
    costs.sort_unstable();

    costs
        .into_iter()
        .rev()
        .enumerate()
        .filter(|(i, _)| i % 3 != 2)
        .map(|(_, c)| c)
        .sum()
}

fn main() {
    let ret = minimum_cost(vec![6, 5, 7, 9, 2, 2]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(minimum_cost(vec![1, 2, 3]), 5);
    assert_eq!(minimum_cost(vec![6, 5, 7, 9, 2, 2]), 23);
    assert_eq!(minimum_cost(vec![5, 5]), 10);
}
