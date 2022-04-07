fn last_stone_weight(stones: Vec<i32>) -> i32 {
    use std::collections::BinaryHeap;

    let mut h = BinaryHeap::new();
    for stone in stones {
        h.push(stone);
    }

    while h.len() > 1 {
        let first = h.pop().unwrap();
        let second = h.pop().unwrap();

        if first != second {
            h.push(first - second);
        }
    }

    if h.len() == 1 {
        h.pop().unwrap()
    } else {
        0
    }
}

fn main() {
    let stones = vec![2, 7, 4, 1, 8, 1];
    let ret = last_stone_weight(stones);
    println!("ret={ret}");
}

#[test]
fn test_last_stone_weight() {
    {
        let stones = vec![2, 7, 4, 1, 8, 1];
        assert_eq!(last_stone_weight(stones), 1);
    }
    {
        let stones = vec![1];
        assert_eq!(last_stone_weight(stones), 1);
    }
    {
        let stones = vec![2, 2];
        assert_eq!(last_stone_weight(stones), 0);
    }
}
