use skia_safe::{Color, Data, EncodedImageFormat, Paint, PaintStyle, Path, Surface};
use std::mem;

use crate::canvas::r#trait::*;

pub struct Canvas {
    surface: Surface,
    path: Path,
    paint: Paint,
}

impl HtmlCanvasApi for Canvas {
    fn new(width: i32, height: i32) -> Self {
        let mut surface = Surface::new_raster_n32_premul((width, height)).expect("no surface!");
        let path = Path::new();
        let mut paint = Paint::default();
        paint.set_color(Color::BLACK);
        paint.set_anti_alias(true);
        paint.set_stroke_width(1.0);
        surface.canvas().clear(Color::WHITE);
        Canvas {
            surface,
            path,
            paint,
        }
    }

    fn save(&mut self) {
        self.canvas().save();
    }

    fn restore(&mut self) {
        self.canvas().restore();
    }

    #[inline]
    fn translate(&mut self, dx: f32, dy: f32) {
        self.canvas().translate((dx, dy));
    }

    #[inline]
    fn scale(&mut self, sx: f32, sy: f32) {
        self.canvas().scale((sx, sy));
    }

    fn resize(&mut self, w: f64, h: f64) {
        todo!()
    }

    #[inline]
    fn move_to(&mut self, x: f32, y: f32) {
        self.begin_path();
        self.path.move_to((x, y));
    }

    #[inline]
    fn line_to(&mut self, x: f32, y: f32) {
        self.path.line_to((x, y));
    }

    #[inline]
    fn quad_to(&mut self, cpx: f32, cpy: f32, x: f32, y: f32) {
        self.path.quad_to((cpx, cpy), (x, y));
    }

    #[allow(dead_code)]
    #[inline]
    fn bezier_curve_to(&mut self, cp1x: f32, cp1y: f32, cp2x: f32, cp2y: f32, x: f32, y: f32) {
        self.path.cubic_to((cp1x, cp1y), (cp2x, cp2y), (x, y));
    }

    fn quadratic_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64) {
        self.path.quad_to((cp1x, cp1y), (cp2x, cp2y));
    }

    fn arc(&mut self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64, anti_clockwise: bool) {
        self.path.add_arc()
    }

    #[allow(dead_code)]
    #[inline]
    fn close_path(&mut self) {
        self.path.close();
    }

    #[inline]
    fn begin_path(&mut self) {
        let new_path = Path::new();
        self.surface.canvas().draw_path(&self.path, &self.paint);
        let _ = mem::replace(&mut self.path, new_path);
    }

    #[inline]
    fn stroke(&mut self) {
        self.paint.set_style(PaintStyle::Stroke);
        self.surface.canvas().draw_path(&self.path, &self.paint);
    }

    #[inline]
    fn fill(&mut self) {
        self.paint.set_style(PaintStyle::Fill);
        self.surface.canvas().draw_path(&self.path, &self.paint);
    }

    #[inline]
    fn set_line_width(&mut self, width: f32) {
        self.paint.set_stroke_width(width);
    }

    fn fill_style(&mut self, style: String) {
        // self.paint.set_style(PaintStyle::)
        todo!()
    }

    fn background_fill_style(&mut self, style: String) {
        todo!()
    }

    fn stroke_style(&mut self, style: String) {
        todo!()
    }

    fn font(&mut self, style: String) {
        // self.paint.set_font_style(FontStyle::)
        todo!()
    }

    fn shadow_color(&mut self, color: String) {
        todo!()
    }

    fn line_cap(&mut self, cap: String) {
        todo!()
    }

    fn line_dash(&mut self, values: Vec<i32>) {
        todo!()
    }

    fn shadow_blur(&mut self, value: f64) {
        todo!()
    }

    fn fill_text(&mut self, text: String, x: f64, y: f64) {
        todo!()
    }

    fn line_width(&mut self, width: f64) {
        todo!()
    }

    fn rect(&mut self, x: f64, y: f64, w: f64, h: f64) {
        todo!()
    }

    fn fill_rect(&mut self, x: f64, y: f64, w: f64, h: f64) {
        todo!()
    }

    fn clear_rect(&mut self, x: f64, y: f64, w: f64, h: f64) {
        todo!()
    }
}

impl Canvas {
    #[inline]
    pub fn data(&mut self) -> Data {
        let image = self.surface.image_snapshot();
        image.encode_to_data(EncodedImageFormat::PNG).unwrap()
    }

    #[inline]
    fn canvas(&mut self) -> &mut skia_safe::Canvas {
        self.surface.canvas()
    }
}