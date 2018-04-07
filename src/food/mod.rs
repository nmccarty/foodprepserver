pub enum Unit {
    Liters(f64),
    Grams(f64),
}

pub struct Ingredient{
    name: String,
    proto_unit: Unit,
}

pub struct Recipe {
    components: Vec<Food>,
    steps: Vec<String>,
    produces: Unit,
}

pub enum FoodType {
    Ingredient(Ingredient),
    Reicpe(Recipe),
}

pub struct Food {
    food: FoodType,
    ammount: Unit,
}

pub struct Dish {
    food: Food,
    ammount: Unit
}
