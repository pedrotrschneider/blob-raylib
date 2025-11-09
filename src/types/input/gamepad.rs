use crate::{
    get_gamepad_axis_count, get_gamepad_axis_movement, get_gamepad_name, is_gamepad_available, is_gamepad_button_down,
    is_gamepad_button_pressed, is_gamepad_button_released, is_gamepad_button_up, set_gamepad_vibration,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gamepad {
    pub(crate) id: i32,
}

impl Gamepad {
    pub fn new(id: i32) -> Self {
        return Self { id };
    }

    pub fn next(&mut self) {
        self.id += 1;
    }

    pub fn prev(&mut self) {
        if self.id == 0 {
            return;
        }
        self.id -= 1;
    }

    pub fn id(&self) -> i32 {
        return self.id;
    }

    pub fn is_available(&self) -> bool {
        return is_gamepad_available(self);
    }

    pub fn name(&self) -> Option<&'static str> {
        return get_gamepad_name(self);
    }

    pub fn is_button_pressed(&self, button: GamepadButton) -> bool {
        return is_gamepad_button_pressed(self, button);
    }

    pub fn is_button_down(&self, button: GamepadButton) -> bool {
        return is_gamepad_button_down(self, button);
    }

    pub fn is_button_released(&self, button: GamepadButton) -> bool {
        return is_gamepad_button_released(self, button);
    }

    pub fn is_button_up(&self, button: GamepadButton) -> bool {
        return is_gamepad_button_up(self, button);
    }

    pub fn get_axis_count(&self) -> i32 {
        return get_gamepad_axis_count(self);
    }

    pub fn get_axis_movement(&self, axis: GamepadAxis) -> f32 {
        return get_gamepad_axis_movement(self, axis);
    }

    pub fn set_vibration(&self, left_motor: f32, right_motor: f32, duration: f32) {
        return set_gamepad_vibration(self, left_motor, right_motor, duration);
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamepadButton {
    Unknown = 0,    // Unknown button, just for error checking
    LeftFaceUp,     // Gamepad left DPAD up button
    LeftFaceRight,  // Gamepad left DPAD right button
    LeftFaceDown,   // Gamepad left DPAD down button
    LeftFaceLeft,   // Gamepad left DPAD left button
    RightFaceUp,    // Gamepad right button up (i.e. PS3: Triangle, Xbox: Y)
    RightFaceRight, // Gamepad right button right (i.e. PS3: Circle, Xbox: B)
    RightFaceDown,  // Gamepad right button down (i.e. PS3: Cross, Xbox: A)
    RightFaceLeft,  // Gamepad right button left (i.e. PS3: Square, Xbox: X)
    LeftTrigger1,   // Gamepad top/back trigger left (first), it could be a trailing button
    LeftTrigger2,   // Gamepad top/back trigger left (second), it could be a trailing button
    RightTrigger1,  // Gamepad top/back trigger right (first), it could be a trailing button
    RightTrigger2,  // Gamepad top/back trigger right (second), it could be a trailing button
    MiddleLeft,     // Gamepad center buttons, left one (i.e. PS3: Select)
    Middle,         // Gamepad center buttons, middle one (i.e. PS3: PS, Xbox: XBOX)
    MiddleRight,    // Gamepad center buttons, right one (i.e. PS3: Start)
    LeftThumb,      // Gamepad joystick pressed button left
    RightThumb,     // Gamepad joystick pressed button right
}

impl TryFrom<i32> for GamepadButton {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        return match value {
            0 => Ok(GamepadButton::Unknown),
            1 => Ok(GamepadButton::LeftFaceUp),
            2 => Ok(GamepadButton::LeftFaceRight),
            3 => Ok(GamepadButton::LeftFaceDown),
            4 => Ok(GamepadButton::LeftFaceLeft),
            5 => Ok(GamepadButton::RightFaceUp),
            6 => Ok(GamepadButton::RightFaceRight),
            7 => Ok(GamepadButton::RightFaceDown),
            8 => Ok(GamepadButton::RightFaceLeft),
            9 => Ok(GamepadButton::LeftTrigger1),
            10 => Ok(GamepadButton::LeftTrigger2),
            11 => Ok(GamepadButton::RightTrigger1),
            12 => Ok(GamepadButton::RightTrigger2),
            13 => Ok(GamepadButton::MiddleLeft),
            14 => Ok(GamepadButton::Middle),
            15 => Ok(GamepadButton::MiddleRight),
            16 => Ok(GamepadButton::LeftThumb),
            17 => Ok(GamepadButton::RightThumb),
            _ => Err(()),
        };
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamepadAxis {
    AxisLeftX = 0,        // Gamepad left stick X axis
    AxisLeftY = 1,        // Gamepad left stick Y axis
    AxisRightX = 2,       // Gamepad right stick X axis
    AxisRightY = 3,       // Gamepad right stick Y axis
    AxisLeftTrigger = 4,  // Gamepad back trigger left, pressure level: [1..-1]
    AxisRightTrigger = 5, // Gamepad back trigger right, pressure level: [1..-1]
}

impl TryFrom<i32> for GamepadAxis {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        return match value {
            0 => Ok(GamepadAxis::AxisLeftX),
            1 => Ok(GamepadAxis::AxisLeftY),
            2 => Ok(GamepadAxis::AxisRightX),
            3 => Ok(GamepadAxis::AxisRightY),
            4 => Ok(GamepadAxis::AxisLeftTrigger),
            5 => Ok(GamepadAxis::AxisRightTrigger),
            _ => Err(()),
        };
    }
}
