use blob_raylib::{
    Color, begin_drawing, clear_background, draw_text, end_drawing, init_window, set_target_fps, window_should_close,
};

fn main() {
    let screen_width = 800;
    let screen_height = 450;

    init_window(screen_width, screen_height, "raylib [core] example - basic window");
    set_target_fps(60);

    while !window_should_close() {
        begin_drawing();
        clear_background(Color::RAYWHITE);
        draw_text(
            "Congrats! You created your first window!",
            190,
            200,
            20,
            Color::LIGHTGRAY,
        );
        end_drawing();
    }
}
