pub mod food;

extern crate hyper;
#[macro_use]
extern crate rustful;
use rustful::{Context, Handler, Response, Server, TreeRouter};
use hyper::header::{AccessControlAllowOrigin, Headers};

struct FoodPlan(&'static str);

impl Handler for FoodPlan {
    fn handle_request(&self, context: Context, response: Response) {
        let mut response = response;
        let resp = "[
	[{
		\"name\": \"Spaghet\",
		\"preptime\": 30,
		\"steps\": [\"Cook noodles\", \"sauce them up\", \"eat\"]
	}],
	[{
		\"name\": \"Spaghet\",
		\"preptime\": 30,
		\"steps\": [\"Cook noodles\", \"sauce them up\", \"eat\"]
	}],
	[{
		\"name\": \"Spaghet\",
		\"preptime\": 30,
		\"steps\": [\"Cook noodles\",
			\"sauce them up\",
			\"eat\"
		]
	}],
	[{
		\"name\": \"Spaghet\",
		\"preptime\": 30,
		\"steps\": [\"Cook noodles\",
			\"sauce them up\",
			\"eat\"
		]
	}],
	[{
		\"name\": \"Spaghet\",
		\"preptime\": 30,
		\"steps\": [\"Cook noodles\",
			\"sauce them up\",
			\"eat\"
		]
	}],
	[{
		\"name\": \"Spaghet\",
		\"preptime\": 30,
		\"steps\": [\"Cook noodles\",
			\"sauce them up\",
			\"eat\"
		]
	}],
	[{
		\"name\": \"Spaghet\",
		\"preptime\": 30,
		\"steps\": [\"Cook noodles\",
			\"sauce them up\",
			\"eat\"
		]
	}]
]";

        addACA(&mut response);
        response.send(resp);
    }
}

fn addACA(response: &mut Response){
    response.headers_mut().set(AccessControlAllowOrigin::Any);
}

fn main() {
    let my_router = insert_routes!{
        //Create a new TreeRouter
        TreeRouter::new() => {
            "foodplan" => {
                Get: FoodPlan("yes"),
                Post: FoodPlan("yes"),
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
