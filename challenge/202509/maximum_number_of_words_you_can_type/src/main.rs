fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
    let broken = broken_letters.bytes().fold([false; 26], |mut acc, b| {
        acc[(b - b'a') as usize] = true;
        acc
    });

    text.split_whitespace().fold(0, |acc, word| {
        if word.bytes().any(|b| broken[(b - b'a') as usize]) {
            acc
        } else {
            acc + 1
        }
    })
}

fn main() {
    let ret = can_be_typed_words("hello world".to_string(), "ad".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(
        can_be_typed_words("hello world".to_string(), "ad".to_string()),
        1
    );
    assert_eq!(
        can_be_typed_words("leet code".to_string(), "lt".to_string()),
        1
    );
    assert_eq!(
        can_be_typed_words("leet code".to_string(), "e".to_string()),
        0
    );
}
