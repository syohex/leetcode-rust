fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
    use std::collections::VecDeque;

    let len = deck.len();
    let mut deck = deck;
    deck.sort_unstable();

    let mut q = VecDeque::new();
    for i in 0..len {
        q.push_back(i);
    }

    let mut ret = vec![0;len];
    for i in 0..len {
        if let Some(pos) = q.pop_front() {
            ret[pos] = deck[i];
        }

        if let Some(pos) = q.pop_front() {
            q.push_back(pos);
        }
    }

    ret
}

fn main() {
    let deck = vec![17, 13, 11, 2, 3, 5, 7];
    let ret = deck_revealed_increasing(deck);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let deck = vec![17, 13, 11, 2, 3, 5, 7];
        let expected = vec![2, 13, 3, 11, 5, 17, 7];
        let ret = deck_revealed_increasing(deck);
        assert_eq!(ret, expected);
    }
    {
        let deck = vec![1, 1000];
        let expected = vec![1, 1000];
        let ret = deck_revealed_increasing(deck);
        assert_eq!(ret, expected);
    }
}
