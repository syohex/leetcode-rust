fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
    let mut baskets = baskets;
    let mut ret = 0;
    for f in fruits {
        if let Some(p) = baskets.iter().position(|&n| n != -1 && f <= n) {
            baskets[p] = -1;
        } else {
            ret += 1;
        }
    }

    ret
}

fn main() {
    let fruits = vec![4, 2, 5];
    let baskets = vec![3, 5, 4];
    let ret = num_of_unplaced_fruits(fruits, baskets);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let fruits = vec![4, 2, 5];
        let baskets = vec![3, 5, 4];
        let ret = num_of_unplaced_fruits(fruits, baskets);
        assert_eq!(ret, 1);
    }
    {
        let fruits = vec![3, 6, 1];
        let baskets = vec![6, 4, 7];
        let ret = num_of_unplaced_fruits(fruits, baskets);
        assert_eq!(ret, 0);
    }
}
