fn stone_game(piles: Vec<i32>) -> bool {
    fn f(piles: &[i32], left: usize, right: usize, cache: &mut Vec<Vec<i32>>) -> i32 {
        if cache[left][right] != i32::MIN {
            return cache[left][right];
        }
        if left >= right {
            return piles[left];
        }

        let v1 = piles[left] - f(piles, left + 1, right, cache);
        let v2 = piles[right] - f(piles, left, right - 1, cache);
        let v = std::cmp::max(v1, v2);

        cache[left][right] = v;
        v
    }

    let len = piles.len();
    let mut cache = vec![vec![i32::MIN; len]; len];
    f(&piles, 0, len - 1, &mut cache) > 0
}

fn main() {
    let ret = stone_game(vec![5, 3, 4, 5]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(stone_game(vec![3, 2, 10, 4]));
    assert!(stone_game(vec![5, 3, 4, 5]));
    assert!(stone_game(vec![3, 7, 2, 3]));
}
