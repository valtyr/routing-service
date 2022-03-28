use mini_router::router;
use mini_router::types::{BatchRoutingResults, Node, RoutingResult};
use mini_router::{storage::read_file, types::TOSMFile};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{routes, State};

#[macro_use]
extern crate rocket;

struct GlobalState {
    routing_data: TOSMFile,
}

#[get("/")]
fn index() -> &'static str {
    "Mini router (ICELAND 🇮🇸)"
}

#[derive(Deserialize)]
struct RoutingOptions {
    from: [f32; 2],
    to: [f32; 2],
}

#[derive(Serialize)]
struct RouteResponse {
    error: Option<String>,
    result: Option<RoutingResult>,
}

#[post("/route", data = "<routing_options>")]
fn route(
    routing_options: Json<RoutingOptions>,
    global_state: &State<GlobalState>,
) -> Json<RouteResponse> {
    let node_a = match router::nearest_node(&global_state.routing_data, &routing_options.from) {
        Some(node) => node,
        _ => {
            return Json(RouteResponse {
                error: Some("Origin not found".to_string()),
                result: None,
            })
        }
    };
    let node_b = match router::nearest_node(&global_state.routing_data, &routing_options.to) {
        Some(node) => node,
        _ => {
            return Json(RouteResponse {
                error: Some("Destination not found".to_string()),
                result: None,
            })
        }
    };

    let route = match router::find_route(&global_state.routing_data, &node_a, &node_b) {
        Some(route) => route,
        _ => {
            return Json(RouteResponse {
                error: Some("No route found".to_string()),
                result: None,
            })
        }
    };

    return Json(RouteResponse {
        error: None,
        result: Some(route),
    });
}

#[derive(Deserialize)]
struct BatchDestination {
    id: Option<String>,
    coords: [f32; 2],
}

#[derive(Deserialize)]
struct BatchOptions {
    from: [f32; 2],
    to: Vec<BatchDestination>,
}

#[derive(Serialize)]
struct BatchResponse {
    error: Option<String>,
    result: Option<BatchRoutingResults>,
}

#[post("/batch", data = "<routing_options>")]
fn batch(
    routing_options: Json<BatchOptions>,
    global_state: &State<GlobalState>,
) -> Json<BatchResponse> {
    let node_a = match router::nearest_node(&global_state.routing_data, &routing_options.from) {
        Some(node) => node,
        _ => {
            return Json(BatchResponse {
                error: Some("Origin not found".to_string()),
                result: None,
            })
        }
    };

    let mut destination_nodes: Vec<(&Node, Option<String>)> = vec![];

    for destination in &routing_options.to {
        match router::nearest_node(&global_state.routing_data, &destination.coords) {
            Some(node) => destination_nodes.push((node, destination.id.to_owned())),
            _ => {
                return Json(BatchResponse {
                    error: Some(
                        format!(
                            "Destination not found ({}, {})[{}]",
                            destination.coords[0],
                            destination.coords[1],
                            destination.id.as_ref().unwrap_or(&" ".to_string())
                        )
                        .to_string(),
                    ),
                    result: None,
                })
            }
        };
    }

    let result = router::find_routes(&global_state.routing_data, &node_a, destination_nodes);

    return Json(BatchResponse {
        error: None,
        result: Some(result),
    });
}

#[launch]
fn rocket() -> _ {
    let routing_data = read_file("data/routing-data.br".to_string());

    rocket::build()
        .manage(GlobalState { routing_data })
        .mount("/", routes![index, route, batch])
}
