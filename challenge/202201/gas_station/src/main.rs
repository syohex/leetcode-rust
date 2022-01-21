fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut total = 0;
    let mut current = 0;

    let mut ret = 0;
    for i in 0..gas.len() {
        total += gas[i] - cost[i];
        current += gas[i] - cost[i];

        if current < 0 {
            ret = i + 1;
            current = 0;
        }
    }

    if total < 0 {
        -1
    } else {
        ret as i32
    }
}

fn main() {
    let gas = vec![1, 2, 3, 4, 5];
    let cost = vec![3, 4, 5, 1, 2];
    let ret = can_complete_circuit(gas, cost);
    println!("ret={ret}");
}

#[test]
fn test_can_complete_circuit() {
    {
        let gas = vec![1, 2, 3, 4, 5];
        let cost = vec![3, 4, 5, 1, 2];
        assert_eq!(can_complete_circuit(gas, cost), 3);
    }
    {
        let gas = vec![2, 3, 4];
        let cost = vec![3, 4, 3];
        assert_eq!(can_complete_circuit(gas, cost), -1);
    }
}
