#![feature(const_fn_floating_point_arithmetic)]

use std::f64::consts::PI;
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[allow(unused_imports)]
use web_sys::console;

#[derive(Serialize, Deserialize)]
enum RobinsonTriangleType {
    ThinLeft,
    ThinRight,
    ThickLeft,
    ThickRight,
}


#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
pub struct RobinsonTriangle {
    triangle_type: RobinsonTriangleType,
    apex: (f64, f64),
    leg_length: f64,
    rotation: f64, // angle of the altitude relative to the X-axis; [0, 2*PI)
}

#[wasm_bindgen]
impl RobinsonTriangle {
    pub fn new() -> RobinsonTriangle {
        RobinsonTriangle {
            triangle_type: RobinsonTriangleType::ThinLeft,
            apex: (300.0, 350.0),
            leg_length: 100.0,
            rotation: PI / 4.0,
        }
    }

    // see https://en.wikipedia.org/wiki/Penrose_tiling#Rhombus_tiling_(P3)
    const fn vertex_angle(&self) -> f64 {
        match self.triangle_type {
            RobinsonTriangleType::ThinLeft | RobinsonTriangleType::ThinRight => 0.1 * PI, // 36 degrees
            RobinsonTriangleType::ThickLeft | RobinsonTriangleType::ThickRight => 0.2 * PI, // 72 degrees
        }
    }

    const fn base_angle(&self) -> f64 {
        match self.triangle_type {
            RobinsonTriangleType::ThinLeft | RobinsonTriangleType::ThinRight => 0.4 * PI, // 72 degrees
            RobinsonTriangleType::ThickLeft | RobinsonTriangleType::ThickRight => 0.3 * PI, // 54 degrees
        }
    }

    fn base_points(&self) -> [(f64, f64); 2] {
        let mut theta_one = self.rotation + self.vertex_angle() / 2.0;

        if theta_one >= 2.0 * PI {
            theta_one -= 2.0 * PI;
        }

        let mut theta_two = self.rotation - self.vertex_angle() / 2.0;

        if theta_two < 0.0 {
            theta_two += 2.0 * PI;
        }

        // reflect Ys across x=apex.x since positive Y is down in graphics coordinates
        let base_point_1 = (
            self.apex.0 + self.leg_length * theta_one.cos(),
            self.apex.1 - self.leg_length * theta_one.sin(),
        );

        let base_point_2 = (
            self.apex.0 + self.leg_length * theta_two.cos(),
            self.apex.1 - self.leg_length * theta_two.sin(),
        );

        [base_point_1, base_point_2]
    }

    pub fn decompose(&self) -> JsValue {
        let new_point: (f64, f64) = match self.triangle_type {
            RobinsonTriangleType::ThinLeft => todo!(),
            RobinsonTriangleType::ThinRight => todo!(),
            RobinsonTriangleType::ThickLeft => todo!(),
            RobinsonTriangleType::ThickRight => todo!(),
        };

        let base_points = self.base_points();

        let triangle_one = match self.triangle_type {
            RobinsonTriangleType::ThinLeft => RobinsonTriangle {
                triangle_type: RobinsonTriangleType::ThickRight,
                apex: new_point,
                leg_length: ((new_point.1 - self.apex.1).powf(2.0) + (new_point.0 - self.apex.0).powf(2.0)).sqrt(),
                rotation: todo!(),
            },
            RobinsonTriangleType::ThinRight => RobinsonTriangle {
                triangle_type: RobinsonTriangleType::ThickLeft,
                apex: new_point,
                leg_length: ((new_point.1 - self.apex.1).powf(2.0) + (new_point.0 - self.apex.0).powf(2.0)).sqrt(),
                rotation: todo!(),
            },
            RobinsonTriangleType::ThickLeft => RobinsonTriangle {
                triangle_type: RobinsonTriangleType::ThinRight,
                apex: base_points[0],
                leg_length: self.leg_length,
                rotation: todo!(),
            },
            RobinsonTriangleType::ThickRight => RobinsonTriangle {
                triangle_type: RobinsonTriangleType::ThinLeft,
                apex: base_points[1],
                leg_length: self.leg_length,
                rotation: todo!(),
            }
        };

        let triangle_two = match self.triangle_type {
            RobinsonTriangleType::ThinLeft => RobinsonTriangle {
                triangle_type: RobinsonTriangleType::ThinLeft,
                apex: base_points[0],
                leg_length: (base_points[0].1 - base_points[1].1).abs(),
                rotation: todo!(),
            },
            RobinsonTriangleType::ThinRight => RobinsonTriangle {
                triangle_type: RobinsonTriangleType::ThinRight,
                apex: base_points[1],
                leg_length: (base_points[0].1 - base_points[1].1).abs(),
                rotation: todo!(),
            },
            RobinsonTriangleType::ThickLeft => RobinsonTriangle {
                triangle_type: RobinsonTriangleType::ThickLeft,
                apex: new_point,
                leg_length: (base_points[1].0 - new_point.0).abs(),
                rotation: todo!(),
            },
            RobinsonTriangleType::ThickRight => RobinsonTriangle {
                triangle_type: RobinsonTriangleType::ThickRight,
                apex: new_point,
                leg_length: (base_points[1].0 - new_point.0).abs(),
                rotation: todo!(),
            }
        };

        serde_wasm_bindgen::to_value(&[triangle_one, triangle_two]).unwrap()
    }
}
