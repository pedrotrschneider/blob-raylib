use blob_raylib::{
    Color, begin_drawing, clear_background, close_window, draw_text, end_drawing, init_window, set_target_fps,
    window_should_close,
};

fn main() {
    let screen_width = 1920;
    let screen_height = 1080;
    let title = "this is a simple test window";

    init_window(screen_width, screen_height, title);
    set_target_fps(60);

    while !window_should_close() {
        begin_drawing();
        clear_background(Color::RAY_WHITE);
        let message = "Hello, Raylib! (No crates)";
        draw_text(message, 190, 200, 20, Color::DARK_GRAY);
        end_drawing();
    }

    close_window();
}
