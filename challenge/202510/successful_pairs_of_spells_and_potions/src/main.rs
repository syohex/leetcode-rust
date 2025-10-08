fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
    let mut potions = potions;
    potions.sort_unstable();

    spells
        .into_iter()
        .map(|n| {
            let p = potions.partition_point(|&m| (n as i64 * m as i64) < success);
            (potions.len() - p) as i32
        })
        .collect()
}

fn main() {
    let spells = vec![5, 1, 3];
    let potions = vec![1, 2, 3, 4, 5];
    let ret = successful_pairs(spells, potions, 7);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let spells = vec![5, 1, 3];
        let potions = vec![1, 2, 3, 4, 5];
        let expected = vec![4, 0, 3];
        let ret = successful_pairs(spells, potions, 7);
        assert_eq!(ret, expected);
    }
    {
        let spells = vec![3, 1, 2];
        let potions = vec![8, 5, 8];
        let expected = vec![2, 0, 2];
        let ret = successful_pairs(spells, potions, 16);
        assert_eq!(ret, expected);
    }
}
