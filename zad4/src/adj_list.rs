use std::mem;

use rand::{
    distributions::Uniform,
    prelude::{Distribution, SliceRandom, SmallRng},
};

use crate::Graph;

type NodeIndex = crate::NodeIndex;
type Weight = crate::Weight;

#[derive(Debug)]
pub struct AdjList {
    max_node: NodeIndex,
    adjs: Vec<(NodeIndex, NodeIndex, Weight)>,
}

impl AdjList {
    pub fn new() -> Self {
        AdjList {
            max_node: 0,
            adjs: Vec::new(),
        }
    }

    pub fn random_connected(num_nodes: usize, edge_probability: f32, rng: &mut SmallRng) -> Self {
        let mut graph = Self::new();
        graph.add_node((num_nodes - 1) as NodeIndex);

        // first we connect all unordered pairs of the graph so that it is connected
        let mut unvisited_set = Vec::new();

        for node in 0..graph.len() {
            unvisited_set.push(node);
        }
        unvisited_set.shuffle(rng);

        let mut cur_vertex = unvisited_set.pop().expect("no nodes in the graph");
        let weight_dist = Uniform::from(1..=20);

        while !unvisited_set.is_empty() {
            let adj_vertex = unvisited_set.pop().unwrap();
            let weight = weight_dist.sample(rng);
            graph.connect(cur_vertex, adj_vertex, weight);
            cur_vertex = adj_vertex;
        }

        graph
    }
}

impl Graph for AdjList {
    fn len(&self) -> NodeIndex {
        self.max_node + 1
    }

    fn add_node(&mut self, node: NodeIndex) {
        if node > self.max_node {
            self.max_node = node;
        }
    }

    fn connect(&mut self, n1: NodeIndex, n2: NodeIndex, weight: i32) {
        if n1 > self.max_node || n2 > self.max_node {
            panic!("node does not exist")
        }

        if self
            .adjs
            .iter_mut()
            .find(|(u, v, _)| *u == n1 && *v == n2)
            .map(|(_, _, w)| *w = weight)
            .is_none()
        {
            self.adjs.push((n1, n2, weight));
        }

        if self
            .adjs
            .iter_mut()
            .find(|(v, u, _)| *u == n1 && *v == n2)
            .map(|(_, _, w)| *w = weight)
            .is_none()
        {
            self.adjs.push((n2, n1, weight));
        }
    }

    fn nodes_connected(&self, n1: NodeIndex, n2: NodeIndex) -> bool {
        self.adjs
            .iter()
            .find(|(u, v, _)| (*u == n1 && *v == n2) || (*u == n2 && *v == n1))
            .is_some()
    }

    fn distance(&self, n1: NodeIndex, n2: NodeIndex) -> i32 {
        self.adjs
            .iter()
            .find(|(u, v, _)| (*u == n1 && *v == n2) || (*u == n2 && *v == n1))
            .unwrap()
            .2
    }

    fn memory(&self) -> usize {
        self.adjs.len() * mem::size_of::<(NodeIndex, NodeIndex, Weight)>()
    }

    fn num_neighbours(&self, _n: crate::NodeIndex) -> usize {
        todo!()
    }

    fn graph_connected(&self) -> bool {
        todo!()
    }

    fn node_neighbours(&self, n: NodeIndex) -> Vec<NodeIndex> {
        self.adjs
            .iter()
            .filter(|(n1, _, _)| n == *n1)
            .map(|(_, n2, _)| *n2)
            .collect()
    }
}
