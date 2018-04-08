pub mod food;
extern crate hyper;
#[macro_use]
extern crate rustful;
extern crate serde;
extern crate serde_json;
use rustful::{Context, Handler, Response, Server, TreeRouter};
use hyper::header::{AccessControlAllowOrigin, Headers};

use food::*;

struct FoodPlan(Vec<Vec<Dish>>);

impl Handler for FoodPlan {
    fn handle_request(&self, context: Context, response: Response) {
        let mut response = response;

        let resp = serde_json::to_string(&self.0).ok().unwrap();

        addACA(&mut response);
        response.send(resp);
    }
}

fn addACA(response: &mut Response) {
    response.headers_mut().set(AccessControlAllowOrigin::Any);
}

fn main() {
    let nudes = Ingredient::new("noodles", Unit::new_grams(0.0));
    let nudes = Food::from_ingredient(nudes, Unit::Nothing);
    let sauce = Ingredient::new("sauce", Unit::new_liters(0.0));
    let sauce = Food::from_ingredient(sauce, Unit::Nothing);
    let alfredo = Ingredient::new("alfredo", Unit::Nothing);
    let alfredo = Food::from_ingredient(alfredo, Unit::Nothing);

    let spa = Recipe::new("Spaghet")
        .add_component(nudes.clone())
        .add_component(sauce.clone())
        .add_step("Cook noodles")
        .add_step("Apply sauce to noodles")
        .add_step("Enjoy")
        .set_takes(30);

    let spa_dish = Food::from_recipe(spa, Unit::Nothing).to_dish();

    let alfred = Recipe::new("Alfred")
        .add_component(nudes.clone())
        .add_component(alfredo.clone())
        .add_step("Cook noodles (al dente)")
        .add_step("Apply alfredo sauce to noodles")
        .add_step("Enjoy alfredoly")
        .set_takes(45);

    let alfred_dish = Food::from_recipe(alfred, Unit::Nothing).to_dish();

    let mut meals = Vec::new();

    for x in 0..7 {
        let mut vec = Vec::new();
        if x % 2 == 0 {
            vec.push(spa_dish.clone());
        } else {
            vec.push(alfred_dish.clone());
        }

        meals.push(vec);
    }

    let my_router = insert_routes!{
        //Create a new TreeRouter
        TreeRouter::new() => {
            "foodplan" => {
                Get: FoodPlan(meals.clone()),
                Post: FoodPlan(meals.clone()),
            }
        }
    };

    Server {
        //Use a closure to handle requests.
        handlers: my_router,
        //Set the listening port to `8080`.
        host: 8080.into(),
        //Fill out everything else with default values.
        ..Server::default()
    }.run();
}
