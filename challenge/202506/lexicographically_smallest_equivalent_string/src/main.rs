fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
    fn f(c: u8, graph: &Vec<Vec<u8>>, checked: &mut Vec<bool>) -> u8 {
        let idx = (c - b'a') as usize;
        checked[idx] = true;

        let mut ret = c;
        for &next in &graph[idx] {
            let idx = (next - b'a') as usize;
            if !checked[idx] {
                ret = std::cmp::min(ret, f(next, graph, checked));
            }
        }

        ret
    }

    let mut graph = vec![vec![]; 26];
    for (b1, b2) in s1.bytes().zip(s2.bytes()) {
        let idx1 = (b1 - b'a') as usize;
        let idx2 = (b2 - b'a') as usize;

        graph[idx1].push(b2);
        graph[idx2].push(b1);
    }

    let mut min_table = ['a'; 26];
    for i in 0..26 {
        let mut checked = vec![false; 26];
        let b = i + b'a';
        let ret = f(b, &graph, &mut checked);
        for (i, check) in checked.into_iter().enumerate() {
            if check {
                min_table[i] = ret as char;
            }
        }
    }

    let mut ret = String::new();
    for b in base_str.bytes() {
        ret.push(min_table[(b - b'a') as usize]);
    }

    ret
}
fn main() {
    let s1 = "parker".to_string();
    let s2 = "morris".to_string();
    let base_str = "parser".to_string();
    let ret = smallest_equivalent_string(s1, s2, base_str);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let s1 = "parker".to_string();
        let s2 = "morris".to_string();
        let base_str = "parser".to_string();
        let ret = smallest_equivalent_string(s1, s2, base_str);
        assert_eq!(ret, "makkek");
    }
    {
        let s1 = "hello".to_string();
        let s2 = "world".to_string();
        let base_str = "hold".to_string();
        let ret = smallest_equivalent_string(s1, s2, base_str);
        assert_eq!(ret, "hdld");
    }
    {
        let s1 = "leetcode".to_string();
        let s2 = "programs".to_string();
        let base_str = "sourcecode".to_string();
        let ret = smallest_equivalent_string(s1, s2, base_str);
        assert_eq!(ret, "aauaaaaada");
    }
}
