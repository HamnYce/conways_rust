use nannou::prelude::*;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;
const ROW_C: u32 = 20;
const COL_C: u32 = 20;
const ROW_U: u32 = HEIGHT / ROW_C;
const COL_U: u32 = WIDTH / COL_C;

// TODO: change sketch to app and add model
//  inside model: matrix of boolean to represent alive or dead state

fn main() {
    nannou::sketch(view)
        .size(WIDTH , HEIGHT)
        .run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();


    draw_grid(&draw, &win);

    draw.background().color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}


fn key_pressed(_app: &App, _model: &mut Model, _key: Key) {}


fn draw_grid(draw: &Draw, win: &Rect) {
    for x in ((win.x.start as i32)..(win.x.end as i32)).step_by(COL_U as usize)
    {
        draw.line()
            .start(pt2(x as f32, win.y.start))
            .end(pt2(x as f32, win.y.end))
            .color(WHITE);

    }
    for y in ((win.y.start as i32)..(win.y.end as i32)).step_by(ROW_U as usize)
    {
        draw.line()
            .start(pt2(win.x.end, y as f32))
            .end(pt2(win.x.start, y as f32))
            .color(WHITE);

    }
}