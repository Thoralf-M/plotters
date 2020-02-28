use crate::DrawResult;
use plotters::prelude::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wbg_rand::{wasm_rng, Rng};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Copy, Clone)]
pub struct Vertex<'a> {
    hash: &'a str,
    trunk: &'a str,
    branch: &'a str,
    pos: (f32, f32),
}

/// Draw power function f(x) = x^power.
pub fn draw(
    canvas_id: &str,
    value: JsValue,
    interations: usize,
) -> DrawResult<impl Fn((i32, i32)) -> Option<(f32, f32)>> {
    let hashes: HashMap<String, String> = serde_wasm_bindgen::from_value(value)?;

    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    let font: FontDesc = ("sans-serif", 20.0).into();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(format!("Txs:{:#?}", hashes.len()), font)
        .build_ranged(-5f32..hashes.len() as f32, -30.2f32..25.2f32)?;

    // chart.configure_mesh().x_labels(3).y_labels(3).draw()?;

    let mut tangle = Vec::new();
    //custom points to test
    // tangle.push(Vertex {
    //     hash: "AA",
    //     trunk: "99",
    //     branch: "99",
    //     pos: (0f32, 0f32),
    // });
    // tangle.push(Vertex {
    //     hash: "BB",
    //     trunk: "AA",
    //     branch: "99",
    //     pos: (1f32, 1f32),
    // });
    // tangle.push(Vertex {
    //     hash: "CC",
    //     trunk: "AA",
    //     branch: "BB",
    //     pos: (3f32, 1f32),
    // });
    // tangle.push(Vertex {
    //     hash: "DD",
    //     trunk: "AA",
    //     branch: "CC",
    //     pos: (2f32, 5f32),
    // });
    // tangle.push(Vertex {
    //     hash: "EE",
    //     trunk: "DD",
    //     branch: "BB",
    //     pos: (1f32, 7f32),
    // });

    //data from zmq
    for txdata in hashes.iter() {
        let txhash = &txdata.0;
        let trunk = &txdata.1[0..81];
        let branch = &txdata.1[81..162];
        let tangleindex = txdata.1[162..172].parse::<f32>().unwrap();
        // log(&format!("Tx:{}", &txdata.0.to_string()));
        let num: usize = wasm_rng().gen_range(5, 15);
        // let num: usize = 10;
        // nodes.push((0.1 * tangleindex as f32, num as f32));
        tangle.push(Vertex {
            hash: txhash,
            trunk: trunk,
            branch: branch,
            pos: (tangleindex as f32, num as f32),
        });
    }


    //calulacte new positions
    for i in 0..interations {
        for (index, tx) in tangle.clone().into_iter().enumerate() {
            let trunk = tangle.clone().into_iter().find(|ver| ver.hash == tx.trunk);
            let branch = tangle.clone().into_iter().find(|ver| ver.hash == tx.branch);
            let modification_strength = 0.4;
            match trunk {
                Some(trunk) => {
                    let trunk_distance = ((tx.pos.0 - trunk.pos.0).powi(2)
                        + (tx.pos.1 - trunk.pos.1).powi(2))
                    .sqrt();
                    // log(&format!("{:?}", &trunk_distance));
                    if trunk_distance > 1.0 {
                        if tx.pos.0 > trunk.pos.0 {
                            let v0 =
                                tx.pos.0 - modification_strength * (tx.pos.0 - trunk.pos.0).abs();
                            let v1 =
                                tx.pos.1 - modification_strength * (tx.pos.1 - trunk.pos.1).abs();
                            tangle[index].pos.0 = v0;
                            tangle[index].pos.1 = v1;
                        } else {
                            let v0 =
                                tx.pos.0 + (modification_strength*0.5) * (tx.pos.0 - trunk.pos.0).abs();
                            let v1 =
                                tx.pos.1 + (modification_strength*0.5) * (tx.pos.1 - trunk.pos.1).abs();
                            tangle[index].pos.0 = v0;
                            tangle[index].pos.1 = v1;
                        }
                    }
                }
                // _ => log(&format!("Referenced tx not found: {:?}", tx.trunk)),
                _ => (),
            }
            
            match branch {
                Some(branch) => {
                    let branch_distance = ((tx.pos.0 - branch.pos.0).powi(2)
                    + (tx.pos.1 - branch.pos.1).powi(2))
                    .sqrt();
                    if branch_distance > 1.0 {
                        if tx.pos.0 > branch.pos.0 {
                            let v0 =
                            tx.pos.0 - modification_strength * (tx.pos.0 - branch.pos.0).abs();
                            let v1 =
                            tx.pos.1 - modification_strength * (tx.pos.1 - branch.pos.1).abs();
                            tangle[index].pos.0 = v0;
                            tangle[index].pos.1 = v1;
                        } else {
                            let v0 =
                            tx.pos.0 + (modification_strength*0.5) * (tx.pos.0 - branch.pos.0).abs();
                            let v1 =
                            tx.pos.1 + (modification_strength*0.5) * (tx.pos.1 - branch.pos.1).abs();
                            tangle[index].pos.0 = v0;
                            tangle[index].pos.1 = v1;
                        }
                    }
                }
                // _ => log(&format!("Referenced tx not found: {:?}", tx.trunk)),
                _ => (),
            }

            // let new_tx_pos = tangle[index].pos;
            // tangle.ins(tx);
        }
    }

    //connect vertices
    for tx in tangle.clone().into_iter() {
        // log(&format!("Tx trunk: {:?}", tx.trunk));
        // log(&format!("Tx branch: {:?}", tx.branch));
        let tx_trunk = tangle.clone().into_iter().find(|ver| ver.hash == tx.trunk);
        let tx_branch = tangle.clone().into_iter().find(|ver| ver.hash == tx.branch);
        match tx_trunk {
            Some(v) => {
                // log(&format!("Tx_trunk: {:?}", &v.hash));
                let mut trunk_connection = Vec::new();
                trunk_connection.push(tx.pos);
                trunk_connection.push(v.pos);
                chart.draw_series(LineSeries::new(trunk_connection, &GREEN))?;
            }
            // _ => log(&format!("Referenced tx not found: {:?}", tx.trunk)),
            _ => (),
        }
        match tx_branch {
            Some(v) => {
                // log(&format!("Tx_branch: {:?}", &v.hash));
                let mut branch_connection = Vec::new();
                branch_connection.push(tx.pos);
                branch_connection.push(v.pos);
                chart.draw_series(LineSeries::new(branch_connection, &RED))?;
            }
            // _ => log(&format!("Referenced tx not found: {:?}", tx.branch)),
            _ => (),
        }
    }

    // let dot_and_label = |x: f32, y: f32| {
    //     return EmptyElement::at((x, y))
    //         + Circle::new((0, 0), 3, ShapeStyle::from(&RED).filled());
    // };
    // root.draw(&dot_and_label(0.5, 0.6))?;

    // And we can draw something in the drawing area

    // chart.draw_series(LineSeries::new(
    //     vec![(0.0, 0.0), (5.0, 5.0), (8.0, 7.0)],
    //     &RED,
    // ))?;
    // chart.draw_series(LineSeries::new(
    //     nodes.clone(),
    //     &GREEN,
    // ))?;

    // Similarly, we can draw point series
    chart.draw_series(PointSeries::of_element(
        // vec![(0.0, 0.0), (5.0, 5.0), (8.0, 7.0)],
        tangle,
        5,
        &RED,
        &|c, s, st| {
            return EmptyElement::at(c.pos)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
            + Text::new(format!("{:?}", &c.hash[0..1]), (10, 0), ("sans-serif", 10).into_font());
        },
    ))?;

    root.present()?;
    return Ok(chart.into_coord_trans());
}
