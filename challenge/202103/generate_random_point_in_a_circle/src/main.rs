struct Solution {
    radius: f64,
    x_center: f64,
    y_center: f64,
}

impl Solution {
    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self {
            radius,
            x_center,
            y_center,
        }
    }

    fn rand_point(&self) -> Vec<f64> {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        loop {
            let r1: f64 = rng.gen_range(-self.radius, self.radius);
            let r2: f64 = rng.gen_range(-self.radius, self.radius);

            let dist = (r1 * r1 + r2 * r2).sqrt();
            if dist <= self.radius {
                return vec![r1 + self.x_center, r2 + self.y_center];
            }
        }
    }
}

fn main() {
    let s = Solution::new(1.0, 0.0, 0.0);
    println!("ret={:?}", s.rand_point());
}

#[test]
fn test_rand_point() {
    {
        let s = Solution::new(1.0, 0.0, 0.0);
        for _ in 0..1000 {
            let point = s.rand_point();
            let dist = (point[0] * point[0] + point[1] * point[1]).sqrt();
            assert!(dist <= 1.0);
        }
    }
    {
        let s = Solution::new(10.0, 5.0, -7.5);
        for _ in 0..1000 {
            let point = s.rand_point();
            let r1 = point[0] - 5.0;
            let r2 = point[1] + 7.5;
            let dist = (r1 * r1 + r2 * r2).sqrt();
            assert!(dist <= 10.0);
        }
    }
}
