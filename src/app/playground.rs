use nannou::prelude::*;

use crate::utils::lerp_points;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 800;

pub struct Model {
    pub percentage: f32,
}

impl Default for Model {
    fn default() -> Self {
        Self { percentage: 0.0 }
    }
}

pub fn model(app: &App) -> Model {
    Model::default()
}

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    model.percentage = model.percentage + 0.01
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    // .translate(vec3(-(WIDTH as f32) / 2.0, -(HEIGHT as f32) / 2.0, 0.0));

    let points = [
        pt2(0.0, 0.0),
        pt2(100.0, 0.0),
        pt2(100.0, 100.0),
        pt2(0.0, 100.0),
    ];
    draw.polygon().points(points).color(RED);
    let mid1 = lerp_points(&points[0], &points[1], model.percentage.sin().abs());
    let mid2 = lerp_points(&points[2], &points[3], model.percentage.sin().abs());
    draw.ellipse().xy(mid1).radius(5.0).color(WHITE);
    draw.ellipse().xy(mid2).radius(5.0).color(GREEN);

    let new_points = [points[0], mid1, points[1], points[2]];
    draw.polygon()
        .stroke(BLACK)
        .stroke_weight(2.0)
        .no_fill()
        .points(new_points);
    new_points.iter().for_each(|p| {
        draw.ellipse().xy(*p).radius(5.0).color(BLUE);
    });

    let new_points = [points[2], mid2, points[3], points[0]];
    draw.polygon()
        .stroke(BLACK)
        .stroke_weight(2.0)
        .no_fill()
        .points(new_points);

    new_points.iter().for_each(|p| {
        draw.ellipse().xy(*p).radius(5.0).color(YELLOW);
    });
    draw.to_frame(app, &frame).unwrap();
}
