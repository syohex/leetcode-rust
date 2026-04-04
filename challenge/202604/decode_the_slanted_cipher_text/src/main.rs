fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
    let cs: Vec<_> = encoded_text.chars().collect();
    let rows = rows as usize;
    let cols = encoded_text.len() / rows;

    let mut ret = String::new();
    for i in 0..cols {
        let mut row = 0;
        let mut col = i;
        while row < rows && col < cols {
            let pos = row * cols + col;
            ret.push(cs[pos]);

            row += 1;
            col += 1;
        }
    }

    ret.trim_end().to_string()
}

fn main() {
    let ret = decode_ciphertext("iveo    eed   l te   olc".to_string(), 4);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(decode_ciphertext("ch   ie   pr".to_string(), 3), "cipher");
    assert_eq!(
        decode_ciphertext("iveo    eed   l te   olc".to_string(), 4),
        "i love leetcode"
    );
    assert_eq!(decode_ciphertext("coding".to_string(), 1), "coding");
}
