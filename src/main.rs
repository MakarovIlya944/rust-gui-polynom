extern crate piston_window;
use piston_window::*;

fn main() {
    let title = "Test";
    let dimensions = [600,400];

    let mut window: PistonWindow = WindowSettings::new(
            title,
            dimensions
        )
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut cursor = [0.0,0.0];
    window.set_lazy(true);
    while let Some(e) = window.next() {
        e.mouse_cursor(|pos| {
            cursor = pos;
            println!("Mouse moved '{} {}'", pos[0], pos[1]);
        });

        window.draw_2d(&e, |c, g, device| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            line::Line
            
        });
    }
}