extern crate serde;
extern crate serde_derive;
extern crate serde_json;
#[macro_use]

pub mod food_lib;
pub mod engine;

use self::serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(PartialEq, Clone, Copy)]
pub enum Unit {
    Liters(f64),
    Grams(f64),
    Ammount(f64),
    Nothing,
}

pub static GRAMS: Unit = Unit::Grams(0.0);
pub static LITERS: Unit = Unit::Liters(0.0);
pub static AMMOUNT: Unit = Unit::Ammount(0.0);
pub static NOTHING: Unit = Unit::Nothing;

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
            Unit::Ammount(_) => Unit::Ammount(ammount),
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

    pub fn to_food(&self, ammount: Unit, on_own: bool) -> Food {
        Food::from_ingredient(self.clone(), ammount, on_own)
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

    pub fn add_component(&self, food: &Food) -> Recipe {
        let food = food.clone();
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
            on_own: true,
        }
    }
}

#[derive(Clone)]
pub struct Food {
    food: FoodType,
    ammount: Unit,
    on_own: bool,
}

impl Food {
    pub fn new(food_type: FoodType, ammount: Unit, own: bool) -> Food {
        Food {
            food: food_type,
            ammount: ammount,
            on_own: own,
        }
    }

    pub fn from_ingredient(ing: Ingredient, ammount: Unit, own: bool) -> Food {
        Food::new(FoodType::Ingredient(ing), ammount, own)
    }

    pub fn from_recipe(rec: Recipe, ammount: Unit) -> Food {
        Food::new(FoodType::Recipe(rec), ammount, true)
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

    pub fn to_dish(&self, time: i32) -> Dish {
        Dish {
            name: self.get_name(),
            prep_time: self.get_prep_time(),
            steps: self.get_steps(),
            contains: self.get_contains(),
            time: time,
        }
    }

    pub fn on_own(&self) -> bool {
        self.on_own
    }

    pub fn is_recipe(&self) -> bool {
        match self.food {
            FoodType::Ingredient(_) => false,
            FoodType::Recipe(_) => true,
        }
    }


    pub fn get_recipe(&self) -> Option<Recipe> {
        let food = &self.food;
        if let FoodType::Recipe(x) = food.clone() {
            Some(x.clone())
        } else
        {
            None
        }
    }

    pub fn breakdown(&self) -> Vec<Food> {
        let mut vec = Vec::new();
        vec.push(self.clone());
        
        if self.is_recipe() {
            let contents = self.get_recipe().unwrap().components;
            for x in contents {
                let mut newx = x.clone();
                vec.append(&mut newx.breakdown());
            }
        } else {

        }

        vec
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

impl PartialEq for Food {
    fn eq(&self, other: &Food) -> bool {
        self.get_name() == other.get_name()
    }
}

#[derive(Clone)]
pub struct Dish {
    name: String,
    prep_time: i32,
    steps: Vec<String>,
    contains: Vec<Food>,
    time: i32,
}

impl Serialize for Dish {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Dish", 5)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("preptime", &self.prep_time)?;
        s.serialize_field("time", &self.time)?;
        s.serialize_field("steps", &self.steps)?;
        s.serialize_field("ingredients", &self.contains)?;
        s.end()
    }
}
