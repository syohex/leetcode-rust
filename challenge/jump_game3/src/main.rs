fn can_reach(arr: Vec<i32>, start: i32) -> bool {
    fn f(pos: i32, arr: &[i32], visited: &mut Vec<bool>) -> bool {
        if pos < 0 || pos >= arr.len() as i32 {
            return false;
        }

        let pos = pos as usize;
        if visited[pos] {
            return false;
        }
        if arr[pos] == 0 {
            return true;
        }

        visited[pos] = true;
        f(pos as i32 + arr[pos], arr, visited) || f(pos as i32 - arr[pos], arr, visited)
    }

    let mut visited = vec![false; arr.len()];
    f(start, &arr, &mut visited)
}

fn main() {
    let ret = can_reach(vec![4, 2, 3, 0, 3, 1, 2], 5);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(can_reach(vec![4, 2, 3, 0, 3, 1, 2], 5));
    assert!(can_reach(vec![4, 2, 3, 0, 3, 1, 2], 0));
    assert!(!can_reach(vec![3, 0, 2, 1, 2], 2));
}
