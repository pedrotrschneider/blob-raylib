fn main() {
    let screen_width = 1920;
    let screen_height = 1080;
    let title = "this is a simple test window";

    blob_raylib::init_window(screen_width, screen_height, title);
    blob_raylib::set_target_fps(60);

    while !blob_raylib::window_should_close() {
        blob_raylib::begin_drawing();
        blob_raylib::clear_background(blob_raylib::RAYWHITE);
        let message = "Hello, Raylib! (No crates)";
        blob_raylib::draw_text(message, 190, 200, 20, blob_raylib::DARKGRAY);
        blob_raylib::end_drawing();
    }

    blob_raylib::close_window();
}
