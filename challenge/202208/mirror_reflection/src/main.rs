fn mirror_reflection(p: i32, q: i32) -> i32 {
    let mut height = q;
    let mut to_left = true;
    let mut to_up = 1;

    while height != 0 && height != p {
        to_left = !to_left;
        let mut h = height + (to_up * q);
        if h > p {
            h = p - (q - (p - height));
            to_up = -1;
        } else if h < 0 {
            h = q - height;
            to_up = 1;
        }

        height = h;
    }

    if height == 0 {
        0
    } else if to_left {
        1
    } else {
        2
    }
}

fn main() {
    let ret = mirror_reflection(2, 1);
    println!("ret={ret}");
}

#[test]
fn test_mirror_reflection() {
    {
        let ret = mirror_reflection(2, 1);
        assert_eq!(ret, 2);
    }
    {
        let ret = mirror_reflection(3, 1);
        assert_eq!(ret, 1);
    }
}
