fn sort_vowels(s: String) -> String {
    fn to_index(c: char) -> usize {
        match c {
            'a' => 0,
            'e' => 1,
            'i' => 2,
            'o' => 3,
            'u' => 4,
            _ => unreachable!(),
        }
    }

    let mut freq = [(0, s.len(), '?'); 5];
    for (i, c) in s.chars().enumerate() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                let idx = to_index(c);

                freq[idx].0 += 1;
                freq[idx].1 = std::cmp::min(freq[idx].1, i);
                freq[idx].2 = c;
            }
            _ => (),
        }
    }

    freq.sort_unstable_by_key(|(count, pos, _)| (std::cmp::Reverse(*count), *pos));

    let mut ret = String::new();
    for c in s.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                for (count, _, ch) in freq.iter_mut() {
                    if *count >= 1 {
                        *count -= 1;
                        ret.push(*ch);
                        break;
                    }
                }
            }
            _ => ret.push(c),
        }
    }

    ret
}
fn main() {
    let ret = sort_vowels("aeiaaioooa".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(sort_vowels("leetcode".to_string()), "leetcedo");
    assert_eq!(sort_vowels("aeiaaioooa".to_string()), "aaaaoooiie");
    assert_eq!(sort_vowels("baeiou".to_string()), "baeiou");
}
