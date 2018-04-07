pub enum Unit {
    Liters(f64),
    Grams(f64),
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
        }
    }
}

pub struct Ingredient {
    name: String,
    proto_unit: Unit,
}

pub struct Recipe {
    components: Vec<Food>,
    steps: Vec<String>,
    produces: Unit,
    name: String,
    // Time, in minutes
    takes: i32,
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
