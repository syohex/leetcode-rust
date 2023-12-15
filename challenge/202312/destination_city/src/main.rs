fn dest_city(paths: Vec<Vec<String>>) -> String {
    use std::collections::HashSet;
    let mut cities = HashSet::new();
    let mut froms = HashSet::new();

    for path in &paths {
        cities.insert(&path[0]);
        cities.insert(&path[1]);
        froms.insert(&path[0]);
    }

    for city in cities {
        if !froms.contains(&city) {
            return city.clone();
        }
    }

    panic!("never reach here");
}

fn main() {
    let paths = vec![
        vec!["London".to_string(), "New York".to_string()],
        vec!["New York".to_string(), "Lima".to_string()],
        vec!["Lima".to_string(), "Sao Paulo".to_string()],
    ];
    let ret = dest_city(paths);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let paths = vec![
            vec!["London".to_string(), "New York".to_string()],
            vec!["New York".to_string(), "Lima".to_string()],
            vec!["Lima".to_string(), "Sao Paulo".to_string()],
        ];
        let ret = dest_city(paths);
        assert_eq!(ret, "Sao Paulo");
    }
    {
        let paths = vec![
            vec!["B".to_string(), "C".to_string()],
            vec!["D".to_string(), "B".to_string()],
            vec!["C".to_string(), "A".to_string()],
        ];
        let ret = dest_city(paths);
        assert_eq!(ret, "A");
    }
    {
        let paths = vec![vec!["A".to_string(), "Z".to_string()]];
        let ret = dest_city(paths);
        assert_eq!(ret, "Z");
    }
}
