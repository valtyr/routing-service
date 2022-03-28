use indicatif::ProgressBar;
use std::collections::HashMap;

use fast_paths::InputGraph;
use kdtree::KdTree;
use mini_router::{
    distance::dist_haversine,
    types::{Node, SourceFile, TOSMFile, Way},
    util::pairwise,
};

pub fn generate_file(path: &str) -> TOSMFile {
    let mut nodes: Vec<Node> = vec![];
    let ways: Vec<Way> = vec![];
    let mut node_indexes: HashMap<u64, usize> = HashMap::new();
    let way_indexes: HashMap<u64, usize> = HashMap::new();
    let mut kd_tree: KdTree<f32, u64, [f32; 2]> = KdTree::new(2);

    let mut input_graph = InputGraph::new();

    {
        println!("📚\tReading source file");
        let source = std::fs::read_to_string(path).unwrap();

        println!("🤖\tDeserializing source file");
        let v: SourceFile = serde_json::from_str(&source).unwrap();

        println!("⚙️\tProcessing nodes and ways");
        let pb = ProgressBar::new((v.nodes.len() + v.ways.len()) as u64);

        for node in v.nodes {
            nodes.push(node.clone());
            node_indexes.insert(node.id, nodes.len() - 1);
            kd_tree.add([node.lat, node.lon], node.id).unwrap();
            pb.inc(1);
        }

        for way in v.ways {
            let mut new_way = Way {
                id: way.id,
                node_ids: way.node_ids,
                one_way: way.one_way,
                name: way.name,
                len_km: 0.0,
            };

            for (a_option, b) in pairwise(&new_way.node_ids) {
                match a_option {
                    Some(a) => {
                        let a_idx = node_indexes.get(a).unwrap();
                        let b_idx = node_indexes.get(b).unwrap();

                        let a_node = &nodes[*a_idx];
                        let b_node = &nodes[*b_idx];

                        let dist =
                            dist_haversine(&[a_node.lat, a_node.lon], &[b_node.lat, b_node.lon]);

                        let ms = ((dist / way.max_speed) * 3600000.0) as i64;

                        if ms < 0 {
                            println!("{}", ms);
                        }

                        if new_way.one_way {
                            input_graph.add_edge(*a_idx, *b_idx, ms as usize);
                        } else {
                            input_graph.add_edge_bidir(*a_idx, *b_idx, ms as usize);
                        }

                        new_way.len_km += dist;
                    }
                    _ => {}
                }
            }
            pb.inc(1);
        }
    }

    println!("🕸\tPreparing graph");
    input_graph.freeze();
    let fast_graph = fast_paths::prepare(&input_graph);

    let file = TOSMFile {
        nodes,
        ways,
        node_indexes,
        way_indexes,
        kd_tree,
        graph: fast_graph,
    };

    file
}
