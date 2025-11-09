use blob_raylib::{
    BLACK, BLUE, ConfigFlags, DARKGRAY, GOLD, GRAY, GREEN, Gamepad, GamepadAxis, GamepadButton, Image, KeyboardKey,
    LIGHTGRAY, LIME, MAROON, PINK, RAYWHITE, RED, Texture2D, VIOLET, begin_drawing, clear_background, draw_circle,
    draw_rectangle, draw_rectangle_rounded, draw_text, draw_texture, draw_triangle, end_drawing,
    get_gamepad_button_pressed, init_window, is_key_pressed, set_config_flags, set_target_fps, window_should_close,
};

const XBOX_ALIAS_1: &str = "xbox";
const XBOX_ALIAS_2: &str = "x-box";
const PS_ALIAS: &str = "playstation";

fn main() {
    let screen_width = 800;
    let screen_height = 450;

    set_config_flags(ConfigFlags::Msaa4xHint.into());
    init_window(screen_width, screen_height, "raylib [core] example - input gamepad");
    set_target_fps(60);

    let ps_pad_img = Image::load_from_memory(".png", include_bytes!("resources/ps3.png"));
    let xbox_pad_img = Image::load_from_memory(".png", include_bytes!("resources/xbox.png"));

    let tex_ps_pad = Texture2D::from_image(ps_pad_img);
    let tex_xbox_pad = Texture2D::from_image(xbox_pad_img);

    // Set axis deadzones
    let left_stick_deadzone_x: f32 = 0.1f32;
    let left_stick_deadzone_y: f32 = 0.1f32;
    let right_stick_deadzone_x: f32 = 0.1f32;
    let right_stick_deadzone_y: f32 = 0.1f32;
    let left_trigger_deadzone: f32 = -0.9f32;
    let right_trigger_deadzone: f32 = -0.9f32;

    let mut gamepad = Gamepad::new(0);

    while !window_should_close() {
        begin_drawing();
        clear_background(RAYWHITE);

        if is_key_pressed(KeyboardKey::Left) {
            gamepad.prev();
        }
        if is_key_pressed(KeyboardKey::Right) {
            gamepad.next();
        }

        if gamepad.is_available() {
            draw_text(
                format!("Gamepad {:?}: {}", gamepad, gamepad.name().unwrap()).as_str(),
                10,
                10,
                10,
                BLACK,
            );

            // Get axis values
            let mut left_stick_x = gamepad.get_axis_movement(GamepadAxis::AxisLeftX);
            let mut left_stick_y = gamepad.get_axis_movement(GamepadAxis::AxisLeftY);
            let mut right_stick_x = gamepad.get_axis_movement(GamepadAxis::AxisRightX);
            let mut right_stick_y = gamepad.get_axis_movement(GamepadAxis::AxisRightY);
            let mut left_trigger = gamepad.get_axis_movement(GamepadAxis::AxisLeftTrigger);
            let mut right_trigger = gamepad.get_axis_movement(GamepadAxis::AxisRightTrigger);

            if left_stick_x > -left_stick_deadzone_x && left_stick_x < left_stick_deadzone_x {
                left_stick_x = 0f32;
            }
            if left_stick_y > -left_stick_deadzone_y && left_stick_y < left_stick_deadzone_y {
                left_stick_y = 0f32;
            }
            if right_stick_x > -right_stick_deadzone_x && right_stick_x < right_stick_deadzone_x {
                right_stick_x = 0f32;
            }
            if right_stick_y > -right_stick_deadzone_y && right_stick_y < right_stick_deadzone_y {
                right_stick_y = 0f32;
            }
            if left_trigger < left_trigger_deadzone {
                left_trigger = -1f32;
            }
            if right_trigger < right_trigger_deadzone {
                right_trigger = -1f32;
            }

            if gamepad.name().unwrap().to_lowercase().contains(XBOX_ALIAS_1)
                || gamepad.name().unwrap().to_lowercase().contains(XBOX_ALIAS_2)
            {
                draw_texture(tex_xbox_pad, 0, 0, DARKGRAY);

                if gamepad.is_button_down(GamepadButton::Middle) {
                    draw_circle(394, 89, 19f32, RED)
                }

                if gamepad.is_button_down(GamepadButton::MiddleRight) {
                    draw_circle(436, 150, 9.0, RED);
                }
                if gamepad.is_button_down(GamepadButton::MiddleLeft) {
                    draw_circle(352, 150, 9.0, RED);
                }
                if gamepad.is_button_down(GamepadButton::RightFaceLeft) {
                    draw_circle(501, 151, 15.0, BLUE);
                }
                if gamepad.is_button_down(GamepadButton::RightFaceDown) {
                    draw_circle(536, 187, 15.0, LIME);
                }
                if gamepad.is_button_down(GamepadButton::RightFaceRight) {
                    draw_circle(572, 151, 15.0, MAROON);
                }
                if gamepad.is_button_down(GamepadButton::RightFaceUp) {
                    draw_circle(536, 115, 15.0, GOLD);
                }

                // Draw buttons: d-pad
                draw_rectangle(317, 202, 19, 71, BLACK);
                draw_rectangle(293, 228, 69, 19, BLACK);

                if gamepad.is_button_down(GamepadButton::LeftFaceUp) {
                    draw_rectangle(317, 202, 19, 26, RED);
                }
                if gamepad.is_button_down(GamepadButton::LeftFaceDown) {
                    draw_rectangle(317, 202 + 45, 19, 26, RED);
                }
                if gamepad.is_button_down(GamepadButton::LeftFaceLeft) {
                    draw_rectangle(292, 228, 25, 19, RED);
                }
                if gamepad.is_button_down(GamepadButton::LeftFaceRight) {
                    draw_rectangle(292 + 44, 228, 26, 19, RED);
                }

                // Draw buttons: left-right back
                if gamepad.is_button_down(GamepadButton::LeftTrigger1) {
                    draw_circle(259, 61, 20.0, RED);
                }
                if gamepad.is_button_down(GamepadButton::RightTrigger1) {
                    draw_circle(536, 61, 20.0, RED);
                }

                // Draw axis: left joystick
                let mut left_gamepad_color = BLACK;
                if gamepad.is_button_down(GamepadButton::LeftThumb) {
                    left_gamepad_color = RED;
                }

                draw_circle(259, 152, 39.0, BLACK);
                draw_circle(259, 152, 34.0, LIGHTGRAY);
                draw_circle(
                    (259.0 + left_stick_x * 20.0) as i32,
                    (152.0 + left_stick_y * 20.0) as i32,
                    25.0,
                    left_gamepad_color,
                );

                // Draw axis: right joystick
                let mut right_gamepad_color = BLACK;
                if gamepad.is_button_down(GamepadButton::RightThumb) {
                    right_gamepad_color = RED;
                }

                draw_circle(461, 237, 38.0, BLACK);
                draw_circle(461, 237, 33.0, LIGHTGRAY);
                draw_circle(
                    (461.0 + right_stick_x * 20.0) as i32,
                    (237.0 + right_stick_y * 20.0) as i32,
                    25.0,
                    right_gamepad_color,
                );

                // Draw axis: left-right triggers
                draw_rectangle(170, 30, 15, 70, GRAY);
                draw_rectangle(604, 30, 15, 70, GRAY);
                draw_rectangle(170, 30, 15, (((1.0 + left_trigger) / 2.0) * 70.0) as i32, RED);
                draw_rectangle(604, 30, 15, (((1.0 + right_trigger) / 2.0) * 70.0) as i32, RED);
            } else if gamepad.name().unwrap().to_lowercase().contains(PS_ALIAS) {
                draw_texture(tex_ps_pad, 0, 0, DARKGRAY);

                // Draw buttons: ps
                if gamepad.is_button_down(GamepadButton::Middle) {
                    draw_circle(396, 222, 13.0, RED);
                }

                // Draw buttons: basic
                if gamepad.is_button_down(GamepadButton::MiddleLeft) {
                    draw_rectangle(328, 170, 32, 13, RED);
                }
                if gamepad.is_button_down(GamepadButton::MiddleRight) {
                    draw_triangle(
                        (436f32, 168f32).into(),
                        (436f32, 185f32).into(),
                        (464f32, 177f32).into(),
                        RED,
                    );
                }
                if gamepad.is_button_down(GamepadButton::RightFaceUp) {
                    draw_circle(557, 144, 13.0, LIME);
                }
                if gamepad.is_button_down(GamepadButton::RightFaceRight) {
                    draw_circle(586, 173, 13.0, RED);
                }
                if gamepad.is_button_down(GamepadButton::RightFaceDown) {
                    draw_circle(557, 203, 13.0, VIOLET);
                }
                if gamepad.is_button_down(GamepadButton::RightFaceLeft) {
                    draw_circle(527, 173, 13.0, PINK);
                }

                // Draw buttons: d-pad
                draw_rectangle(225, 132, 24, 84, BLACK);
                draw_rectangle(195, 161, 84, 25, BLACK);
                if gamepad.is_button_down(GamepadButton::LeftFaceUp) {
                    draw_rectangle(225, 132, 24, 29, RED);
                }
                if gamepad.is_button_down(GamepadButton::LeftFaceDown) {
                    draw_rectangle(225, 132 + 54, 24, 30, RED);
                }
                if gamepad.is_button_down(GamepadButton::LeftFaceLeft) {
                    draw_rectangle(195, 161, 30, 25, RED);
                }
                if gamepad.is_button_down(GamepadButton::LeftFaceRight) {
                    draw_rectangle(195 + 54, 161, 30, 25, RED);
                }

                // Draw buttons: left-right back buttons
                if gamepad.is_button_down(GamepadButton::LeftTrigger1) {
                    draw_circle(239, 82, 20.0, RED);
                }
                if gamepad.is_button_down(GamepadButton::RightTrigger1) {
                    draw_circle(557, 82, 20.0, RED);
                }

                // Draw axis: left joystick
                let mut left_gamepad_color = BLACK;
                if gamepad.is_button_down(GamepadButton::LeftThumb) {
                    left_gamepad_color = RED;
                }

                draw_circle(319, 255, 35.0, BLACK);
                draw_circle(319, 255, 31.0, LIGHTGRAY);
                draw_circle(
                    (319.0 + left_stick_x * 20.0) as i32,
                    (255.0 + left_stick_y * 20.0) as i32,
                    25.0,
                    left_gamepad_color,
                );

                // Draw axis: right joystick
                let mut right_gamepad_color = BLACK;
                if gamepad.is_button_down(GamepadButton::RightThumb) {
                    right_gamepad_color = RED;
                }

                draw_circle(475, 255, 35.0, BLACK);
                draw_circle(475, 255, 31.0, LIGHTGRAY);
                draw_circle(
                    (475.0 + right_stick_x * 20.0) as i32,
                    (255.0 + right_stick_y * 20.0) as i32,
                    25.0,
                    right_gamepad_color,
                );

                // Draw axis: left-right triggers
                draw_rectangle(169, 48, 15, 70, GRAY);
                draw_rectangle(611, 48, 15, 70, GRAY);
                draw_rectangle(169, 48, 15, (((1.0 + left_trigger) / 2.0) * 70.0) as i32, RED);
                draw_rectangle(611, 48, 15, (((1.0 + right_trigger) / 2.0) * 70.0) as i32, RED);
            } else {
                // Draw background: generic
                draw_rectangle_rounded((175f32, 110f32, 460f32, 220f32).into(), 0.3, 16, DARKGRAY);

                // Draw buttons: basic
                draw_circle(365, 170, 12.0, RAYWHITE);
                draw_circle(405, 170, 12.0, RAYWHITE);
                draw_circle(445, 170, 12.0, RAYWHITE);
                draw_circle(516, 191, 17.0, RAYWHITE);
                draw_circle(551, 227, 17.0, RAYWHITE);
                draw_circle(587, 191, 17.0, RAYWHITE);
                draw_circle(551, 155, 17.0, RAYWHITE);

                if gamepad.is_button_down(GamepadButton::MiddleLeft) {
                    draw_circle(365, 170, 10.0, RED);
                }
                if gamepad.is_button_down(GamepadButton::Middle) {
                    draw_circle(405, 170, 10.0, GREEN);
                }
                if gamepad.is_button_down(GamepadButton::MiddleRight) {
                    draw_circle(445, 170, 10.0, BLUE);
                }
                if gamepad.is_button_down(GamepadButton::RightFaceLeft) {
                    draw_circle(516, 191, 15.0, GOLD);
                }
                if gamepad.is_button_down(GamepadButton::RightFaceDown) {
                    draw_circle(551, 227, 15.0, BLUE);
                }
                if gamepad.is_button_down(GamepadButton::RightFaceRight) {
                    draw_circle(587, 191, 15.0, GREEN);
                }
                if gamepad.is_button_down(GamepadButton::RightFaceUp) {
                    draw_circle(551, 155, 15.0, RED);
                }

                // Draw buttons: d-pad
                draw_rectangle(245, 145, 28, 88, RAYWHITE);
                draw_rectangle(215, 174, 88, 29, RAYWHITE);
                draw_rectangle(247, 147, 24, 84, BLACK);
                draw_rectangle(217, 176, 84, 25, BLACK);

                if gamepad.is_button_down(GamepadButton::LeftFaceUp) {
                    draw_rectangle(247, 147, 24, 29, RED);
                }
                if gamepad.is_button_down(GamepadButton::LeftFaceDown) {
                    draw_rectangle(247, 147 + 54, 24, 30, RED);
                }
                if gamepad.is_button_down(GamepadButton::LeftFaceLeft) {
                    draw_rectangle(217, 176, 30, 25, RED);
                }
                if gamepad.is_button_down(GamepadButton::LeftFaceRight) {
                    draw_rectangle(217 + 54, 176, 30, 25, RED);
                }

                // Draw buttons: left-right back
                draw_rectangle_rounded((215f32, 98f32, 100f32, 10f32).into(), 0.5, 16, DARKGRAY);
                draw_rectangle_rounded((495f32, 98f32, 100f32, 10f32).into(), 0.5, 16, DARKGRAY);

                if gamepad.is_button_down(GamepadButton::LeftTrigger1) {
                    draw_rectangle_rounded((215f32, 98f32, 100f32, 10f32).into(), 0.5, 16, RED);
                }
                if gamepad.is_button_down(GamepadButton::RightTrigger1) {
                    draw_rectangle_rounded((495f32, 98f32, 100f32, 10f32).into(), 0.5, 16, RED);
                }

                // Draw axis: left joystick
                let mut left_gamepad_color = BLACK;
                if gamepad.is_button_down(GamepadButton::LeftThumb) {
                    left_gamepad_color = RED;
                }

                draw_circle(345, 260, 40.0, BLACK);
                draw_circle(345, 260, 35.0, LIGHTGRAY);
                draw_circle(
                    (345.0 + left_stick_x * 20.0) as i32,
                    (260.0 + left_stick_y * 20.0) as i32,
                    25.0,
                    left_gamepad_color,
                );

                // Draw axis: right joystick
                let mut right_gamepad_color = BLACK;
                if gamepad.is_button_down(GamepadButton::RightThumb) {
                    right_gamepad_color = RED;
                }

                draw_circle(465, 260, 40.0, BLACK);
                draw_circle(465, 260, 35.0, LIGHTGRAY);
                draw_circle(
                    (465.0 + right_stick_x * 20.0) as i32,
                    (260.0 + right_stick_y * 20.0) as i32,
                    25.0,
                    right_gamepad_color,
                );

                // Draw axis: left-right triggers
                draw_rectangle(151, 110, 15, 70, GRAY);
                draw_rectangle(644, 110, 15, 70, GRAY);
                draw_rectangle(151, 110, 15, (((1.0 + left_trigger) / 2.0) * 70.0) as i32, RED);
                draw_rectangle(644, 110, 15, (((1.0 + right_trigger) / 2.0) * 70.0) as i32, RED);
            }
            draw_text(
                &format!("DETECTED AXIS [{}]:", gamepad.get_axis_count()),
                10,
                50,
                10,
                MAROON,
            );

            for i in 0..gamepad.get_axis_count() {
                draw_text(
                    &format!("AXIS {}: {:.02}", i, gamepad.get_axis_movement(i.try_into().unwrap())),
                    20,
                    70 + 20 * i,
                    10,
                    DARKGRAY,
                );
            }

            if let Some(button) = get_gamepad_button_pressed() {
                draw_text(&format!("DETECTED BUTTON: {:?}", button), 10, 430, 10, RED);
            } else {
                draw_text("DETECTED BUTTON: NONE", 10, 430, 10, GRAY);
            }
        } else {
            draw_text(&format!("GP{}: NOT DETECTED", gamepad.id()), 10, 10, 10, GRAY);
            draw_texture(tex_xbox_pad, 0, 0, LIGHTGRAY);
        }

        end_drawing();
    }
}
