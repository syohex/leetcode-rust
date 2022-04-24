use std::collections::HashMap;

struct UndergroundSystem {
    check: HashMap<i32, (String, f64)>,
    time: HashMap<(String, String), (i32, f64)>,
}

impl UndergroundSystem {
    fn new() -> Self {
        Self {
            check: HashMap::new(),
            time: HashMap::new(),
        }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.check.insert(id, (station_name, t as f64));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        if let Some((station, start)) = self.check.get(&id) {
            let key = (station.to_owned(), station_name);
            let diff = t as f64 - *start;
            if let Some((num, time)) = self.time.get(&key) {
                let num = *num;
                let time = *time;
                self.time.insert(key, (num + 1, time + diff));
            } else {
                self.time.insert(key, (1, diff));
            }
        }
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let key = (start_station, end_station);
        let (num, time) = self.time.get(&key).unwrap();
        *time / *num as f64
    }
}

fn main() {
    let mut u = UndergroundSystem::new();
    u.check_in(45, "Leyton".to_string(), 3);
    u.check_in(32, "Paradise".to_string(), 8);
    u.check_in(27, "Leyton".to_string(), 10);

    u.check_out(45, "Waterloo".to_string(), 15);
    u.check_out(27, "Waterloo".to_string(), 20);
    u.check_out(32, "Cambridge".to_string(), 22);

    println!(
        "ret1={}",
        u.get_average_time("Paradise".to_string(), "Cambridge".to_string()),
    );
    println!(
        "ret2={}",
        u.get_average_time("Leyton".to_string(), "Waterloo".to_string()),
    );

    u.check_in(10, "Leyton".to_string(), 24);

    println!(
        "ret3={}",
        u.get_average_time("Leyton".to_string(), "Waterloo".to_string()),
    );

    u.check_out(10, "Waterloo".to_string(), 38);

    println!(
        "ret4={}",
        u.get_average_time("Leyton".to_string(), "Waterloo".to_string()),
    );
}

#[test]
fn test_underground_system() {
    {
        let mut u = UndergroundSystem::new();
        u.check_in(45, "Leyton".to_string(), 3);
        u.check_in(32, "Paradise".to_string(), 8);
        u.check_in(27, "Leyton".to_string(), 10);

        u.check_out(45, "Waterloo".to_string(), 15);
        u.check_out(27, "Waterloo".to_string(), 20);
        u.check_out(32, "Cambridge".to_string(), 22);

        assert_eq!(
            u.get_average_time("Paradise".to_string(), "Cambridge".to_string()),
            14.0
        );
        assert_eq!(
            u.get_average_time("Leyton".to_string(), "Waterloo".to_string()),
            11.0
        );

        u.check_in(10, "Leyton".to_string(), 24);

        assert_eq!(
            u.get_average_time("Leyton".to_string(), "Waterloo".to_string()),
            11.0
        );

        u.check_out(10, "Waterloo".to_string(), 38);

        assert_eq!(
            u.get_average_time("Leyton".to_string(), "Waterloo".to_string()),
            12.0
        );
    }
    {
        let mut u = UndergroundSystem::new();
        u.check_in(10, "Leyton".to_string(), 3);
        u.check_out(10, "Paradise".to_string(), 8);

        assert_eq!(
            u.get_average_time("Leyton".to_string(), "Paradise".to_string()),
            5.0
        );

        u.check_in(5, "Leyton".to_string(), 10);
        u.check_out(5, "Paradise".to_string(), 16);

        assert_eq!(
            u.get_average_time("Leyton".to_string(), "Paradise".to_string()),
            5.5
        );

        u.check_in(2, "Leyton".to_string(), 21);
        u.check_out(2, "Paradise".to_string(), 30);

        assert_eq!(
            u.get_average_time("Leyton".to_string(), "Paradise".to_string()),
            20.0 / 3.0
        );
    }
}
