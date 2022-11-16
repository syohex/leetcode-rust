static mut PICKED: i32 = 0;

unsafe fn guess(num: i32) -> i32 {
    if num > PICKED {
        -1
    } else if num == PICKED {
        0
    } else {
        1
    }
}

unsafe fn guess_number(n: i32) -> i32 {
    let mut left = 1;
    let mut right = n;

    while left <= right {
        let mid = ((left as i64 + right as i64) / 2) as i32;
        let v = guess(mid);
        if v == 0 {
            return mid;
        }

        if v == -1 {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    -1
}

fn main() {
    unsafe {
        PICKED = 6;
        let ret = guess_number(10);
        println!("ret={ret}");
    }
}

#[test]
fn test_guess_number() {
    unsafe {
        {
            PICKED = 6;
            let ret = guess_number(10);
            assert_eq!(ret, PICKED);
        }
        {
            PICKED = 1;
            let ret = guess_number(1);
            assert_eq!(ret, PICKED);
        }
        {
            PICKED = 1;
            let ret = guess_number(2);
            assert_eq!(ret, PICKED);
        }
        {
            PICKED = 1702766719;
            let ret = guess_number(2126753390);
            assert_eq!(ret, PICKED);
        }
    }
}
