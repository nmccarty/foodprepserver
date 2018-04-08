extern crate serde;
extern crate serde_derive;
extern crate serde_json;
#[macro_use]

use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(PartialEq, Clone)]
pub enum Unit {
    Liters(f64),
    Grams(f64),
    Nothing,
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

#[derive(PartialEq, Clone)]
pub struct Ingredient {
    name: String,
    proto_unit: Unit,
}

impl Ingredient {
    pub fn new(name: &str, proto: Unit) -> Ingredient {
        let name = name.to_string();
        Ingredient {
            name: name,
            proto_unit: proto.zero(),
        }
    }
}

#[derive(Clone)]
pub enum FoodType {
    Ingredient(Ingredient),
    Recipe(Recipe),
}

impl FoodType {
    pub fn new_ingredient(ingredient: Ingredient) -> FoodType {
        FoodType::Ingredient(ingredient)
    }
    pub fn new_reicpe(recipe: Recipe) -> FoodType {
        FoodType::Recipe(recipe)
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
    pub fn new(name: &str) -> Recipe {
        let name = name.to_string();
        Recipe {
            components: Vec::new(),
            steps: Vec::new(),
            produces: Unit::Nothing,
            name: name,
            takes: 0,
        }
    }

    pub fn add_component(&self, food: Food) -> Recipe {
        let mut new_rec = self.clone();
        new_rec.components.push(food);
        new_rec
    }

    pub fn add_step(&self, step: &str) -> Recipe {
        let step = step.to_string();
        let mut rec = self.clone();
        rec.steps.push(step);
        rec
    }

    pub fn set_produces(&self, unit: Unit) -> Recipe {
        let mut rec = self.clone();
        rec.produces = unit;
        rec
    }

    pub fn set_takes(&self, time: i32) -> Recipe {
        let mut rec = self.clone();
        rec.takes = time;
        rec
    }

    pub fn make_recipe(&self) -> Food {
        Food {
            food: FoodType::Recipe(self.clone()),
            ammount: self.produces.clone(),
        }
    }
}

#[derive(Clone)]
pub struct Food {
    food: FoodType,
    ammount: Unit,
}

impl Food {
    pub fn new(food_type: FoodType, ammount: Unit) -> Food {
        Food {
            food: food_type,
            ammount: ammount,
        }
    }

    pub fn from_ingredient(ing: Ingredient, ammount: Unit) -> Food {
        Food::new(FoodType::Ingredient(ing), ammount)
    }

    pub fn from_recipe(rec: Recipe, ammount: Unit) -> Food {
        Food::new(FoodType::Recipe(rec), ammount)
    }

    pub fn get_name(&self) -> String {
        match self.food {
            FoodType::Ingredient(ref x) => x.name.clone(),
            FoodType::Recipe(ref x) => x.name.clone(),
        }
    }

    pub fn get_prep_time(&self) -> i32 {
        match self.food {
            FoodType::Ingredient(_) => 0,
            FoodType::Recipe(ref x) => x.takes,
        }
    }

    pub fn get_steps(&self) -> Vec<String> {
        match self.food {
            FoodType::Ingredient(_) => Vec::new(),
            FoodType::Recipe(ref x) => x.steps.clone(),
        }
    }

    pub fn get_contains(&self) -> Vec<Food> {
        match self.food {
            FoodType::Ingredient(_) => Vec::new(),
            FoodType::Recipe(ref x) => x.components.clone(),
        }
    }

    pub fn to_dish(&self) -> Dish {
        Dish {
            name: self.get_name(),
            prep_time: self.get_prep_time(),
            steps: self.get_steps(),
            contains: self.get_contains(),
        }
    }
}

impl Serialize for Food {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Food", 1)?;
        let name = self.get_name();
        s.serialize_field("name", &name)?;
        s.end()
    }
}

#[derive(Clone)]
pub struct Dish {
    name: String,
    prep_time: i32,
    steps: Vec<String>,
    contains: Vec<Food>,
}

impl Serialize for Dish {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let num = 9;
        let mut s = serializer.serialize_struct("Dish", 5)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("preptime", &self.prep_time)?;
        s.serialize_field("time", &num)?;
        s.serialize_field("steps", &self.steps)?;
        s.serialize_field("ingredients", &self.contains)?;
        s.end()
    }
}
