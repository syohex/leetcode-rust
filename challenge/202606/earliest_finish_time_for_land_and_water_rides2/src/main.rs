fn earliest_finish_time(
    land_start_time: Vec<i32>,
    land_duration: Vec<i32>,
    water_start_time: Vec<i32>,
    water_duration: Vec<i32>,
) -> i32 {
    fn f(s1: &[i32], d1: &[i32], s2: &[i32], d2: &[i32]) -> i32 {
        let mut ret1 = i32::MAX;
        for (s, d) in s1.iter().zip(d1) {
            ret1 = std::cmp::min(ret1, s + d);
        }

        let mut ret2 = i32::MAX;
        for (s, d) in s2.iter().zip(d2) {
            ret2 = std::cmp::min(ret2, std::cmp::max(ret1, *s) + d);
        }

        ret2
    }

    let ret1 = f(
        &land_start_time,
        &land_duration,
        &water_start_time,
        &water_duration,
    );
    let ret2 = f(
        &water_start_time,
        &water_duration,
        &land_start_time,
        &land_duration,
    );
    std::cmp::min(ret1, ret2)
}

fn main() {
    let land_start_time = vec![2, 8];
    let land_duration = vec![4, 1];
    let water_start_time = vec![6];
    let water_duration = vec![3];
    let ret = earliest_finish_time(
        land_start_time,
        land_duration,
        water_start_time,
        water_duration,
    );
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let land_start_time = vec![2, 8];
        let land_duration = vec![4, 1];
        let water_start_time = vec![6];
        let water_duration = vec![3];
        let ret = earliest_finish_time(
            land_start_time,
            land_duration,
            water_start_time,
            water_duration,
        );
        assert_eq!(ret, 9);
    }
    {
        let land_start_time = vec![5];
        let land_duration = vec![3];
        let water_start_time = vec![1];
        let water_duration = vec![10];
        let ret = earliest_finish_time(
            land_start_time,
            land_duration,
            water_start_time,
            water_duration,
        );
        assert_eq!(ret, 14);
    }
}
