fn earlist_finish_time(
    land_start_time: Vec<i32>,
    land_duration: Vec<i32>,
    water_start_time: Vec<i32>,
    water_duration: Vec<i32>,
) -> i32 {
    let mut ret = i32::MAX;
    for (s1, d1) in land_start_time.iter().zip(&land_duration) {
        for (s2, d2) in water_start_time.iter().zip(&water_duration) {
            if s1 + d1 >= *s2 {
                ret = std::cmp::min(ret, s1 + d1 + d2);
            } else {
                ret = std::cmp::min(ret, s2 + d2);
            }
        }
    }

    for (s1, d1) in water_start_time.iter().zip(&water_duration) {
        for (s2, d2) in land_start_time.iter().zip(&land_duration) {
            if s1 + d1 >= *s2 {
                ret = std::cmp::min(ret, s1 + d1 + d2);
            } else {
                ret = std::cmp::min(ret, s2 + d2);
            }
        }
    }

    ret
}

fn main() {
    let land_start_time = vec![2, 8];
    let land_duration = vec![4, 1];
    let water_start_time = vec![6];
    let water_duration = vec![3];
    let ret = earlist_finish_time(
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
        let land_start_time = vec![99];
        let land_duration = vec![59];
        let water_start_time = vec![99, 54];
        let water_duration = vec![85, 20];
        let ret = earlist_finish_time(
            land_start_time,
            land_duration,
            water_start_time,
            water_duration,
        );
        assert_eq!(ret, 158);
    }
    {
        let land_start_time = vec![2, 8];
        let land_duration = vec![4, 1];
        let water_start_time = vec![6];
        let water_duration = vec![3];
        let ret = earlist_finish_time(
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
        let ret = earlist_finish_time(
            land_start_time,
            land_duration,
            water_start_time,
            water_duration,
        );
        assert_eq!(ret, 14);
    }
}
