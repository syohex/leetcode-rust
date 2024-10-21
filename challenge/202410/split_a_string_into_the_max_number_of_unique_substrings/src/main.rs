fn max_unique_split(s: String) -> i32 {
    use std::collections::HashSet;

    fn f(pos: usize, cs: &Vec<char>, substrs: &mut HashSet<String>) -> usize {
        if pos == cs.len() {
            return substrs.len();
        }

        let mut ret = 0;
        let mut acc = String::new();
        for i in pos..cs.len() {
            acc.push(cs[i]);

            if !substrs.contains(&acc) {
                substrs.insert(acc.clone());
                ret = std::cmp::max(ret, f(i + 1, cs, substrs));
                substrs.remove(&acc);
            }
        }
        ret
    }

    let cs = s.chars().collect();
    let mut substrs = HashSet::new();
    f(0, &cs, &mut substrs) as i32
}

fn main() {
    let s = "ababccc".to_string();
    let ret = max_unique_split(s);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let s = "ababccc".to_string();
        let ret = max_unique_split(s);
        assert_eq!(ret, 5);
    }
    {
        let s = "aba".to_string();
        let ret = max_unique_split(s);
        assert_eq!(ret, 2);
    }
    {
        let s = "aa".to_string();
        let ret = max_unique_split(s);
        assert_eq!(ret, 1);
    }
}
