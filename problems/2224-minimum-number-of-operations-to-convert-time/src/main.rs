fn convert_time(current: String, correct: String) -> i32 {
    fn to_time(time: &str) -> i32 {
        let bs = time.bytes().collect::<Vec<u8>>();
        (bs[0] as i32 * 10 + bs[1] as i32) * 60 + bs[3] as i32 * 10 + bs[4] as i32
    }

    let mut ret = 0;
    let diff = to_time(&correct) - to_time(&current);
dbg!(&diff);
    ret += diff / 60;
    let diff = diff % 60;

    ret += diff / 15;
    let diff = diff % 15;

    ret += diff / 5;
    let diff = diff % 5;

    ret + diff
}

fn main() {
    let current = "02:30".to_string();
    let correct = "04:35".to_string();
    let ret = convert_time(current, correct);
    println!("ret={ret}");
}

#[test]
fn test_convert_time() {
    {
        let current = "00:00".to_string();
        let correct = "23:59".to_string();
        assert_eq!(convert_time(current, correct), 32);
    }
    {
        let current = "02:30".to_string();
        let correct = "04:35".to_string();
        assert_eq!(convert_time(current, correct), 3);
    }
    {
        let current = "11:00".to_string();
        let correct = "11:01".to_string();
        assert_eq!(convert_time(current, correct), 1);
    }
    {
        let current = "11:00".to_string();
        let correct = "11:00".to_string();
        assert_eq!(convert_time(current, correct), 0);
    }
}
