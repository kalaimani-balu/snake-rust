mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;


use crate::game::Game;
use crate::draw::to_coordinate_u32;


const BACK_COLOR: Color = [0.50, 0.50, 0.50, 1.0];

fn main() {
    let (width, height) = (30, 30);

    let title = "Snake";

    let size = [
        to_coordinate_u32(width),
        to_coordinate_u32(height)
    ];

    let mut window: PistonWindow =
        WindowSettings::new(title, size)
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g| {
            clear(BACK_COLOR, g);
            game.draw(c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
