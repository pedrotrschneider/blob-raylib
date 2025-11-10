use blob_raylib::{
    BLACK, DARKGRAY, ORANGE, RAYWHITE, Vector2, begin_drawing, clear_background, draw_circle_v, draw_text, end_drawing,
    get_touch_point_count, get_touch_position, init_window, set_target_fps, window_should_close,
};

const MAX_TOUCH_POINTS: usize = 10;

fn main() {
    let screen_width = 800;
    let screen_height = 450;

    init_window(screen_width, screen_height, "raylib [core] example - input multitouch");
    set_target_fps(60);

    let mut touch_positions = [Vector2::zero(); MAX_TOUCH_POINTS];

    while !window_should_close() {
        let t_count = get_touch_point_count().min(MAX_TOUCH_POINTS as i32);
        for i in 0..t_count {
            touch_positions[i as usize] = get_touch_position(i);
        }

        begin_drawing();
        clear_background(RAYWHITE);

        for i in 0..(t_count as usize) {
            if touch_positions[i].x <= 0f32 && touch_positions[i].y <= 0f32 {
                continue;
            }
            draw_circle_v(touch_positions[i], 34f32, ORANGE);
            draw_text(
                i.to_string().as_str(),
                (touch_positions[i].x - 10f32) as i32,
                (touch_positions[i].y - 70f32) as i32,
                40,
                BLACK,
            );
        }

        draw_text(
            "touch the screen at multiple locations to get multiple balls",
            10,
            10,
            20,
            DARKGRAY,
        );

        draw_text(
            format!("Detected {} touch points", t_count).as_str(),
            10,
            40,
            20,
            DARKGRAY,
        );

        end_drawing();
    }
}
