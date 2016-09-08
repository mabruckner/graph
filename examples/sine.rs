extern crate graph;

use std::f32;
use graph::GridPrint;

fn main() {
    let mut g = graph::Graph::hist(100, 40, Box::new(|x| {
        0.5 + (x / 3.0)
    }));
    let mut v1 = Vec::new();
    for i in 0..100 {
        v1.push((i as f32 / 5.0).sin());
    }
    g.set_data(v1);
    g.print();
}
