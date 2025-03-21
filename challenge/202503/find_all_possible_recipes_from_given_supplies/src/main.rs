fn find_all_recipes(
    recipes: Vec<String>,
    ingredients: Vec<Vec<String>>,
    supplies: Vec<String>,
) -> Vec<String> {
    use std::collections::HashSet;

    let mut supplies: HashSet<_> = supplies.into_iter().collect();

    let mut ret = HashSet::new();
    loop {
        let old = ret.len();
        for (recipe, ingredient) in recipes.iter().zip(ingredients.iter()) {
            if ret.contains(recipe) {
                continue;
            }

            if ingredient.iter().all(|g| supplies.contains(g)) {
                supplies.insert(recipe.clone());
                ret.insert(recipe.clone());
            }
        }

        if old == ret.len() {
            break;
        }
    }

    ret.into_iter().collect()
}

fn main() {
    let recipes = vec![
        "bread".to_string(),
        "sandwich".to_string(),
        "burger".to_string(),
    ];
    let ingredients = vec![
        vec!["yeast".to_string(), "flour".to_string()],
        vec!["bread".to_string(), "meat".to_string()],
        vec![
            "sandwich".to_string(),
            "meat".to_string(),
            "bread".to_string(),
        ],
    ];
    let supplies = vec!["yeast".to_string(), "flour".to_string(), "corn".to_string()];
    let ret = find_all_recipes(recipes, ingredients, supplies);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let recipes = vec!["bread".to_string()];
        let ingredients = vec![vec!["yeast".to_string(), "flour".to_string()]];
        let supplies = vec!["yeast".to_string(), "flour".to_string(), "corn".to_string()];
        let expected = vec!["bread".to_string()];
        let ret = find_all_recipes(recipes, ingredients, supplies);
        assert_eq!(ret, expected);
    }
    {
        let recipes = vec!["bread".to_string(), "sandwich".to_string()];
        let ingredients = vec![
            vec!["yeast".to_string(), "flour".to_string()],
            vec!["bread".to_string(), "meat".to_string()],
        ];
        let supplies = vec!["yeast".to_string(), "flour".to_string(), "meat".to_string()];
        let expected = vec!["bread".to_string(), "sandwich".to_string()];
        let ret = find_all_recipes(recipes, ingredients, supplies);
        assert_eq!(ret, expected);
    }
    {
        let recipes = vec![
            "bread".to_string(),
            "sandwich".to_string(),
            "burger".to_string(),
        ];
        let ingredients = vec![
            vec!["yeast".to_string(), "flour".to_string()],
            vec!["bread".to_string(), "meat".to_string()],
            vec![
                "sandwich".to_string(),
                "meat".to_string(),
                "bread".to_string(),
            ],
        ];
        let supplies = vec!["yeast".to_string(), "flour".to_string(), "meat".to_string()];
        let expected = vec![
            "bread".to_string(),
            "sandwich".to_string(),
            "burger".to_string(),
        ];
        let ret = find_all_recipes(recipes, ingredients, supplies);
        assert_eq!(ret, expected);
    }
}
