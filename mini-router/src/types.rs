use std::{borrow::Borrow, collections::HashMap};

use fast_paths::{deserialize_32, serialize_32, FastGraph};
use kdtree::KdTree;
use serde::{Deserialize, Serialize};

pub trait GetById {
    fn get_node(&self, id: u64) -> Option<&Node>;
    fn get_way(&self, id: u64) -> Option<&Way>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub id: u64,
    pub lat: f32,
    pub lon: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceWay {
    pub id: u64,
    pub node_ids: Vec<u64>,
    pub one_way: bool,
    pub name: Option<String>,
    pub max_speed: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Way {
    pub id: u64,
    pub node_ids: Vec<u64>,
    pub one_way: bool,
    pub name: Option<String>,
    pub len_km: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SourceFile {
    pub nodes: Vec<Node>,
    pub ways: Vec<SourceWay>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TOSMFile {
    pub nodes: Vec<Node>,
    pub ways: Vec<Way>,

    #[serde(serialize_with = "serialize_32", deserialize_with = "deserialize_32")]
    pub graph: FastGraph,

    pub node_indexes: HashMap<u64, usize>,
    pub way_indexes: HashMap<u64, usize>,

    pub kd_tree: KdTree<f32, u64, [f32; 2]>,
}

impl GetById for TOSMFile {
    fn get_node(&self, id: u64) -> Option<&Node> {
        let idx = self.node_indexes.get(&id)?;
        Some(self.nodes[idx.to_owned()].borrow())
    }
    fn get_way(&self, id: u64) -> Option<&Way> {
        let idx = self.way_indexes.get(&id)?;
        Some(self.ways[idx.to_owned()].borrow())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoutingResult {
    pub distance_km: f32,
    pub time_ms: u64,
    pub node_coords: Vec<(f32, f32)>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BatchRoutingResult {
    pub distance_km: f32,
    pub time_ms: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BatchRoutingResultWrapper {
    pub id: Option<String>,
    pub result: Option<BatchRoutingResult>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BatchRoutingResults {
    pub start_coords: (f32, f32),
    pub destinations: Vec<BatchRoutingResultWrapper>,
}
