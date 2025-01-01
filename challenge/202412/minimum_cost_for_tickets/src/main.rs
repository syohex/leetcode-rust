fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
    use std::cmp::min;
    use std::collections::HashMap;

    fn f(pos: usize, days: &Vec<i32>, costs: &Vec<i32>, cache: &mut HashMap<usize, i32>) -> i32 {
        if pos >= days.len() {
            return 0;
        }

        if let Some(v) = cache.get(&pos) {
            return *v;
        }

        let ret1 = costs[0] + f(pos + 1, days, costs, cache);
        let mut p = pos;
        for i in pos..days.len() {
            if days[i] - days[pos] < 7 {
                p = i;
            } else {
                break;
            }
        }

        let ret7 = costs[1] + f(p + 1, days, costs, cache);
        let mut p = pos;
        for i in pos..days.len() {
            if days[i] - days[pos] < 30 {
                p = i;
            } else {
                break;
            }
        }

        let ret30 = costs[2] + f(p + 1, days, costs, cache);
        println!("pos={pos}, ret1={ret1}, ret7={ret7}, ret30={ret30}");
        let ret = min(ret1, min(ret7, ret30));
        cache.insert(pos, ret);
        ret
    }

    let mut cache = HashMap::new();
    f(0, &days, &costs, &mut cache)
}

fn main() {
    let days = vec![1, 4, 6, 7, 8, 20];
    let costs = vec![2, 7, 15];
    let ret = mincost_tickets(days, costs);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let days = vec![1, 4, 6, 7, 8, 20];
        let costs = vec![2, 7, 15];
        let ret = mincost_tickets(days, costs);
        assert_eq!(ret, 11);
    }
    {
        let days = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31];
        let costs = vec![2, 7, 15];
        let ret = mincost_tickets(days, costs);
        assert_eq!(ret, 17);
    }
}
