fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
    let mut costs = costs;
    costs.sort_unstable_by(|a, b| (a[0] - a[1]).cmp(&(b[0] - b[1])));

    let mut ret = 0;
    let half = costs.len() / 2;
    for i in 0..half {
        ret += costs[i][0] + costs[i + half][1];
    }
    ret
}

fn main() {
    let costs = vec![vec![10, 20], vec![30, 200], vec![400, 50], vec![30, 20]];
    let ret = two_city_sched_cost(costs);
    println!("ret={ret}");
}

#[test]
fn test_two_city_sched_cost() {
    {
        let costs = vec![vec![10, 20], vec![30, 200], vec![400, 50], vec![30, 20]];
        let ret = two_city_sched_cost(costs);
        assert_eq!(ret, 110);
    }
    {
        let costs = vec![
            vec![259, 770],
            vec![448, 54],
            vec![926, 667],
            vec![184, 139],
            vec![840, 118],
            vec![577, 469],
        ];
        let ret = two_city_sched_cost(costs);
        assert_eq!(ret, 1859);
    }
    {
        let costs = vec![
            vec![515, 563],
            vec![451, 713],
            vec![537, 709],
            vec![343, 819],
            vec![855, 779],
            vec![457, 60],
            vec![650, 359],
            vec![631, 42],
        ];
        let ret = two_city_sched_cost(costs);
        assert_eq!(ret, 3086);
    }
}
