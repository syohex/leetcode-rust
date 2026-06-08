fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let mut smallers = vec![];
    let mut pivots = vec![];
    let mut largers = vec![];

    for n in nums {
        match n.cmp(&pivot) {
            std::cmp::Ordering::Less => smallers.push(n),
            std::cmp::Ordering::Equal => pivots.push(n),
            std::cmp::Ordering::Greater => largers.push(n),
        }
    }

    smallers.append(&mut pivots);
    smallers.append(&mut largers);
    smallers
}

fn main() {
    let ret = pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(
        pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10),
        vec![9, 5, 3, 10, 10, 12, 14]
    );
    assert_eq!(pivot_array(vec![-3, 4, 3, 2], 2), vec![-3, 2, 4, 3]);
}
