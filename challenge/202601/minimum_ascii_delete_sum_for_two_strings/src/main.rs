fn minimum_delete_sum(s1: String, s2: String) -> i32 {
    fn f(pos1: usize, pos2: usize, cs1: &[char], cs2: &[char], cache: &mut Vec<Vec<i32>>) -> i32 {
        if pos1 >= cs1.len() && pos2 >= cs2.len() {
            return 0;
        }

        if cache[pos1][pos2] != -1 {
            return cache[pos1][pos2];
        }

        let v = if pos1 >= cs1.len() {
            cs2[pos2] as i32 + f(pos1, pos2 + 1, cs1, cs2, cache)
        } else if pos2 >= cs2.len() {
            cs1[pos1] as i32 + f(pos1 + 1, pos2, cs1, cs2, cache)
        } else if cs1[pos1] == cs2[pos2] {
            f(pos1 + 1, pos2 + 1, cs1, cs2, cache)
        } else {
            let v1 = cs1[pos1] as i32 + f(pos1 + 1, pos2, cs1, cs2, cache);
            let v2 = cs2[pos2] as i32 + f(pos1, pos2 + 1, cs1, cs2, cache);
            std::cmp::min(v1, v2)
        };

        cache[pos1][pos2] = v;
        v
    }

    let cs1: Vec<_> = s1.chars().collect();
    let cs2: Vec<_> = s2.chars().collect();
    let mut cache = vec![vec![-1; cs2.len() + 1]; cs1.len() + 1];
    f(0, 0, &cs1, &cs2, &mut cache)
}

fn main() {
    let ret = minimum_delete_sum("sea".to_string(), "eat".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(
        minimum_delete_sum("sea".to_string(), "eat".to_string()),
        231
    );
    assert_eq!(
        minimum_delete_sum("delete".to_string(), "leet".to_string()),
        403
    );
}
