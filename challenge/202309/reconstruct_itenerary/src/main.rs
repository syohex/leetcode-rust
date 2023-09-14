fn find_itenerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    use std::collections::HashMap;

    fn f<'a>(
        airport: &'a str,
        used: usize,
        graph: &HashMap<&'a str, Vec<&'a str>>,
        limit: usize,
        visited: &mut Vec<&'a str>,
        tickets: &mut HashMap<(&'a str, &'a str), i32>,
        ret: &mut Vec<&'a str>,
    ) {
        if used == limit {
            if ret.is_empty() || visited < ret {
                *ret = visited.clone();
            }

            return;
        }

        if let Some(nexts) = graph.get(airport) {
            for next in nexts {
                let key = (airport, *next);
                if !tickets.contains_key(&key) {
                    continue;
                }

                let count = *tickets.get(&key).unwrap();
                if count > 0 {
                    tickets.insert(key, count - 1);
                    visited.push(next);

                    f(next, used + 1, graph, limit, visited, tickets, ret);

                    visited.pop();
                    tickets.insert(key, count);
                }
            }
        }
    }

    let limit = tickets.len();
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut used = HashMap::new();
    for ticket in &tickets {
        graph.entry(&ticket[0]).or_insert(vec![]).push(&ticket[1]);
        *used
            .entry((ticket[0].as_str(), ticket[1].as_str()))
            .or_insert(0) += 1;
    }

    let mut visited = vec!["JFK"];
    let mut ret = vec![];

    f("JFK", 0, &graph, limit, &mut visited, &mut used, &mut ret);
    ret.into_iter().map(|s| s.to_string()).collect()
}

fn main() {
    let tickets = vec![
        vec!["MUC".to_string(), "LHR".to_string()],
        vec!["JFK".to_string(), "MUC".to_string()],
        vec!["SFO".to_string(), "SJC".to_string()],
        vec!["LHR".to_string(), "SFO".to_string()],
    ];
    let ret = find_itenerary(tickets);
    println!("ret={ret:?}");
}

#[test]
fn test_find_itinerary() {
    {
        let tickets = vec![
            vec!["MUC".to_string(), "LHR".to_string()],
            vec!["JFK".to_string(), "MUC".to_string()],
            vec!["SFO".to_string(), "SJC".to_string()],
            vec!["LHR".to_string(), "SFO".to_string()],
        ];
        let expected = vec![
            "JFK".to_string(),
            "MUC".to_string(),
            "LHR".to_string(),
            "SFO".to_string(),
            "SJC".to_string(),
        ];
        let ret = find_itenerary(tickets);
        assert_eq!(ret, expected);
    }
    {
        let tickets = vec![
            vec!["JFK".to_string(), "SFO".to_string()],
            vec!["JFK".to_string(), "ATL".to_string()],
            vec!["SFO".to_string(), "ATL".to_string()],
            vec!["ATL".to_string(), "JFK".to_string()],
            vec!["ATL".to_string(), "SFO".to_string()],
        ];
        let expected = vec![
            "JFK".to_string(),
            "ATL".to_string(),
            "JFK".to_string(),
            "SFO".to_string(),
            "ATL".to_string(),
            "SFO".to_string(),
        ];
        let ret = find_itenerary(tickets);
        assert_eq!(ret, expected);
    }
    {
        let tickets = vec![
            vec!["EZE".to_string(), "AXA".to_string()],
            vec!["TIA".to_string(), "ANU".to_string()],
            vec!["ANU".to_string(), "JFK".to_string()],
            vec!["JFK".to_string(), "ANU".to_string()],
            vec!["ANU".to_string(), "EZE".to_string()],
            vec!["TIA".to_string(), "ANU".to_string()],
            vec!["AXA".to_string(), "TIA".to_string()],
            vec!["TIA".to_string(), "JFK".to_string()],
            vec!["ANU".to_string(), "TIA".to_string()],
            vec!["JFK".to_string(), "TIA".to_string()],
        ];
        let expected = vec![
            "JFK".to_string(),
            "ANU".to_string(),
            "EZE".to_string(),
            "AXA".to_string(),
            "TIA".to_string(),
            "ANU".to_string(),
            "JFK".to_string(),
            "TIA".to_string(),
            "ANU".to_string(),
            "TIA".to_string(),
            "JFK".to_string(),
        ];
        let ret = find_itenerary(tickets);
        assert_eq!(ret, expected);
    }
}
