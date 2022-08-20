fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
    let mut dp = vec![0; stations.len() + 1];
    dp[0] = start_fuel;

    let len = stations.len();
    for i in 0..len {
        for j in (0..=i).rev() {
            if dp[j] >= stations[i][0] {
                dp[j + 1] = std::cmp::max(dp[j + 1], dp[j] + stations[i][1]);
            }
        }
    }

    for i in 0..dp.len() {
        if dp[i] >= target {
            return i as i32;
        }
    }

    -1
}

fn main() {
    let stations = vec![vec![10, 60], vec![20, 30], vec![30, 30], vec![60, 40]];
    let ret = min_refuel_stops(100, 1, stations);
    println!("ret={ret}");
}

#[test]
fn test_min_refuel_stops() {
    {
        let ret = min_refuel_stops(1, 1, vec![]);
        assert_eq!(ret, 0);
    }
    {
        let ret = min_refuel_stops(100, 1, vec![vec![10, 100]]);
        assert_eq!(ret, -1);
    }
    {
        let stations = vec![vec![10, 60], vec![20, 30], vec![30, 30], vec![60, 40]];
        let ret = min_refuel_stops(100, 10, stations);
        assert_eq!(ret, 2);
    }
    {
        let stations = vec![vec![25, 25], vec![50, 50]];
        let ret = min_refuel_stops(100, 50, stations);
        assert_eq!(ret, 1);
    }

    {
        let stations = vec![
            vec![25, 27],
            vec![36, 187],
            vec![140, 186],
            vec![378, 6],
            vec![492, 202],
            vec![517, 89],
            vec![579, 234],
            vec![673, 86],
            vec![808, 53],
            vec![954, 49],
        ];
        let ret = min_refuel_stops(1000, 83, stations);
        assert_eq!(ret, -1);
    }
}
