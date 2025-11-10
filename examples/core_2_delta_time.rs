use blob_raylib::{
    Color, KeyboardKey, Vector2, begin_drawing, clear_background, draw_circle_v, draw_text, end_drawing, get_fps,
    get_frame_time, get_mouse_wheel_move, init_window, is_key_pressed, set_target_fps, window_should_close,
};

fn main() {
    let screen_width = 800;
    let screen_height = 450;

    init_window(screen_width, screen_height, "raylib [core] example - delta time");

    let mut current_fps = 60;

    let mut delta_circle = Vector2 {
        x: 0f32,
        y: screen_height as f32 / 3f32,
    };
    let mut frame_circle = Vector2 {
        x: 0f32,
        y: screen_height as f32 * (2f32 / 3f32),
    };

    let speed = 10f32;
    let circle_radius = 32f32;

    set_target_fps(current_fps);

    while !window_should_close() {
        let mouse_wheel = get_mouse_wheel_move();
        if mouse_wheel.abs() > 0f32 {
            current_fps += mouse_wheel as i32;
            if current_fps < 0 {
                current_fps = 0;
            }
            set_target_fps(current_fps);
        }

        delta_circle.x += get_frame_time() * 6f32 * speed;
        frame_circle.x += 0.1f32 * speed;

        if delta_circle.x > screen_width as f32 {
            delta_circle.x = 0f32
        }
        if frame_circle.x > screen_width as f32 {
            frame_circle.x = 0f32
        }

        if is_key_pressed(KeyboardKey::R) {
            delta_circle.x = 0f32;
            frame_circle.x = 0f32;
        }

        begin_drawing();
        clear_background(Color::RAYWHITE);

        draw_circle_v(delta_circle, circle_radius, Color::RED);
        draw_circle_v(frame_circle, circle_radius, Color::BLUE);

        let fps_text;
        if current_fps <= 0 {
            fps_text = format!("FPS: unlimited {}", get_fps());
        } else {
            fps_text = format!("FPS: {} (target: {})", get_fps(), current_fps);
        }

        draw_text(fps_text.as_str(), 10, 10, 20, Color::DARKGRAY);
        draw_text(
            format!("Frame time: {:.3} ms", get_frame_time()).as_str(),
            10,
            30,
            20,
            Color::DARKGRAY,
        );
        draw_text(
            "Use the scroll wheel to change the fps limit, r to reset",
            10,
            50,
            20,
            Color::DARKGRAY,
        );

        draw_text("FUNC: x += GetFrameTime()*speed", 10, 90, 20, Color::RED);
        draw_text("FUNC: x += speed", 10, 240, 20, Color::BLUE);

        end_drawing();
    }
}
