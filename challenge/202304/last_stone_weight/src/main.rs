fn last_stone_weight(stones: Vec<i32>) -> i32 {
    use std::collections::BinaryHeap;

    let mut q = BinaryHeap::new();
    for stone in stones {
        q.push(stone);
    }

    while q.len() >= 2 {
        let s1 = q.pop().unwrap();
        let s2 = q.pop().unwrap();

        if s1 != s2 {
            q.push(s1 - s2);
        }
    }

    if q.is_empty() {
        0
    } else {
        q.pop().unwrap()
    }
}

fn main() {
    let stones = vec![2, 4, 3, 1, 8, 1];
    let ret = last_stone_weight(stones);
    println!("ret={ret}");
}

#[test]
fn test_last_stone_weights() {
    {
        let stones = vec![2, 4, 3, 1, 8, 1];
        let ret = last_stone_weight(stones);
        assert_eq!(ret, 1);
    }
    {
        let stones = vec![1];
        let ret = last_stone_weight(stones);
        assert_eq!(ret, 1);
    }
}
