use minesweeper_logic::{CellState, Game, GameState};
use nannou::prelude::*;

struct Model {
    game: Game,
    bomb_texture: wgpu::Texture,
    flag_texture: wgpu::Texture
}

fn main() {
    nannou::app(model).run();
}

fn map_range(from_range: (f32, f32), to_range: (f32, f32), s: f32) -> f32 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::R => model.game = Game::new(),
        _ => (),
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
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let assets = app.assets_path().unwrap();

    let bomb_path = assets.join("bomb.png");
    let bomb_texture = wgpu::Texture::from_path(app, bomb_path).unwrap();

    let flag_path = assets.join("flag.png");
    let flag_texture = wgpu::Texture::from_path(app, flag_path).unwrap();

    let game = Game::new();

    Model {
        game,
        bomb_texture,
        flag_texture
    }
}

fn draw_cell_border(draw: &Draw, pos: Vec2, cell_size: f32) {
    draw.rect()
        .xy(pos)
        .wh(vec2(cell_size, cell_size))
        .stroke(Rgb8::new(144, 255, 202))
        .no_fill()
        .stroke_weight(2.0);
}

fn draw_hidden_cell(draw: &Draw, pos: Vec2, cell_size: f32) {
    draw_cell_border(draw, pos, cell_size);
    draw.rect()
        .xy(pos)
        .wh(vec2(cell_size / 2., cell_size / 2.))
        .stroke(Rgb8::new(144, 255, 202))
        .no_fill()
        .stroke_weight(2.0);
}

fn draw_bombed_cell(model: &Model, draw: &Draw, pos: Vec2, cell_size: f32) {
    draw_cell_border(draw, pos, cell_size);
    draw.texture(&model.bomb_texture)
        .xy(pos)
        .wh(vec2(cell_size - 10.0, cell_size - 10.0));
}

fn draw_sealed_cell(model: &Model, draw: &Draw, pos: Vec2, cell_size: f32) {
    draw_cell_border(draw, pos, cell_size);
    draw.texture(&model.flag_texture)
        .xy(pos)
        .wh(vec2(cell_size - 10.0, cell_size - 10.0));
}

fn draw_numbered_cell(draw: &Draw, pos: Vec2, cell_size: f32, value: u8) {
    draw_cell_border(draw, pos, cell_size);
    draw.text(&value.to_string())
        .xy(pos)
        .rgb8(255, 230, 153)
        .font_size(20);
}

fn draw_game_progress(draw: &Draw, window_rect: Rect, model: &Model) {
    let text_pos = vec2(window_rect.w() / 2.0, window_rect.h() / -2.0);
    match model.game.state {
        GameState::LOST => {
            draw.text("YOU LOST").xy(text_pos).color(RED).font_size(60);
        }
        GameState::WON => {
            draw.text("YOU WON!")
                .xy(text_pos)
                .color(GREEN)
                .font_size(60);
        }
        GameState::INPROGRESS => (),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let window_rect = app.window_rect();
    let cell_size = window_rect.w() / model.game.grid.cells.len() as f32;

    // translate origin from center to top left corner
    let draw = app
        .draw()
        .x_y(window_rect.w() * -0.5, window_rect.h() * 0.5);

    let mut col_count = 0u32;
    let mut row_count = 0u32;

    for cells in model.game.grid.cells {
        for cell in cells {
            // shapes are draw from center so offset x,y by half width,height
            let shift_x = cell_size * col_count as f32 + cell_size / 2.;
            let shift_y = -cell_size * row_count as f32 - cell_size / 2.;
            let pos = vec2(shift_x, shift_y);
            match cell.state {
                CellState::HIDDEN => draw_hidden_cell(&draw, pos, cell_size),
                CellState::EXPOSED => {
                    if cell.bombed {
                        draw_bombed_cell(model, &draw, pos, cell_size);
                    } else if cell.value > 0 {
                        draw_numbered_cell(&draw, pos, cell_size, cell.value);
                    } else {
                        draw_cell_border(&draw, pos, cell_size);
                    }
                }
                CellState::SEALED => draw_sealed_cell(model, &draw, pos, cell_size),
            }
            col_count += 1;
        }
        row_count += 1;
        col_count = 0;
    }

    draw_game_progress(&draw, window_rect, model);

    draw.to_frame(app, &frame).unwrap();
}
