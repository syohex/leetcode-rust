use std::collections::HashMap;

struct CheckInData {
    station: String,
    time: i32,
}

struct Summary {
    count: i32,
    total_time: i32,
}

struct UndergroundSystem {
    check_in_table: HashMap<i32, CheckInData>,
    total_table: HashMap<String, Summary>,
}

impl UndergroundSystem {
    fn new() -> Self {
        Self {
            check_in_table: HashMap::new(),
            total_table: HashMap::new(),
        }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.check_in_table.insert(
            id,
            CheckInData {
                station: station_name,
                time: t,
            },
        );
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        if let Some(data) = self.check_in_table.get(&id) {
            let key = format!("{} {}", data.station, station_name);
            self.total_table
                .entry(key)
                .and_modify(|s| {
                    s.count += 1;
                    s.total_time += t - data.time;
                })
                .or_insert(Summary {
                    count: 1,
                    total_time: t - data.time,
                });
        }
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let key = format!("{} {}", start_station, end_station);
        let data = self.total_table.get(&key).unwrap();
        data.total_time as f64 / data.count as f64
    }
}

fn main() {
    let mut s = UndergroundSystem::new();
    s.check_in(45, "Leyton".to_string(), 3);
    s.check_in(32, "Paradise".to_string(), 8);
    s.check_in(27, "Leyton".to_string(), 10);
    s.check_out(45, "Waterloo".to_string(), 15);
    s.check_out(27, "Waterloo".to_string(), 20);
    s.check_out(32, "Cambridge".to_string(), 22);

    let ret = s.get_average_time("Paradise".to_string(), "Cambridge".to_string());
    println!("ret={}", ret);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(a: f64, b: f64) {
        eprintln!("a={}, b={}", a, b);
        assert!((a - b).abs() < 0.00001);
    }

    #[test]
    fn test_underground_system() {
        {
            let mut s = UndergroundSystem::new();
            s.check_in(45, "Leyton".to_string(), 3);
            s.check_in(32, "Paradise".to_string(), 8);
            s.check_in(27, "Leyton".to_string(), 10);
            s.check_out(45, "Waterloo".to_string(), 15);
            s.check_out(27, "Waterloo".to_string(), 20);
            s.check_out(32, "Cambridge".to_string(), 22);

            check(
                s.get_average_time("Paradise".to_string(), "Cambridge".to_string()),
                14.0,
            );
            check(
                s.get_average_time("Leyton".to_string(), "Waterloo".to_string()),
                11.0,
            );

            s.check_in(10, "Leyton".to_string(), 24);
            check(
                s.get_average_time("Leyton".to_string(), "Waterloo".to_string()),
                11.0,
            );
            s.check_out(10, "Waterloo".to_string(), 38);
            check(
                s.get_average_time("Leyton".to_string(), "Waterloo".to_string()),
                12.0,
            );
        }
        {
            let mut s = UndergroundSystem::new();
            s.check_in(10, "Leyton".to_string(), 3);
            s.check_out(10, "Paradise".to_string(), 8);
            check(
                s.get_average_time("Leyton".to_string(), "Paradise".to_string()),
                5.000,
            );
            s.check_in(5, "Leyton".to_string(), 10);
            s.check_out(5, "Paradise".to_string(), 16);
            check(
                s.get_average_time("Leyton".to_string(), "Paradise".to_string()),
                5.500,
            );
            s.check_in(2, "Leyton".to_string(), 21);
            s.check_out(2, "Paradise".to_string(), 30);
            check(
                s.get_average_time("Leyton".to_string(), "Paradise".to_string()),
                6.66667,
            );
        }
    }
}
