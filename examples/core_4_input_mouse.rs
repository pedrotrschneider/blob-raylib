use blob_raylib::{
    BEIGE, DARKBLUE, DARKGRAY, KeyboardKey, LIME, MAROON, MouseButton, ORANGE, PURPLE, RAYWHITE, RED, YELLOW,
    begin_drawing, clear_background, draw_circle_v, draw_text, end_drawing, get_mouse_position, hide_cursor,
    init_window, is_cursor_hidden, is_key_pressed, is_mouse_button_pressed, set_target_fps, show_cursor,
    window_should_close,
};

fn main() {
    let screen_width = 800;
    let screen_height = 450;

    init_window(screen_width, screen_height, "raylib [core] example - input mouse");
    set_target_fps(60);

    let mut ball_position;
    let mut ball_color = DARKBLUE;

    while !window_should_close() {
        if is_key_pressed(KeyboardKey::H) {
            if is_cursor_hidden() {
                show_cursor();
            } else {
                hide_cursor();
            }
        }

        ball_position = get_mouse_position();

        if is_mouse_button_pressed(MouseButton::Left) {
            ball_color = MAROON
        } else if is_mouse_button_pressed(MouseButton::Middle) {
            ball_color = LIME
        } else if is_mouse_button_pressed(MouseButton::Right) {
            ball_color = DARKBLUE
        } else if is_mouse_button_pressed(MouseButton::Side) {
            ball_color = PURPLE
        } else if is_mouse_button_pressed(MouseButton::Extra) {
            ball_color = YELLOW
        } else if is_mouse_button_pressed(MouseButton::Forward) {
            ball_color = ORANGE
        } else if is_mouse_button_pressed(MouseButton::Back) {
            ball_color = BEIGE
        }

        begin_drawing();
        clear_background(RAYWHITE);

        draw_circle_v(ball_position, 40f32, ball_color);

        draw_text(
            "move ball with mouse and click mouse button to change color",
            10,
            10,
            20,
            DARKGRAY,
        );
        draw_text("Press 'H' to toggle cursor visibility", 10, 30, 20, DARKGRAY);

        if is_cursor_hidden() {
            draw_text("CURSOR HIDDEN", 20, 60, 20, RED);
        } else {
            draw_text("CURSOR VISIBLE", 20, 60, 20, LIME);
        }

        end_drawing();
    }
}
