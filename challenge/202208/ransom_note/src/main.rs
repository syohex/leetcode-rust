fn can_construct(random_note: String, magazine: String) -> bool {
    fn f(s: &str) -> Vec<i32> {
        s.bytes().fold(vec![0;26], |mut acc, b| {
            let index = (b - b'a') as usize;
            acc[index] += 1;
            acc
        })
    }

    let a = f(&random_note);
    let b = f(&magazine);

    a.into_iter().zip(b).all(|(i, j)| {
        i <= j
    })
}

fn main() {
        let random_note = "aa".to_string();
        let magazine = "aab".to_string();
        let ret = can_construct(random_note, magazine);
    println!("ret={ret}");
}

#[test]
fn test_can_construct() {
    {
        let random_note = "a".to_string();
        let magazine = "b".to_string();
        let ret = can_construct(random_note, magazine);
        assert!(!ret);
    }
    {
        let random_note = "aa".to_string();
        let magazine = "ab".to_string();
        let ret = can_construct(random_note, magazine);
        assert!(!ret);
    }
    {
        let random_note = "aa".to_string();
        let magazine = "aab".to_string();
        let ret = can_construct(random_note, magazine);
        assert!(ret);
    }
}
