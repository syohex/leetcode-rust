fn recover_order(order: Vec<i32>, friends: Vec<i32>) -> Vec<i32> {
    use std::collections::HashSet;

    let s: HashSet<_> = friends.into_iter().collect();
    order.into_iter().filter(|n| s.contains(n)).collect()
}

fn main() {
    let order = vec![3, 1, 2, 5, 4];
    let friends = vec![1, 3, 4];
    let ret = recover_order(order, friends);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let order = vec![3, 1, 2, 5, 4];
        let friends = vec![1, 3, 4];
        let ret = recover_order(order, friends);
        assert_eq!(ret, vec![3, 1, 4]);
    }
    {
        let order = vec![1, 4, 5, 3, 2];
        let friends = vec![2, 5];
        let ret = recover_order(order, friends);
        assert_eq!(ret, vec![5, 2]);
    }
}
