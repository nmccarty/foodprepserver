pub mod food;
extern crate hyper;
#[macro_use]
extern crate rustful;
extern crate serde;
extern crate serde_json;
use rustful::{Context, Handler, Response, Server, TreeRouter};
use hyper::header::{AccessControlAllowOrigin, Headers};

use food::*;

struct FoodPlan(Dish);

impl Handler for FoodPlan {
    fn handle_request(&self, context: Context, response: Response) {
        let mut response = response;

        let mut day = Vec::new();
        day.push(self.0.clone());

        let mut list = Vec::new();

        for x in 0..7 {
            list.push(day.clone());
        }

        let resp = serde_json::to_string(&list).ok().unwrap();

        addACA(&mut response);
        response.send(resp);
    }
}

fn addACA(response: &mut Response) {
    response.headers_mut().set(AccessControlAllowOrigin::Any);
}

fn main() {
    let nudes = Ingredient::new("noodles".to_string(), Unit::new_grams(0.0));
    let nudes = Food::from_ingredient(nudes, Unit::Nothing);
    let sauce = Ingredient::new("sauce".to_string(), Unit::new_liters(0.0));
    let sauce = Food::from_ingredient(sauce, Unit::Nothing);
    let spa = Recipe::new("Spaghet".to_string())
        .add_component(nudes)
        .add_component(sauce)
        .add_step("Cook noodles".to_string())
        .add_step("Apply sauce to noodles".to_string())
        .add_step("Enjoy".to_string())
        .set_takes(30);

    let spa_dish = Food::from_recipe(spa, Unit::Nothing).to_dish();

    let my_router = insert_routes!{
        //Create a new TreeRouter
        TreeRouter::new() => {
            "foodplan" => {
                Get: FoodPlan(spa_dish.clone()),
                Post: FoodPlan(spa_dish.clone()),
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
