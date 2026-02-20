fn toggle_light_bulbs(bulbs: Vec<i32>) -> Vec<i32> {
    use std::collections::HashSet;

    let mut s = HashSet::new();
    for b in bulbs {
        if s.contains(&b) {
            s.remove(&b);
        } else {
            s.insert(b);
        }
    }

    let mut ret: Vec<_> = s.into_iter().collect();
    ret.sort_unstable();
    ret
}

fn main() {
    let ret = toggle_light_bulbs(vec![10, 30, 20, 10]);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(toggle_light_bulbs(vec![10, 30, 20, 10]), [20, 30]);
    assert_eq!(toggle_light_bulbs(vec![100, 100]), []);
}
