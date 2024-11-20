fn take_character(s: String, k: i32) -> i32 {
    fn to_index(b: u8) -> usize {
        (b - b'a') as usize
    }

    let bs: Vec<u8> = s.bytes().collect();
    let mut reminders = [0; 3];
    for &b in &bs {
        reminders[to_index(b)] += 1;
    }

    for &t in &reminders {
        if t < k {
            return -1;
        }
    }

    let mut left = 0;
    let mut right = 0;

    let mut count = [0; 3];

    let mut ret = 0;
    while right < bs.len() {
        count[to_index(bs[right])] += 1;

        while left <= right
            && (reminders[0] - count[0] < k || reminders[1] - count[1] < k || reminders[2] - count[2] < k)
        {
            count[to_index(bs[left])] -= 1;
            left += 1;
        }

        ret = std::cmp::max(ret, right - left + 1);
        right += 1;
    }

    (bs.len() - ret) as i32
}

fn main() {
    let s = "aabaaaacaabc".to_string();
    let ret = take_character(s, 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let s = "aabaaaacaabc".to_string();
        let ret = take_character(s, 2);
        assert_eq!(ret, 8);
    }
    {
        let s = "a".to_string();
        let ret = take_character(s, 1);
        assert_eq!(ret, -1);
    }
    {
        let s = "abc".to_string();
        let ret = take_character(s, 0);
        assert_eq!(ret, 0);
    }
    {
        let s = "abc".to_string();
        let ret = take_character(s, 1);
        assert_eq!(ret, 3);
    }
}
