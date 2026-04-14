fn minimum_total_distance(robot: Vec<i32>, factory: Vec<Vec<i32>>) -> i64 {
    use std::collections::HashMap;

    fn f(
        robot_idx: usize,
        factory_idx: usize,
        robots: &[i32],
        factories: &[i32],
        cache: &mut HashMap<(usize, usize), i64>,
    ) -> i64 {
        if robot_idx >= robots.len() {
            return 0;
        }

        if factory_idx >= factories.len() {
            return 2i64.pow(60);
        }

        let key = (robot_idx, factory_idx);
        if let Some(v) = cache.get(&key) {
            return *v;
        }

        let distance = (robots[robot_idx] - factories[factory_idx]).abs() as i64;
        let ret1 = distance + f(robot_idx + 1, factory_idx + 1, robots, factories, cache);
        let ret2 = f(robot_idx, factory_idx + 1, robots, factories, cache);
        let ret = std::cmp::min(ret1, ret2);
        cache.insert(key, ret);

        ret
    }

    let mut robots = robot;
    let mut factory = factory;

    robots.sort_unstable();
    factory.sort_unstable();

    let mut factories = vec![];
    for f in factory {
        for _ in 0..f[1] {
            factories.push(f[0]);
        }
    }

    let mut cache = HashMap::new();
    f(0, 0, &robots, &factories, &mut cache)
}

fn main() {
    let robot = vec![0, 4, 6];
    let factory = vec![vec![2, 2], vec![6, 2]];
    let ret = minimum_total_distance(robot, factory);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let robot = vec![0, 4, 6];
        let factory = vec![vec![2, 2], vec![6, 2]];
        let ret = minimum_total_distance(robot, factory);
        assert_eq!(ret, 4);
    }
    {
        let robot = vec![1, -1];
        let factory = vec![vec![-2, 1], vec![2, 1]];
        let ret = minimum_total_distance(robot, factory);
        assert_eq!(ret, 2);
    }
}
