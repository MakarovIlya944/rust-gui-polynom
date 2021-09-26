mod polynom;
extern crate piston_window;
use piston_window::*;
use std::path::Path;
use polynom::structs::*;


static LINE_COLOR:[f32; 4] = [0.0, 1., 0.0, 1.0];
static LINE_RADIUS:f64 = 1.;
static WIDTH:f64 = 600.;
static HEIGHT:f64 = 300.;


fn main() {
    let title = "Test";
    let dimensions = [WIDTH, HEIGHT];

    let rx = 10.0;
    let ry = 10.0;
    let px = 4.0;
    let py = 4.0;
    let gu = 50.0;
    let gx = WIDTH / gu;
    let gy = HEIGHT / gu;
    let textx = 0.0;
    let texty = 20.0;
    let text_font_size = 16;
    let ox = HEIGHT / 2.;
    

    let mut cursor = [0.0, 0.0];
    let mut main_line:Vec<[f64; 2]> = Vec::new();
    let mut main_points:Vec<[f64; 2]> = Vec::new();

    let mut line_view_type:LineViewType = LineViewType::Straight;
    
    let mut window: PistonWindow = WindowSettings::new(
        title,
        dimensions
    )
    .exit_on_esc(true)
    .build()
    .unwrap();
    let path = Path::new("C:/Windows/Fonts/Arial.ttf");
    let mut glyphs = window.load_font(path).unwrap();
    window.set_lazy(true);
    while let Some(e) = window.next() {
        e.mouse_cursor(|pos| {
            cursor = pos;
        });
        if let Some(Button::Mouse(button)) = e.press_args() {
            println!("Pressed mouse button '{:?}'", button);
        };
        if let Some(button) = e.release_args() {
            match button {
                Button::Keyboard(key) => {
                    println!("Released keyboard key '{:?}'", key);
                    if key == Key::D1 {
                        line_view_type = LineViewType::Straight;
                    }
                    else if key == Key::D2 {
                        line_view_type = LineViewType::Lagrange;
                    }
                },
                Button::Mouse(button) => {
                    println!("Released mouse button '{:?}'", button);
                    if button == MouseButton::Left {
                        main_points.push(cursor);
                        main_line.push(cursor);
                        main_points.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());
                        main_line.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());
                    }
                    else if button == MouseButton::Right {
                        main_points.pop();
                        main_line.pop();
                    }
                },
                Button::Controller(button) => println!("Released controller button '{:?}'", button),
                Button::Hat(hat) => println!("Released controller hat `{:?}`", hat),
            }
        };

        window.draw_2d(&e, |c, g, device| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            // Draw grid and supply
            let grid = grid::Grid{cols:gx as u32, rows:gy as u32, units:gu};
            let grid_line = Line::new([0.2,0.3,0.2,0.6],1.);
            grid.draw(&grid_line,&c.draw_state,c.transform,g);
            line([1.0, 0., 0.0, 1.0],
                1.,
                [0., ox, WIDTH, ox],
                c.transform, g);

            // Cursor pointer
            ellipse([1., 0., 0., 0.2],
                [cursor[0] - rx / 2.0, cursor[1] - ry / 2.0, rx, ry],
                c.transform, g);

            // Draw main line
            match line_view_type {
                LineViewType::Straight => draw_straight(&main_line, c.transform, g),
                LineViewType::Lagrange => draw_lagrange(&main_line, c.transform, g),
            }

            // Draw nodes of line
            let l = main_points.len();
            if l > 0 {
                for i in 1..=l {
                    let _i = i - 1;
                    ellipse([1., 0., 1., 1.],
                        [main_points[_i][0] - px / 2.0, main_points[_i][1] - py / 2.0, px, py],
                        c.transform, g);
                }
            }

            // Supply text info
            let transform = c.transform.trans(textx, texty);
            text::Text::new_color([0.4, 0.4, 0.7, 1.0], text_font_size).draw(
                cast_enum(&line_view_type),
                &mut glyphs,
                &c.draw_state,
                transform, g
            ).unwrap();

            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });
    }
}

fn cast_enum(view_type: &LineViewType) -> &'static str {
    match view_type {
        LineViewType::Straight => return "Straight",
        LineViewType::Lagrange => return "Lagrange",
    }
}

fn draw_straight<G: Graphics>(points: &Vec<[f64; 2]>, transform: [[f64; 3]; 2], g: &mut G) {
    let l = points.len();
    if l > 1 {
        for _i in 1..=l-1 {
            line(LINE_COLOR,
                LINE_RADIUS,
                [points[_i-1][0], points[_i-1][1], points[_i][0], points[_i][1]],
                transform, g);
        }
    }
}

fn draw_lagrange<G: Graphics>(points: &Vec<[f64; 2]>, transform: [[f64; 3]; 2], g: &mut G) {
    let dx = 2.0;
    let l = points.len();
    if l > 1 {
        let right = points[l-1][0];
        let mut x = points[0][0];
        let mut cur_line = [x, points[0][1], x, points[0][1]];
        while x < right {
            cur_line[0] = cur_line[2];
            cur_line[1] = cur_line[3];
            let y = lagrange_func(points, x);
            cur_line[2] = x;
            cur_line[3] = y;
            line(LINE_COLOR,LINE_RADIUS, cur_line,transform, g);
            x += dx;
        }
    }
}

fn lagrange_func(points: &Vec<[f64; 2]>, x:f64) -> f64 {
    let mut result = 0.;
    for _i in 1..=points.len() {
        let i = _i - 1;
        result += points[i][1] * lagrange_basis_func(points, x, i);
    }
    return result;
}

fn lagrange_basis_func(points: &Vec<[f64; 2]>, x:f64, i:usize) -> f64 {
    let mut result = 1.;
    for _i in 1..=points.len() {
        let j = _i - 1;
        if j != i {
            result *= (x - points[j][0]) / (points[i][0] - points[j][0]);
        }
    }
    return result;
}
