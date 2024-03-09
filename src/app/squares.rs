// Recreate https://openprocessing.org/sketch/821016

use nannou::{
    color::rgb::Rgb,
    ease::{map, quad::ease_in_out},
    prelude::*,
};
use once_cell::sync::Lazy;

use crate::utils::RandomStepRange;

const PAPER_DENSITY: f32 = 10.0 / 100.0;
pub const WIDTH: f32 = 800.0;
pub const HEIGHT: f32 = 800.0;
static PALETTE: Lazy<Vec<Rgb8>> = Lazy::new(|| {
    vec![
        (0xED, 0xF7, 0xF5),
        (0xB7, 0xD7, 0xD8),
        (0xFF, 0x89, 0x84),
        (0x20, 0x4E, 0x5F),
        (0xFF, 0xC6, 0xA8),
    ]
    .iter()
    .map(|&c| Rgb8::new(c.0, c.1, c.2))
    .collect()
});

pub struct Model {}

impl Default for Model {
    fn default() -> Self {
        Self {}
    }
}

pub fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::rate_fps(0.5));
    Model::default()
}

pub fn update(_app: &App, _model: &mut Model, _update: Update) {}

pub fn view(app: &App, _model: &Model, frame: Frame) {
    if app.elapsed_frames() % 60 != 0 {
        return;
    }
    frame.clear(GRAY);
    let draw = app.draw().translate(vec3(-WIDTH / 2.0, -HEIGHT / 2.0, 0.0));

    draw_paper_texture(app, &draw);
    draw_squares(app, &draw);

    draw.to_frame(app, &frame).unwrap();
}

pub fn draw_paper_texture(app: &App, draw: &Draw) {
    (0..(WIDTH * HEIGHT * PAPER_DENSITY) as u32)
        .map(|_| {
            let x = random_range(0.0, WIDTH);
            let y = random_range(0.0, HEIGHT);
            let weight = random_range(1.0, 3.0);
            (x, y, weight)
        })
        .for_each(|(x, y, weight)| {
            draw.ellipse().x_y(x, y).radius(weight).rgba8(0, 0, 0, 5);
        });
}

pub fn draw_squares(app: &App, draw: &Draw) {
    let min_margin = random_range(1, 10) as f32 / 10.0;
    // let cells = random_range(8, 15);
    let cells = 8;
    let cols = cells;
    let rows = cells;

    let offset = WIDTH / 20.0;
    let margin = if random_f32() > 0.5 {
        offset as f32 / 5.0
    } else {
        0.0
    };
    let w = (WIDTH - offset as f32 * 2.0 - margin * (cols as f32 - 1.0)) / cols as f32;
    let h = (HEIGHT - offset as f32 * 2.0 - margin * (rows as f32 - 1.0)) / rows as f32;

    let random_margin = || {
        if 1.0 - min_margin == min_margin {
            0.0
        } else {
            random_range(min_margin, 1.0 - min_margin)
        }
    };

    let j_range: Vec<_> = RandomStepRange::new(0, rows, 4).collect();
    for (j, j_step) in j_range {
        let ch = h * j_step as f32 + margin * (j_step as f32 - 1.0);

        for (i, i_step) in RandomStepRange::new(0, cols, 3) {
            let x = map_range(
                i as f32,
                0.0,
                cols as f32 - 1.0,
                offset as f32,
                WIDTH - offset as f32 - w,
            );
            let y = map_range(
                j as f32,
                0.0,
                rows as f32 - 1.0,
                offset as f32,
                HEIGHT - offset as f32 - h,
            );

            let cw = w * i_step as f32 + margin * (i_step as f32 - 1.0);
            draw.rect()
                .x_y(x + cw / 2.0, y + ch / 2.0)
                .w_h(cw, ch)
                .rgb8(255, 255, 255)
                .stroke(BLACK)
                .stroke_weight(1.0);

            let points = [
                Point2::new(x, y),
                Point2::new(x + cw, y),
                Point2::new(x + cw, y + ch),
                Point2::new(x, y + ch),
            ];
            draw_separate_rects(draw, &random_margin, &points, 3);
        }
    }
}

fn lerp_points(a: &Point2, b: &Point2, t: f32) -> Point2 {
    let x = ease_in_out(t, a.x, b.x, 1.0);
    let y = ease_in_out(t, a.y, b.y, 1.0);
    pt2(x, y)
}

fn draw_separate_rects<F>(draw: &Draw, random_margin: &F, points: &[Point2], depth: i32)
where
    F: Fn() -> f32,
{
    if depth < 0 {
        return;
    }
    let n = random_range(0, points.len() / 2);
    let m = n + points.len() / 2;
    let n_f = random_margin();
    let m_f = random_margin();
    let n_next = (n + 1) % points.len();
    let m_next = (m + 1) % points.len();
    let p_n = points.get(n).unwrap();
    let p_n_next = points.get(n_next).unwrap();
    let p_m = points.get(m).unwrap();
    let p_m_next = points.get(m_next).unwrap();
    let p3 = lerp_points(p_n, p_n_next, n_f);
    let p4 = lerp_points(p_m, p_m_next, m_f);

    draw.polygon()
        .color(PALETTE[random_range(0, PALETTE.len())])
        .stroke(BLACK)
        .stroke_weight(1.0)
        .join_round()
        .points([p3, p4, *p_m_next, *p_n])
        .finish();

    draw.polygon()
        .color(PALETTE[random_range(0, PALETTE.len())])
        .stroke(BLACK)
        .stroke_weight(1.0)
        .join_round()
        .points([p4, p3, *p_n_next, *p_m])
        .finish();

    let points_a = [p3, p4, *p_m_next, *p_n];
    draw_separate_rects(draw, random_margin, &points_a, depth - 1);
    let points_b = [p4, p4, *p_n_next, *p_m];
    draw_separate_rects(draw, random_margin, &points_b, depth - 1);
}
