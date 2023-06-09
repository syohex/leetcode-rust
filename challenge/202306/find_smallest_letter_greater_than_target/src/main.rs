fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    let len = letters.len() as i32;
    let mut left = 0;
    let mut right = len - 1;

    while left <= right {
        let mid = left + (right - left) / 2;
        if letters[mid as usize] > target {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    if left >= len {
        letters[0]
    } else {
        letters[left as usize]
    }
}

fn main() {
    let letters = vec!['c', 'f', 'j'];
    let ret = next_greatest_letter(letters, 'a');
    println!("ret={ret}");
}

#[test]
fn test_next_greatest_letter() {
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
