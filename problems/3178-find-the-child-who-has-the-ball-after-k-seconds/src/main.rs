fn number_of_child(n: i32, k: i32) -> i32 {
    let mut pos = 0;
    let mut dir = 1;

    for _ in 1..=k {
        pos += dir;

        if pos == n - 1 {
            dir = -1;
        } else if pos == 0 {
            dir = 1;
        }
    }

    pos
}

fn main() {
    let ret = number_of_child(5, 6);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let ret = number_of_child(3, 5);
        assert_eq!(ret, 1);
    }
    {
        let ret = number_of_child(5, 6);
        assert_eq!(ret, 2);
    }
    {
        let ret = number_of_child(4, 2);
        assert_eq!(ret, 2);
    }
}
