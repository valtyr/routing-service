use crate::{
    distance::dist_haversine,
    types::{
        BatchRoutingResult, BatchRoutingResultWrapper, BatchRoutingResults, GetById, Node,
        RoutingResult, TOSMFile,
    },
    util::pairwise,
};

pub fn nearest_node<'a>(file: &'a TOSMFile, coords: &'a [f32; 2]) -> Option<&'a Node> {
    let result = file.kd_tree.nearest(coords, 1, &dist_haversine);
    match result {
        Ok(vec) => {
            let (_, id) = vec.first()?;
            Some(file.get_node(**id)?)
        }
        _ => None,
    }
}

pub fn find_route(file: &TOSMFile, start_node: &Node, end_node: &Node) -> Option<RoutingResult> {
    let start_idx = file.node_indexes.get(&start_node.id)?;
    let end_idx = file.node_indexes.get(&end_node.id)?;

    let shortest_path = fast_paths::calc_path(&file.graph, *start_idx, *end_idx)?;

    let time_ms = shortest_path.get_weight() as u64;

    let nodes = shortest_path.get_nodes();

    let distance_km = pairwise(nodes).fold(0_f32, |acc, (a_option, b)| match a_option {
        Some(a) => {
            let node_a = &file.nodes[*a];
            let node_b = &file.nodes[*b];

            let dist = dist_haversine(&[node_a.lat, node_a.lon], &[node_b.lat, node_b.lon]);

            acc + dist
        }
        _ => acc,
    });

    let node_coords = nodes
        .iter()
        .map(|node_idx| {
            let node = &file.nodes[*node_idx];
            (node.lat, node.lon)
        })
        .collect();

    Some(RoutingResult {
        time_ms,
        node_coords,
        distance_km,
    })
}

pub fn find_routes(
    file: &TOSMFile,
    start_node: &Node,
    destinations: Vec<(&Node, Option<String>)>,
) -> BatchRoutingResults {
    let mut path_calculator = fast_paths::create_calculator(&file.graph);

    // Should be safe?
    let start_idx = file.node_indexes.get(&start_node.id).unwrap();

    let results: Vec<BatchRoutingResultWrapper> = destinations
        .iter()
        .map(|(node, id)| -> BatchRoutingResultWrapper {
            let end_idx = match file.node_indexes.get(&node.id) {
                Some(idx) => idx,
                None => {
                    return BatchRoutingResultWrapper {
                        id: id.to_owned(),
                        result: None,
                    }
                }
            };
            let shortest_path = match path_calculator.calc_path(&file.graph, *start_idx, *end_idx) {
                Some(path) => path,
                _ => {
                    return BatchRoutingResultWrapper {
                        id: id.to_owned(),
                        result: None,
                    }
                }
            };

            let time_ms = shortest_path.get_weight() as u64;

            let nodes = shortest_path.get_nodes();

            let distance_km = pairwise(nodes).fold(0_f32, |acc, (a_option, b)| match a_option {
                Some(a) => {
                    let node_a = &file.nodes[*a];
                    let node_b = &file.nodes[*b];

                    let dist = dist_haversine(&[node_a.lat, node_a.lon], &[node_b.lat, node_b.lon]);

                    acc + dist
                }
                _ => acc,
            });

            BatchRoutingResultWrapper {
                id: id.to_owned(),
                result: Some(BatchRoutingResult {
                    distance_km,
                    time_ms,
                }),
            }
        })
        .collect();

    return BatchRoutingResults {
        start_coords: (start_node.lat, start_node.lon),
        destinations: results,
    };
}
