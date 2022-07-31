mod utils;

extern crate serde_json;
extern crate wasm_bindgen;

use forceatlas2::{Coord, Layout, Nodes, Settings};
use serde::{Deserialize, Deserializer, Serialize};
use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// #[macro_use]
extern crate serde_derive;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, forceatlas2-wasm!");
}

// struct SimpleStruct {

// }

// #[derive(Serialize, Deserialize)]
// struct LayoutSettings<T: Coord>(Settings<T>);

// impl<'de> Deserialize<'de> for LayoutSettings<f32> {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         //    #[derive(Deserialize)]
//         //    #[serde(field_identifier, rename_all = "lowercase")]
//         //    enum Field { Secs, Nanos }
//     }
// }

#[wasm_bindgen]
pub fn generate_layout(elements_raw: JsValue, setting_raw: JsValue) -> String {
    let elements: Vec<usize> = elements_raw
        .into_serde()
        .expect("Values should be an array of integer pairs");

    // let setting = setting_raw.into_serde::<LayoutSettings<f32>>().ok();

    // let setting: Settings = match setting_raw.into_serde<Settings>() {
    //     None => None,
    //     Some(raw_value) => raw_value.into_serde().expect("Values should be Settings")
    // };

    // let original_edges = vec![(0, 1), (2, 3), (1, 3)];
    let original_edges = elements
        .iter()
        .enumerate()
        .map(|(a, b)| (a, *b))
        .collect::<Vec<(usize, usize)>>();
    let mut nodes = 0usize;
    let mut edges = Vec::<(usize, usize)>::new();
    for (n1, n2) in original_edges.iter() {
        if *n1 > nodes {
            nodes = *n1;
        }
        if *n2 > nodes {
            nodes = *n2;
        }
        if n1 != n2 {
            edges.push(if n1 < n2 { (*n1, *n2) } else { (*n2, *n1) });
        }
    }
    nodes += 1;

    let mut layout = Layout::<f32>::from_graph(
        edges,
        Nodes::Degree(nodes),
        None,
        Settings {
            #[cfg(feature = "barnes_hut")]
            barnes_hut: None,
            chunk_size: None,
            dimensions: 2,
            dissuade_hubs: false,
            ka: 0.01,
            kg: 0.001,
            kr: 0.002,
            lin_log: false,
            speed: 1.0,
            prevent_overlapping: None,
            strong_gravity: false,
        },
    );

    let ITERATIONS = 100usize;
    for i in 0..ITERATIONS {
        // if ANIM_MODE {
        //     // draw_graph(&layout, i);
        // }
        print!("{}/{}\r", i, ITERATIONS);
        layout.iteration();
    }

    let ret: Vec<(f32, f32)> = layout.points.iter().map(|pos| (pos[0], pos[1])).collect();

    serde_json::to_string(&ret).unwrap()
}
