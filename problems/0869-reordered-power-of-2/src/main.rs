fn reordered_power_of2(n: i32) -> bool {
    let f = |n: i32| -> Vec<i32> {
        let mut n = n;
        let mut ret = vec![];
        while n > 0 {
            ret.push(n % 10);
            n /= 10;
        }

        ret.sort_unstable();
        ret
    };

    let v = f(n);
    let mut m = 1;
    loop {
        let vv = f(m);
        if v == vv {
            return true;
        }

        if vv.len() > v.len() {
            return false;
        }

        m <<= 1;
    }
}

fn main() {
    assert!(reordered_power_of2(1));
    println!("ret={}", reordered_power_of2(65563));
}

#[test]
fn test_reordered_power_of2() {
    assert!(reordered_power_of2(1));
    assert!(!reordered_power_of2(10));
    assert!(reordered_power_of2(16));
    assert!(!reordered_power_of2(24));
    assert!(reordered_power_of2(46));
}
