use blob_raylib::{
    Color, begin_drawing, clear_background, draw_rectangle, draw_text, end_drawing, get_mouse_wheel_move, init_window,
    set_target_fps, window_should_close,
};

fn main() {
    let screen_width = 800;
    let screen_height = 450;

    init_window(screen_width, screen_height, "raylib [core] example - input mouse wheel");
    set_target_fps(60);

    let mut box_position_y = screen_height / 2 - 40;
    let scroll_speed = 4;

    while !window_should_close() {
        begin_drawing();
        clear_background(Color::RAY_WHITE);

        box_position_y -= get_mouse_wheel_move() as i32 * scroll_speed;

        draw_rectangle(screen_width / 2 - 40, box_position_y, 80, 80, Color::MAROON);

        draw_text("Use mouse wheel to move the cube up and down!", 10, 10, 20, Color::GRAY);
        draw_text(
            format!("Box position Y: {:.3}", box_position_y).as_str(),
            10,
            40,
            20,
            Color::LIGHT_GRAY,
        );

        end_drawing();
    }
}
