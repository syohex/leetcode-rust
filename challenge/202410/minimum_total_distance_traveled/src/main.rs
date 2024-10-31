fn minimum_total_distance(robot: Vec<i32>, factory: Vec<Vec<i32>>) -> i64 {
    use std::collections::HashMap;

    fn f(
        robots: &Vec<i32>,
        factories: &Vec<i32>,
        i: usize,
        j: usize,
        cache: &mut HashMap<(usize, usize), i64>,
    ) -> i64 {
        if i >= robots.len() {
            return 0;
        }
        if j >= factories.len() {
            return 2i64.pow(40) as i64;
        }

        let key = (i, j);
        if let Some(&v) = cache.get(&key) {
            return v;
        }

        let ret1 =
            (robots[i] - factories[j]).abs() as i64 + f(robots, factories, i + 1, j + 1, cache);
        let ret2 = f(robots, factories, i, j + 1, cache);
        let ret = std::cmp::min(ret1, ret2);

        cache.insert(key, ret);
        return ret;
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
    f(&robots, &factories, 0, 0, &mut cache)
}

fn main() {
    let robots = vec![0, 4, 6];
    let factory = vec![vec![2, 2], vec![6, 2]];
    let ret = minimum_total_distance(robots, factory);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let robots = vec![0, 4, 6];
        let factory = vec![vec![2, 2], vec![6, 2]];
        let ret = minimum_total_distance(robots, factory);
        assert_eq!(ret, 4);
    }
    {
        let robots = vec![1, -1];
        let factory = vec![vec![-2, 1], vec![2, 1]];
        let ret = minimum_total_distance(robots, factory);
        assert_eq!(ret, 2);
    }
}
