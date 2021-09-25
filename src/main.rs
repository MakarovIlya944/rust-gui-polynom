mod polynom;
extern crate piston_window;
use piston_window::*;
use std::path::Path;
use polynom::structs::*;

fn main() {
    let title = "Test";
    let width:f64 = 600.0;
    let height:f64 = 400.0;
    let dimensions = [width, height];

    let rx = 10.0;
    let ry = 10.0;
    let px = 4.0;
    let py = 4.0;
    let textx = 0.0;
    let texty = 20.0;
    let text_font_size = 16;

    let mut cursor = [0.0, 0.0];
    let mut main_line:Vec<[f64; 2]> = Vec::new();
    let mut main_points:Vec<[f64; 2]> = Vec::new();

    let mut line_view_type:LineViewType = LineViewType::Straight;
    
    let path = Path::new("C:/Windows/Fonts/Arial.ttf");

    let mut window: PistonWindow = WindowSettings::new(
            title,
            dimensions
        )
        .exit_on_esc(true)
        .build()
        .unwrap();
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
            ellipse([1., 0., 0., 0.2],
                [cursor[0] - rx / 2.0, cursor[1] - ry / 2.0, rx, ry],
                c.transform, g);

            draw_straight(&main_line, c.transform, g);
            let l = main_points.len();
            if l > 0 {
                for i in 1..=l {
                    let _i = i - 1;
                    ellipse([1., 0., 1., 1.],
                        [main_points[_i][0] - px / 2.0, main_points[_i][1] - py / 2.0, px, py],
                        c.transform, g);
                }
            }

            let transform = c.transform.trans(textx, texty);
            text::Text::new_color([0.8, 0.2, 1.0, 1.0], text_font_size).draw(
                castEnum(&line_view_type),
                &mut glyphs,
                &c.draw_state,
                transform, g
            ).unwrap();

            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });
    }
}

fn castEnum(view_type: &LineViewType) -> &'static str {
    match view_type {
        LineViewType::Straight => return "Straight",
        LineViewType::Lagrange => return "Lagrange",
    }
}

fn draw_straight<G: Graphics>(points: &Vec<[f64; 2]>, transform: [[f64; 3]; 2], g: &mut G) {
    let l = points.len();
    if l > 1 {
        for _i in 1..=l-1 {
            line([0.0, 1., 0.0, 1.0],
                1.,
                [points[_i-1][0], points[_i-1][1], points[_i][0], points[_i][1]],
                transform, g);
        }
    }
}