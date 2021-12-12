use minesweeper_logic::{CellState, Game, GameState};
use nannou::prelude::*;

struct Model {
    game: Game,
    bomb_texture: wgpu::Texture,
    flag_texture: wgpu::Texture,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn map_range(from_range: (f32, f32), to_range: (f32, f32), s: f32) -> f32 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    match model.game.state {
        GameState::LOST => {}
        GameState::WON => {}
        GameState::INPROGRESS => (),
    }
}

fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    let window = app.window_rect();
    let row = usize::try_from(
        map_range(
            (window.w() / 2., window.w() / -2.),
            (0.0, 10.0),
            app.mouse.y,
        )
        .floor() as u32,
    )
    .unwrap();
    let col = usize::try_from(
        map_range(
            (window.h() / -2., window.h() / 2.),
            (0.0, 10.0),
            app.mouse.x,
        )
        .floor() as u32,
    )
    .unwrap();
    match button {
        MouseButton::Left => model.game.grid.expose_cell(row, col),
        MouseButton::Right => model.game.grid.toggle_seal(row, col),
        _ => (),
    }
    model.game.update_game_state();
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(500, 500)
        .view(view)
        .mouse_pressed(mouse_pressed)
        .build()
        .unwrap();

    let game = Game::new();

    let assets = app.assets_path().unwrap();
    let bomb_image_path = assets.join("mine.png");
    let flag_image_path = assets.join("flag.png");

    let bomb_texture = wgpu::Texture::from_path(app, bomb_image_path).unwrap();
    let flag_texture = wgpu::Texture::from_path(app, flag_image_path).unwrap();

    Model {
        game,
        bomb_texture,
        flag_texture,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(WHITE);

    let window_rect = app.window_rect();
    // translate orign from center to top left corner
    let draw = app
        .draw()
        .x_y(window_rect.w() * -0.5, window_rect.h() * 0.5);
    let mut col_count = 0u8;
    let mut row_count = 0u8;

    for cells in model.game.grid.cells {
        for cell in cells {
            // shapes are draw from center so offset x,y by half width,height
            let shift_x = 50.0 * f32::try_from(col_count).unwrap() + 25.0;
            let shift_y = -50.0 * f32::try_from(row_count).unwrap() - 25.0;
            match cell.state {
                CellState::HIDDEN => {
                    draw.rect()
                        .x(shift_x)
                        .y(shift_y)
                        .wh(vec2(50.0, 50.0))
                        .stroke(BLACK)
                        .stroke_weight(4.0);
                    draw.rect()
                        .x(shift_x)
                        .y(shift_y)
                        .wh(vec2(30.0, 30.0))
                        .stroke(BLACK)
                        .stroke_weight(4.0);
                }
                CellState::EXPOSED => {
                    if cell.bombed {
                        draw.texture(&model.bomb_texture)
                            .x(shift_x)
                            .y(shift_y)
                            .wh(vec2(50.0, 50.0));
                    } else if cell.value > 0 {
                        // draw number
                        draw.rect()
                            .x(shift_x)
                            .y(shift_y)
                            .wh(vec2(50.0, 50.0))
                            .stroke(BLACK)
                            .stroke_weight(4.0);
                        draw.text(&cell.value.to_string())
                            .x(shift_x)
                            .y(shift_y)
                            .color(BLACK)
                            .font_size(16);
                    } else {
                        draw.rect()
                            .x(shift_x)
                            .y(shift_y)
                            .wh(vec2(50.0, 50.0))
                            .stroke(BLACK)
                            .stroke_weight(4.0);
                    }
                }
                CellState::SEALED => {
                    draw.texture(&model.flag_texture)
                        .x(shift_x)
                        .y(shift_y)
                        .wh(vec2(50.0, 50.0));
                }
            }
            col_count += 1;
        }
        row_count += 1;
        col_count = 0;
    }

    match model.game.state {
        GameState::LOST => {
            draw.text("YOU LOST")
                .x(window_rect.w() / 2.0)
                .y(window_rect.h() / -2.0)
                .color(RED)
                .font_size(60);
        }
        GameState::WON => {
            draw.text("YOU WON!")
                .x(window_rect.w() / 2.0)
                .y(window_rect.h() / -2.0)
                .color(GREEN)
                .font_size(60);
        }
        GameState::INPROGRESS => (),
    }

    draw.to_frame(app, &frame).unwrap();
}
