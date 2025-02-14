struct ProductOfNumbers {
    nums: Vec<i32>,
}

impl ProductOfNumbers {
    fn new() -> Self {
        Self { nums: vec![1] }
    }

    fn add(&mut self, num: i32) {
        if num == 0 {
            self.nums = vec![1];
        } else {
            self.nums.push(self.nums[self.nums.len() - 1] * num);
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        let k = k as usize;
        if k >= self.nums.len() {
            return 0;
        }

        let len = self.nums.len();
        self.nums[len - 1] / self.nums[len - 1 - k]
    }
}

fn main() {
    let mut pn = ProductOfNumbers::new();
    pn.add(3);
    pn.add(0);
    pn.add(2);
    pn.add(5);
    pn.add(4);
    pn.add(8);
    let ret = pn.get_product(2);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let mut pn = ProductOfNumbers::new();
        pn.add(3);
        pn.add(0);
        pn.add(2);
        pn.add(5);
        pn.add(4);
        assert_eq!(pn.get_product(2), 20);
        assert_eq!(pn.get_product(3), 40);
        assert_eq!(pn.get_product(4), 0);
        pn.add(8);
        assert_eq!(pn.get_product(2), 32);
    }
}
