fn does_valid_array_exist(derived: Vec<i32>) -> bool {
    let ret1 = derived.iter().fold(0, |prev, &n| prev ^ n);
    let ret2 = derived.iter().fold(1, |prev, &n| prev ^ n);
    ret1 == 0 || ret2 == 1
}

fn main() {
    let derived = vec![1, 1, 0];
    let ret = does_valid_array_exist(derived);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let derived = vec![1, 1, 0];
        assert!(does_valid_array_exist(derived));
    }
    {
        let derived = vec![1, 1];
        assert!(does_valid_array_exist(derived));
    }
    {
        let derived = vec![1, 0];
        assert!(!does_valid_array_exist(derived));
    }
}
