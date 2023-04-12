#![feature(const_fn_floating_point_arithmetic)]

use std::f64::consts::PI;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[allow(unused_imports)]
use web_sys::console;

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();

    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let t1 = RobinsonTriangle {
        triangle_type: RobinsonTriangleType::ThinLeft,
        apex: (300.0, 350.0),
        leg_length: 100.0,
        rotation: PI / 4.0,
    };

    t1.draw(&ctx);
}

enum RobinsonTriangleType {
    ThinLeft,
    ThinRight,
    ThickLeft,
    ThickRight,
}

struct RobinsonTriangle {
    triangle_type: RobinsonTriangleType,
    apex: (f64, f64),
    leg_length: f64,
    rotation: f64, // angle of the altitude relative to the X-axis; [0, 2*PI)
}

// see https://en.wikipedia.org/wiki/Penrose_tiling#Rhombus_tiling_(P3)
impl RobinsonTriangle {
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

    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.begin_path();

        let [base_point_1, base_point_2] = self.base_points();

        ctx.move_to(self.apex.0, self.apex.1);
        ctx.line_to(base_point_1.0, base_point_1.1);
        ctx.line_to(base_point_2.0, base_point_2.1);
        ctx.line_to(self.apex.0, self.apex.1);

        ctx.stroke();
    }
}
