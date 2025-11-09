use blob_raylib::{
    DARKGRAY, KeyboardKey, MAROON, RAYWHITE, Vector2, begin_drawing, clear_background, draw_circle_v, draw_text,
    end_drawing, init_window, is_key_down, set_target_fps, window_should_close,
};

fn main() {
    let screen_width = 800;
    let screen_height = 450;

    init_window(screen_width, screen_height, "raylib [core] example - input keys");
    set_target_fps(60);

    let mut ball_position = Vector2 {
        x: screen_width as f32 * 0.5f32,
        y: screen_height as f32 * 0.5f32,
    };

    while !window_should_close() {
        if is_key_down(KeyboardKey::Right) {
            ball_position.x += 2f32;
        }
        if is_key_down(KeyboardKey::Left) {
            ball_position.x -= 2f32;
        }
        if is_key_down(KeyboardKey::Down) {
            ball_position.y += 2f32;
        }
        if is_key_down(KeyboardKey::Up) {
            ball_position.y -= 2f32;
        }

        begin_drawing();
        clear_background(RAYWHITE);

        draw_text("move the ball with arrow keys", 10, 10, 20, DARKGRAY);
        draw_circle_v(ball_position, 50f32, MAROON);

        end_drawing();
    }
}
