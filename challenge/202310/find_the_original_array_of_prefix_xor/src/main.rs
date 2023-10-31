fn find_array(pref: Vec<i32>) -> Vec<i32> {
    pref.iter()
        .skip(1)
        .fold((vec![pref[0]], pref[0]), |(mut ret, acc), &n| {
            let acc1 = acc ^ n;
            ret.push(acc1);
            (ret, acc ^ acc1)
        })
        .0
}

fn main() {
    let pref = vec![5, 2, 0, 3, 1];
    let ret = find_array(pref);
    println!("ret={ret:?}");
}

#[test]
fn test_find_array() {
    {
        let pref = vec![5, 2, 0, 3, 1];
        let expected = vec![5, 7, 2, 3, 2];
        let ret = find_array(pref);
        assert_eq!(ret, expected);
    }
    {
        let pref = vec![13];
        let expected = vec![13];
        let ret = find_array(pref);
        assert_eq!(ret, expected);
    }
}
