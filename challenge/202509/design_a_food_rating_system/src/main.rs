use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug)]
struct FoodRatings {
    cuisines: HashMap<String, String>,
    rating: HashMap<String, i32>,
    scores: HashMap<String, BinaryHeap<(i32, Reverse<String>)>>,
}

impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut cs = HashMap::new();
        let mut rating = HashMap::new();
        let mut scores = HashMap::new();
        for i in 0..foods.len() {
            cs.insert(foods[i].clone(), cuisines[i].clone());
            rating.insert(foods[i].clone(), ratings[i]);
            scores
                .entry(cuisines[i].clone())
                .or_insert(BinaryHeap::new())
                .push((ratings[i], Reverse(foods[i].clone())));
        }

        Self {
            cuisines: cs,
            rating,
            scores,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        self.rating.insert(food.clone(), new_rating);

        let cuisine = self.cuisines.get(&food).unwrap();
        let q = self.scores.get_mut(cuisine).unwrap();
        q.push((new_rating, Reverse(food)));
    }

    fn highest_rated(&mut self, cuisine: String) -> String {
        let q = self.scores.get_mut(&cuisine).unwrap();

        while let Some((score, Reverse(food))) = q.peek() {
            if let Some(current_score) = self.rating.get(food)
                && *score == *current_score
            {
                return food.clone();
            }
            q.pop();
        }

        unreachable!("never reach here");
    }
}

fn main() {
    let foods = vec![
        "kimchi".to_string(),
        "miso".to_string(),
        "sushi".to_string(),
        "moussaka".to_string(),
        "ramen".to_string(),
        "bulgogi".to_string(),
    ];
    let cuisines = vec![
        "korean".to_string(),
        "japanese".to_string(),
        "japanese".to_string(),
        "greek".to_string(),
        "japanese".to_string(),
        "korean".to_string(),
    ];
    let ratings = vec![9, 12, 8, 15, 14, 7];
    let mut f = FoodRatings::new(foods, cuisines, ratings);
    assert_eq!(f.highest_rated("korean".to_string()), "kimchi");
    assert_eq!(f.highest_rated("japanese".to_string()), "ramen");
    f.change_rating("sushi".to_string(), 16);
    assert_eq!(f.highest_rated("japanese".to_string()), "sushi");
    f.change_rating("ramen".to_string(), 16);
    assert_eq!(f.highest_rated("japanese".to_string()), "ramen");
}

#[test]
fn test() {
    {
        let foods = vec![
            "emgqdbo".to_string(),
            "jmvfxjohq".to_string(),
            "qnvseohnoe".to_string(),
            "yhptazyko".to_string(),
            "ocqmvmwjq".to_string(),
        ];
        let cuisines = vec![
            "snaxol".to_string(),
            "snaxol".to_string(),
            "snaxol".to_string(),
            "fajbervsj".to_string(),
            "fajbervsj".to_string(),
        ];
        let ratings = vec![2, 6, 18, 6, 5];
        let mut f = FoodRatings::new(foods, cuisines, ratings);
        f.change_rating("qnvseohnoe".to_string(), 11);
        assert_eq!(f.highest_rated("fajbervsj".to_string()), "yhptazyko");
        f.change_rating("emgqdbo".to_string(), 3);
        f.change_rating("jmvfxjohq".to_string(), 9);
        f.change_rating("emgqdbo".to_string(), 14);
        dbg!(&f);
        assert_eq!(f.highest_rated("fajbervsj".to_string()), "yhptazyko");
        assert_eq!(f.highest_rated("snaxol".to_string()), "emgqdbo");
    }
    {
        let foods = vec![
            "kimchi".to_string(),
            "miso".to_string(),
            "sushi".to_string(),
            "moussaka".to_string(),
            "ramen".to_string(),
            "bulgogi".to_string(),
        ];
        let cuisines = vec![
            "korean".to_string(),
            "japanese".to_string(),
            "japanese".to_string(),
            "greek".to_string(),
            "japanese".to_string(),
            "korean".to_string(),
        ];
        let ratings = vec![9, 12, 8, 15, 14, 7];
        let mut f = FoodRatings::new(foods, cuisines, ratings);
        assert_eq!(f.highest_rated("korean".to_string()), "kimchi");
        assert_eq!(f.highest_rated("japanese".to_string()), "ramen");
        f.change_rating("sushi".to_string(), 16);
        assert_eq!(f.highest_rated("japanese".to_string()), "sushi");
        f.change_rating("ramen".to_string(), 16);
        assert_eq!(f.highest_rated("japanese".to_string()), "ramen");
    }
}
