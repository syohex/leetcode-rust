fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
    use std::collections::HashSet;

    fn f(node: i32, limit: usize, rooms: &Vec<Vec<i32>>, visited: &mut HashSet<i32>) -> bool {
        if visited.len() == limit {
            return true;
        }

        for num in &rooms[node as usize] {
            if !visited.contains(num) {
                visited.insert(*num);
                if f(*num, limit, rooms, visited) {
                    return true;
                }
            }
        }

        false
    }

    let mut visited = HashSet::new();
    visited.insert(0);

    f(0, rooms.len(), &rooms, &mut visited)
}

fn main() {
    let rooms = vec![vec![1], vec![2], vec![3], vec![]];
    let ret = can_visit_all_rooms(rooms);
    println!("ret={ret}");
}

#[test]
fn test_can_visit_all_rooms() {
    {
        let rooms = vec![vec![1], vec![2], vec![3], vec![]];
        let ret = can_visit_all_rooms(rooms);
        assert!(ret);
    }
    {
        let rooms = vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]];
        let ret = can_visit_all_rooms(rooms);
        assert!(!ret);
    }
    {
        let rooms = vec![vec![2, 3], vec![], vec![2], vec![1, 3]];
        let ret = can_visit_all_rooms(rooms);
        assert!(ret);
    }
}
