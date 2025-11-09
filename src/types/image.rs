use crate::{
    Color, Font, Rectangle, Texture2D, Vector2, export_image, export_image_to_memory, gen_image_cellular,
    gen_image_checked, gen_image_color, gen_image_gradient_linear, gen_image_gradient_radial,
    gen_image_gradient_square, gen_image_perlin_noise, gen_image_text, gen_image_white_noise, get_image_alpha_border,
    get_image_color, image_alpha_clear, image_alpha_crop, image_alpha_mask, image_alpha_premultiply,
    image_blur_gaussian, image_clear_background, image_color_brightness, image_color_contrast, image_color_grayscale,
    image_color_invert, image_color_replace, image_color_tint, image_copy, image_crop, image_dither, image_draw,
    image_draw_circle, image_draw_circle_lines, image_draw_circle_lines_v, image_draw_circle_v, image_draw_line,
    image_draw_line_ex, image_draw_line_v, image_draw_pixel, image_draw_pixel_v, image_draw_rectangle,
    image_draw_rectangle_lines, image_draw_rectangle_rec, image_draw_rectangle_v, image_draw_text, image_draw_text_ex,
    image_draw_triangle, image_draw_triangle_ex, image_draw_triangle_fan, image_draw_triangle_lines,
    image_draw_triangle_strip, image_flip_horizontal, image_flip_vertical, image_format, image_from_channel,
    image_from_image, image_kernel_convolution, image_mipmaps, image_resize, image_resize_canvas, image_resize_nn,
    image_rotate, image_rotate_ccw, image_rotate_cw, image_text, image_text_ex, image_to_pot, is_image_valid,
    load_image, load_image_anim, load_image_anim_from_memory, load_image_colors, load_image_from_memory,
    load_image_from_screen, load_image_from_texture, load_image_palette, load_image_raw, unload_image,
};
use std::os::raw::c_void;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    UncompressedGrayscale = 1,
    UncompressedGrayAlpha,
    UncompressedR5g6b5,
    UncompressedR8g8b8,
    UncompressedR5g5b5a1,
    UncompressedR4g4b4a4,
    UncompressedR8g8b8a8,
    UncompressedR32,
    UncompressedR32g32b32,
    UncompressedR32g32b32a32,
    UncompressedR16,
    UncompressedR16g16b16,
    UncompressedR16g16b16a16,
    CompressedDxt1Rgb,
    CompressedDxt1Rgba,
    CompressedDxt3Rgba,
    CompressedDxt5Rgba,
    CompressedEtc1Rgb,
    CompressedEtc2Rgb,
    CompressedEtc2EacRgba,
    CompressedPvrtRgb,
    CompressedPvrtRgba,
    CompressedAstc4x4Rgba,
    CompressedAstc8x8Rgba,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Image {
    pub data: *mut c_void,   // Image raw data
    pub width: i32,          // Image base width
    pub height: i32,         // Image base height
    pub mipmaps: i32,        // Mipmap levels, 1 by default
    pub format: PixelFormat, // Data format (PixelFormat type)
}

impl Image {
    // Loading

    pub fn load(filename: &str) -> Image {
        return load_image(filename);
    }

    pub fn load_raw(filename: &str, width: i32, height: i32, format: PixelFormat, header_size: i32) -> Image {
        return load_image_raw(filename, width, height, format, header_size);
    }

    pub fn load_anim(filename: &str, frames: &mut i32) -> Image {
        return load_image_anim(filename, frames);
    }

    pub fn load_anim_from_memory(file_type: &str, file_data: &[u8], frames: &mut i32) -> Image {
        return load_image_anim_from_memory(file_type, file_data, frames);
    }

    pub fn load_from_memory(file_type: &str, file_data: &[u8]) -> Image {
        return load_image_from_memory(file_type, file_data);
    }

    pub fn from_texture(texture: Texture2D) -> Image {
        return load_image_from_texture(texture);
    }

    pub fn from_screen() -> Image {
        return load_image_from_screen();
    }

    pub fn is_valid(&self) -> bool {
        return is_image_valid(*self);
    }

    pub fn unload(&self) {
        unload_image(*self);
    }

    pub fn export(&self, filename: &str) -> bool {
        return export_image(*self, filename);
    }

    pub fn export_to_memory(&self, file_type: &str) -> Vec<u8> {
        return export_image_to_memory(*self, file_type);
    }

    // Generation

    pub fn gen_color(width: i32, height: i32, color: Color) -> Image {
        return gen_image_color(width, height, color);
    }

    pub fn gen_linear_gradient(width: i32, height: i32, direction: i32, start: Color, end: Color) -> Image {
        return gen_image_gradient_linear(width, height, direction, start, end);
    }

    pub fn gen_radial_gradient(width: i32, height: i32, density: f32, inner: Color, outer: Color) -> Image {
        return gen_image_gradient_radial(width, height, density, inner, outer);
    }

    pub fn gen_square_gradient(width: i32, height: i32, density: f32, inner: Color, outer: Color) -> Image {
        return gen_image_gradient_square(width, height, density, inner, outer);
    }

    pub fn gen_checked(width: i32, height: i32, checks_x: i32, checks_y: i32, col1: Color, col2: Color) -> Image {
        return gen_image_checked(width, height, checks_x, checks_y, col1, col2);
    }

    pub fn gen_white_noise(width: i32, height: i32, factor: f32) -> Image {
        return gen_image_white_noise(width, height, factor);
    }

    pub fn gen_perlin_noise(width: i32, height: i32, offset_x: i32, offset_y: i32, scale: f32) -> Image {
        return gen_image_perlin_noise(width, height, offset_x, offset_y, scale);
    }

    pub fn gen_cellular(width: i32, height: i32, tile_size: i32) -> Image {
        return gen_image_cellular(width, height, tile_size);
    }

    pub fn gen_text(width: i32, height: i32, text: &str) -> Image {
        return gen_image_text(width, height, text);
    }

    // Manipulation

    pub fn copy(&self) -> Image {
        return image_copy(*self);
    }

    pub fn from_image(image: Image, rec: Rectangle) -> Image {
        return image_from_image(image, rec);
    }

    pub fn from_channel(image: Image, selected_channel: i32) -> Image {
        return image_from_channel(image, selected_channel);
    }

    pub fn from_text(text: &str, font_size: i32, color: Color) -> Image {
        return image_text(text, font_size, color);
    }

    pub fn from_text_ex(font: Font, text: &str, font_size: f32, spacing: f32, tint: Color) -> Image {
        return image_text_ex(font, text, font_size, spacing, tint);
    }

    pub fn change_format(&mut self, new_format: PixelFormat) {
        return image_format(self, new_format);
    }

    pub fn pad_to_power_of_two(&mut self, fill: Color) {
        image_to_pot(self, fill);
    }

    pub fn crop(&mut self, crop: Rectangle) {
        image_crop(self, crop);
    }

    pub fn alpha_crop(&mut self, threshold: f32) {
        image_alpha_crop(self, threshold);
    }

    pub fn alpha_clear(&mut self, color: Color, threshold: f32) {
        image_alpha_clear(self, color, threshold);
    }

    pub fn alpha_mask(&mut self, alpha_mask: Image) {
        image_alpha_mask(self, alpha_mask);
    }

    pub fn alpha_premultiply(&mut self) {
        image_alpha_premultiply(self);
    }

    pub fn blur_gaussian(&mut self, blur_size: i32) {
        image_blur_gaussian(self, blur_size);
    }

    pub fn kernel_convolution(&mut self, kernel: &[f32]) {
        image_kernel_convolution(self, kernel, kernel.len() as i32);
    }

    pub fn resize(&mut self, new_width: i32, new_height: i32) {
        image_resize(self, new_width, new_height);
    }

    pub fn resize_nearest_neighbor(&mut self, new_width: i32, new_height: i32) {
        image_resize_nn(self, new_width, new_height);
    }

    pub fn resize_canvas(&mut self, new_width: i32, new_height: i32, offset_x: i32, offset_y: i32, fill: Color) {
        image_resize_canvas(self, new_width, new_height, offset_x, offset_y, fill);
    }

    pub fn gen_mipmaps(&mut self) {
        image_mipmaps(self);
    }

    pub fn dither(&mut self, r_bpp: i32, g_bpp: i32, b_bpp: i32, a_bpp: i32) {
        image_dither(self, r_bpp, g_bpp, b_bpp, a_bpp);
    }

    pub fn flip_vertical(&mut self) {
        image_flip_vertical(self);
    }

    pub fn flip_horizontal(&mut self) {
        image_flip_horizontal(self);
    }

    pub fn rotate(&mut self, degrees: i32) {
        image_rotate(self, degrees);
    }

    pub fn rotate_clockwise(&mut self) {
        image_rotate_cw(self);
    }

    pub fn rotate_counter_clockwise(&mut self) {
        image_rotate_ccw(self);
    }

    pub fn color_tint(&mut self, color: Color) {
        image_color_tint(self, color);
    }

    pub fn color_invert(&mut self) {
        image_color_invert(self);
    }

    pub fn color_grayscale(&mut self) {
        image_color_grayscale(self);
    }

    pub fn color_contrast(&mut self, contrast: f32) {
        image_color_contrast(self, contrast);
    }

    pub fn color_brightness(&mut self, brightness: i32) {
        image_color_brightness(self, brightness);
    }

    pub fn color_replace(&mut self, color: Color, replace: Color) {
        image_color_replace(self, color, replace);
    }

    pub fn load_colors(&self) -> &'static mut [Color] {
        return load_image_colors(*self);
    }

    pub fn load_palette(&self, max_palette_size: i32) -> &'static mut [Color] {
        return load_image_palette(*self, max_palette_size);
    }

    pub fn get_alpha_border(&self, threshold: f32) -> Rectangle {
        return get_image_alpha_border(*self, threshold);
    }

    pub fn get_color(&self, x: i32, y: i32) -> Color {
        return get_image_color(*self, x, y);
    }

    // Drawing

    pub fn clear_background(&mut self, color: Color) {
        image_clear_background(self, color);
    }

    pub fn draw_pixel(&mut self, pos_x: i32, pos_y: i32, color: Color) {
        image_draw_pixel(self, pos_x, pos_y, color);
    }

    pub fn draw_pixel_v(&mut self, position: Vector2, color: Color) {
        image_draw_pixel_v(self, position, color);
    }

    pub fn draw_line(&mut self, start_pos_x: i32, start_pos_y: i32, end_pos_x: i32, end_pos_y: i32, color: Color) {
        image_draw_line(self, start_pos_x, start_pos_y, end_pos_x, end_pos_y, color);
    }

    pub fn draw_line_v(&mut self, start: Vector2, end: Vector2, color: Color) {
        image_draw_line_v(self, start, end, color);
    }

    pub fn draw_line_ex(dst: &mut Image, start: Vector2, end: Vector2, thick: i32, color: Color) {
        image_draw_line_ex(dst, start, end, thick, color);
    }

    pub fn draw_circle(&mut self, center_x: i32, center_y: i32, radius: i32, color: Color) {
        image_draw_circle(self, center_x, center_y, radius, color);
    }

    pub fn draw_circle_v(&mut self, center: Vector2, radius: i32, color: Color) {
        image_draw_circle_v(self, center, radius, color);
    }

    pub fn draw_circle_lines(&mut self, center_x: i32, center_y: i32, radius: i32, color: Color) {
        image_draw_circle_lines(self, center_x, center_y, radius, color);
    }

    pub fn draw_circle_lines_v(&mut self, center: Vector2, radius: i32, color: Color) {
        image_draw_circle_lines_v(self, center, radius, color);
    }

    pub fn draw_rectangle(&mut self, pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
        image_draw_rectangle(self, pos_x, pos_y, width, height, color);
    }

    pub fn draw_rectangle_v(&mut self, position: Vector2, size: Vector2, color: Color) {
        image_draw_rectangle_v(self, position, size, color);
    }

    pub fn draw_rectangle_rec(&mut self, rec: Rectangle, color: Color) {
        image_draw_rectangle_rec(self, rec, color);
    }

    pub fn draw_rectangle_lines(&mut self, rec: Rectangle, thick: i32, color: Color) {
        image_draw_rectangle_lines(self, rec, thick, color);
    }

    pub fn draw_triangle(&mut self, v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
        image_draw_triangle(self, v1, v2, v3, color);
    }

    pub fn draw_triangle_ex(&mut self, v1: Vector2, v2: Vector2, v3: Vector2, c1: Color, c2: Color, c3: Color) {
        image_draw_triangle_ex(self, v1, v2, v3, c1, c2, c3);
    }

    pub fn draw_triangle_lines(&mut self, v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
        image_draw_triangle_lines(self, v1, v2, v3, color);
    }

    pub fn draw_triangle_fan(&mut self, points: &[Vector2], color: Color) {
        image_draw_triangle_fan(self, points, color);
    }

    pub fn draw_triangle_strip(&mut self, points: &[Vector2], color: Color) {
        image_draw_triangle_strip(self, points, color);
    }

    pub fn draw(&mut self, src: Image, src_rec: Rectangle, dst_rec: Rectangle, tint: Color) {
        image_draw(self, src, src_rec, dst_rec, tint);
    }

    pub fn draw_text(&mut self, text: &str, pos_x: i32, pos_y: i32, font_size: i32, color: Color) {
        image_draw_text(self, text, pos_x, pos_y, font_size, color);
    }

    pub fn draw_text_ex(
        &mut self,
        font: Font,
        text: &str,
        position: Vector2,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) {
        image_draw_text_ex(self, font, text, position, font_size, spacing, tint);
    }
}
