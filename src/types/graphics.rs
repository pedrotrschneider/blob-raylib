use crate::{clear_window_state, close_window, get_current_monitor, get_monitor_count, get_monitor_height, get_monitor_name, get_monitor_physical_height, get_monitor_physical_width, get_monitor_position, get_monitor_refresh_rate, get_monitor_width, get_window_position, get_window_scale_dpi, init_window, is_window_focused, is_window_fullscreen, is_window_hidden, is_window_maximized, is_window_minimized, is_window_ready, is_window_resized, maximize_window, minimize_window, open_url, restore_window, set_config_flags, set_window_focused, set_window_icon, set_window_icons, set_window_max_size, set_window_min_size, set_window_monitor, set_window_position, set_window_size, set_window_state, set_window_title, take_screenshot, toggle_borderless_windowed, toggle_fullscreen, window_should_close, ConfigFlag, Image, Vector2};

pub struct Window;

impl Window {
    pub fn init(width: i32, height: i32, title: &str) {
        init_window(width, height, title);
    }

    pub fn should_close() -> bool {
        return window_should_close();
    }

    pub fn close() {
        close_window();
    }

    pub fn is_ready() -> bool {
        return is_window_ready();
    }

    pub fn is_fullscreen() -> bool {
        return is_window_fullscreen();
    }

    pub fn is_hidden() -> bool {
        return is_window_hidden();
    }

    pub fn is_minimized() -> bool {
        return is_window_minimized();
    }

    pub fn is_maximized() -> bool {
        return is_window_maximized();
    }

    pub fn is_focused() -> bool {
        return is_window_focused();
    }

    pub fn is_resized() -> bool {
        return is_window_resized()
    }

    pub fn set_config_flags(flags: ConfigFlag) {
        set_config_flags(flags);
    }

    pub fn set_state(flag: ConfigFlag) {
        set_window_state(flag);
    }

    pub fn clear_state(flag: ConfigFlag) {
        clear_window_state(flag);
    }

    pub fn toggle_fullscreen() {
        toggle_fullscreen();
    }

    pub fn toggle_borderless_windowed() {
        toggle_borderless_windowed();
    }

    pub fn maximize() {
        maximize_window();
    }

    pub fn minimize() {
        minimize_window();
    }

    pub fn restore() {
        restore_window();
    }

    pub fn set_icon(image: Image) {
        set_window_icon(image);
    }

    pub fn set_icons(images: &mut [Image]) {
        set_window_icons(images);
    }

    pub fn set_title(title: &str) {
        set_window_title(title);
    }

    pub fn set_position(x: i32, y: i32) {
        set_window_position(x, y);
    }

    pub fn set_monitor(monitor: i32) {
        set_window_monitor(monitor);
    }

    pub fn set_min_size(width: i32, height: i32) {
        set_window_min_size(width, height);
    }

    pub fn set_max_size(width: i32, height: i32) {
        set_window_max_size(width, height);
    }

    pub fn set_size(width: i32, height: i32) {
        set_window_size(width, height);
    }

    pub fn set_focused() {
        set_window_focused();
    }

    pub fn position() -> Vector2 {
        return get_window_position();
    }

    pub fn scale_dpi() -> Vector2 {
        return get_window_scale_dpi();
    }

    pub fn screenshot(filename: &str) {
        take_screenshot(filename);
    }

    pub fn open_url(url: &str) {
        open_url(url);
    }
}

pub struct Monitor {
    pub id: i32,
}

impl Monitor {
    pub fn count() -> i32 {
        return get_monitor_count();
    }

    pub fn current() -> Monitor {
        return get_current_monitor();
    }

    pub fn position(&self) -> Vector2 {
        return get_monitor_position(self);
    }

    pub fn width(&self) -> i32 {
        return get_monitor_width(self);
    }

    pub fn height(&self) -> i32 {
        return get_monitor_height(self);
    }

    pub fn physical_width(&self) -> i32 {
        return get_monitor_physical_width(self);
    }

    pub fn physical_height(&self) -> i32 {
        return get_monitor_physical_height(self);
    }

    pub fn refresh_rate(&self) -> i32 {
        return get_monitor_refresh_rate(self);
    }

    pub fn name(&self) -> &'static str {
        return get_monitor_name(self);
    }
}

impl From<i32> for Monitor {
    fn from(id: i32) -> Self {
        return Monitor { id };
    }
}