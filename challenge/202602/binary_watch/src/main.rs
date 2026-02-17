fn read_binary_watch(turned_on: i32) -> Vec<String> {
    let ones: Vec<_> = (0..60i32).map(|n| n.count_ones() as i32).collect();
    let mut ret = vec![];

    for m in 0..12 {
        for s in 0..60 {
            if ones[m] + ones[s] == turned_on {
                ret.push(format!("{}:{:02}", m, s));
            }
        }
    }

    ret
}

fn main() {
    let ret = read_binary_watch(2);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let expected = [
            "0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00",
        ];
        let ret = read_binary_watch(1);
        assert_eq!(ret, expected);
    }
    {
        let ret = read_binary_watch(9);
        assert!(ret.is_empty());
    }
}
