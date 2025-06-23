use json::JsonValue;

pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub proteins: f64,
    pub fats: f64,
    pub carbs: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> JsonValue {
    let mut total_cals = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;
    let mut total_fats = 0.0;

    for food in foods {
        let kcal_value = if food.calories[1].ends_with("kcal") {
            food.calories[1].trim_end_matches("kcal").parse::<f64>().unwrap()
        } else {
            food.calories[1].trim_end_matches("kcal").parse::<f64>().unwrap()
        };
        
        total_cals += kcal_value * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
    }

    let round = |value: f64| {
        let rounded = (value * 100.0).round() / 100.0;
        if (rounded * 10.0).fract() == 0.0 {
            (rounded * 10.0).round() / 10.0
        } else {
            rounded
        }
    };

    json::object! {
        "cals" => round(total_cals),
        "carbs" => round(total_carbs),
        "proteins" => round(total_proteins),
        "fats" => round(total_fats),
    }
}