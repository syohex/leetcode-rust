fn unique_morse_representations(words: Vec<String>) -> i32 {
    use std::collections::HashSet;

    let codes = [
        ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
        "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
        "--..",
    ];

    let mut hs = HashSet::new();
    for word in words {
        let s = word.bytes().fold(String::new(), |mut acc, b| {
            let index = (b - b'a') as usize;
            acc.push_str(codes[index]);
            acc
        });

        hs.insert(s);
    }

    hs.len() as i32
}

fn main() {
    let words = vec![
        "gin".to_string(),
        "zen".to_string(),
        "gig".to_string(),
        "msg".to_string(),
    ];
    let ret = unique_morse_representations(words);
    println!("ret={ret}");
}

#[test]
fn test_unique_morse_representations() {
    {
        let words = vec![
            "gin".to_string(),
            "zen".to_string(),
            "gig".to_string(),
            "msg".to_string(),
        ];
        let ret = unique_morse_representations(words);
        assert_eq!(ret, 2);
    }
    {
        let words = vec!["a".to_string()];
        let ret = unique_morse_representations(words);
        assert_eq!(ret, 1);
    }
}
