fn minimum_flips(n: i32) -> i32 {
    let s = format!("{n:b}");
    let cs : Vec<_> = s.chars().collect();
    let reverse : Vec<_> = s.chars().rev().collect();

    cs.into_iter().zip(reverse.into_iter()).filter(|(a, b)| a != b).count() as i32
}

fn main() {
    let ret = minimum_flips(10);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(minimum_flips(7), 0);
    assert_eq!(minimum_flips(10), 4);
}
