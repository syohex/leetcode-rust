fn minimum_cost(cost: Vec<i32>) -> i32 {
    let mut cost = cost.clone();
    cost.sort();

    let mut ret = 0;
    while !cost.is_empty() {
        if let Some(first) = cost.pop() {
            ret += first;
            if let Some(second) = cost.pop() {
                ret += second;
                let _ = cost.pop(); // free
            }
        }
    }

    ret
}

fn main() {
    let cost = vec![6, 5, 7, 9, 2, 2];
    let ret = minimum_cost(cost);
    println!("ret={ret}");
}

#[test]
fn test_minimum_cost() {
    {
        let cost = vec![1, 2, 3];
        assert_eq!(minimum_cost(cost), 5);
    }
    {
        let cost = vec![6, 5, 7, 9, 2, 2];
        assert_eq!(minimum_cost(cost), 23);
    }
    {
        let cost = vec![5, 5];
        assert_eq!(minimum_cost(cost), 10);
    }
}
