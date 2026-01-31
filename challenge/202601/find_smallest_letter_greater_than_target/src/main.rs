fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    let pos = letters.partition_point(|&c| c <= target);
    if pos == letters.len() {
        letters[0]
    } else {
        letters[pos]
    }
}

fn main() {
    let letters = vec!['c', 'f', 'j'];
    let ret = next_greatest_letter(letters, 'c');
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let letters = vec!['c', 'f', 'j'];
        let ret = next_greatest_letter(letters, 'a');
        assert_eq!(ret, 'c');
    }
    {
        let letters = vec!['c', 'f', 'j'];
        let ret = next_greatest_letter(letters, 'c');
        assert_eq!(ret, 'f');
    }
    {
        let letters = vec!['x', 'x', 'y', 'y'];
        let ret = next_greatest_letter(letters, 'z');
        assert_eq!(ret, 'x');
    }
}
