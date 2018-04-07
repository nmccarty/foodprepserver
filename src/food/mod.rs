#[derive(PartialEq,Clone)]
pub enum Unit {
    Liters(f64),
    Grams(f64),
    Nothing
}

impl Unit {
    pub fn new_liters(ammount: f64) -> Unit {
        Unit::Liters(ammount)
    }

    pub fn new_grams(ammount: f64) -> Unit {
        Unit::Grams(ammount)
    }

    pub fn zero(&self) -> Unit {
        self.from_zero(0.0)
    }

    pub fn from_zero(&self, ammount: f64) -> Unit {
        match *self {
            Unit::Liters(_) => Unit::Liters(ammount),
            Unit::Grams(_) => Unit::Grams(ammount),
            Unit::Nothing => Unit::Nothing,
        }
    }
}

#[derive(PartialEq,Clone)]
pub struct Ingredient {
    name: String,
    proto_unit: Unit,
}

impl Ingredient {
    pub fn new(name: String, proto: Unit) -> Ingredient{
        Ingredient {
            name: name,
            proto_unit: proto.zero(),
        }
    }
}

#[derive(Clone)]
pub enum FoodType {
    Ingredient(Ingredient),
    Reicpe(Recipe),
}

impl FoodType {
    pub fn new_ingredient(ingredient: Ingredient) -> FoodType {
        FoodType::Ingredient(ingredient)
    }
    pub fn new_reicpe(recipe: Recipe) -> FoodType {
        FoodType::Reicpe(recipe)
    }
}

#[derive(Clone)]
pub struct Recipe {
    components: Vec<Food>,
    steps: Vec<String>,
    produces: Unit,
    name: String,
    // Time, in minutes
    takes: i32,
}

impl Recipe {
    pub fn new_empty() -> Recipe {
        Recipe {
            components: Vec::new(),
            steps: Vec::new(),
            produces: Unit::Nothing,
            name: String::new(),
            takes: 0,
        }
    }
}

#[derive(Clone)]
pub struct Food {
    food: FoodType,
    ammount: Unit,
}

pub struct Dish {
    food: Food,
    ammount: Unit,
}

pub struct PrettyDish {
    name: String,
    time: i32,
    prep_time: i32,
    steps: Vec<String>,
    makes: Unit,
    contains: Vec<Food>,
}
