use std::fs::File;
use std::mem;
use std::io::Write;

use hex_color::HexColor;
use pathfinder_canvas::{CanvasFontContext, CanvasImageSource, CanvasRenderingContext2D, ColorU, FillRule, FillStyle, LineCap, Path2D, TextAlign, TextBaseline};
use pathfinder_geometry::rect::RectF;
use pathfinder_geometry::vector::{vec2f, vec2i};
use pathfinder_content::outline::ArcDirection;
use pathfinder_export::FileFormat;
use pathfinder_renderer::scene::{Scene, DrawPathId};

use crate::canvas::r#trait::*;

/*
 find postscriptnames by looking at Mac FontBook font details, or on linux
 use fc-list to list all available, and then fc-scan *font path* to find details
 */

const FONT_ARIAL_REGULAR : &str = "ArialMT";
const FONT_TIMES_REGULAR : &str = "TimesNewRomanPSMT";

pub struct Canvas {
    context: CanvasRenderingContext2D,
    path: Option<Path2D>
}

impl HtmlCanvasApi for Canvas {
    fn new(width: i32, height: i32) -> Self {
        let framebuffer_size = vec2i(width, height);
        let font_context = CanvasFontContext::from_system_source();
        let canvas = pathfinder_canvas::Canvas::new(framebuffer_size.to_f32());
        let context = canvas.get_context_2d(font_context);
        Self {
            context,
            path: None
        }
    }

    // todo: save to standardised resource paths indexed by
    // editor hash and model content hash
    fn export(self, output: &str) -> anyhow::Result<()> {
        // render svg
        let scene = self
            .context
            .into_canvas()
            .into_scene();

        pathfinder_export::Export::export(
            &scene,
            &mut File::create(output)?,
            FileFormat::SVG)?;

        // render png
        let mut opt = usvg::Options::default();
        // Get file's absolute directory.
        opt.fontdb.load_system_fonts();

        let svg_data = std::fs::read_to_string(output)?;
        let rtree = usvg::Tree::from_data(
            &svg_data.as_bytes(),
            &opt.to_ref()
        ).unwrap();

        let pixmap_size = rtree
            .svg_node()
            .size
            .to_screen_size();

        let mut pixmap = tiny_skia::Pixmap::new(
            pixmap_size.width(),
            pixmap_size.height()
        ).unwrap();
        resvg::render(
            &rtree,
            usvg::FitTo::Original,
            tiny_skia::Transform::default(),
            pixmap.as_mut()
        ).unwrap();
        pixmap.save_png("./output.png")?;

        Ok(())
    }

    fn save(&mut self) {
        self.context.save();
    }

    fn restore(&mut self) {
        self.context.restore();
    }

    fn translate(&mut self, dx: f32, dy: f32) {
        self.context.translate(vec2f(dx, dy));
    }

    fn scale(&mut self, sx: f32, sy: f32) {
        self.context.scale(vec2f(sx, sy));
    }

    fn resize(&mut self, w: f64, h: f64) {
        todo!()
    }

    fn move_to(&mut self, x: f32, y: f32) {
        self
            .get_path_mut()
            .move_to(vec2f(x, y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self
            .get_path_mut()
            .line_to(vec2f(x, y));
    }

    fn bezier_curve_to(&mut self,
                       cp1x: f32,
                       cp1y: f32,
                       cp2x: f32,
                       cp2y: f32,
                       x: f32,
                       y: f32
    ) {
        self
            .get_path_mut()
            .bezier_curve_to(
                vec2f(cp1x, cp1y),
                vec2f(cp2x, cp2y),
                vec2f(x, y));
    }

    fn quadratic_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64) {
        self
            .get_path_mut()
            .quadratic_curve_to(
                vec2f(cp1x as f32, cp1y as f32),
                vec2f(cp2x as f32, cp2y as f32));
    }

    fn arc(&mut self,
           x: f64,
           y: f64,
           radius: f64,
           start_angle: f64,
           end_angle: f64,
           anti_clockwise: bool
    ) {
        self
            .get_path_mut()
            .arc(
                vec2f(x as f32, y as f32),
                radius as f32,
                start_angle as f32,
                end_angle as f32, {
                    if anti_clockwise {
                        ArcDirection::CCW
                    }
                    else {
                        ArcDirection::CW
                    }
                }
            );
    }

    fn close_path(&mut self) {
        self
            .get_path_mut()
            .close_path();
    }

    fn begin_path(&mut self) {
        match &self.path {
            None => {
                self.path = Some(Path2D::new());
            },
            Some(path) => {
                panic!("another path was already started!")
            }
        }
    }

    fn stroke(&mut self) {
        let path = self.close_and_take_path();
        self.context.stroke_path(path);
    }

    fn fill(&mut self) {
        let path = self.close_and_take_path();
        self
            .context
            .fill_path(path, FillRule::Winding);
    }

    fn fill_style(&mut self, style: String) {
        self
            .context
            .set_fill_style(
                parse_fill_style(style).unwrap());
    }

    fn background_fill_style(&mut self, style: String) {
        todo!("see canvascontext.ts")
    }

    fn stroke_style(&mut self, style: String) {
        self.context.set_stroke_style(
            parse_stroke_style(style).unwrap()
        );
    }

    fn font(&mut self, style: String) {
        // self.context.set_font(&[FONT_NAME_REGULAR, FONT_NAME_EMOJI][..]);
        // self.context.set_font_size(18.0);
        // self.context.set_fill_style(ColorU::white());
        // self.context.set_text_align(TextAlign::Left);
        // self.context.set_text_baseline(TextBaseline::Alphabetic);

        if style.to_lowercase().as_str() == "10pt arial" {
            self.context.set_font_size(10.0);
            self.context.set_font(FONT_ARIAL_REGULAR);
        }

        else if style.to_lowercase().as_str() == "11pt arial" {
            self.context.set_font_size(11.0);
            self.context.set_font(FONT_ARIAL_REGULAR);
        }

        else if style.to_lowercase().as_str() == "7.5pt arial" {
            self.context.set_font_size(7.5);
            self.context.set_font(FONT_ARIAL_REGULAR);
        }

        else if style.to_lowercase().as_str() == "16pt arial" {
            self.context.set_font_size(16.0);
            self.context.set_font(FONT_ARIAL_REGULAR);
        }

        // todo: what is 'normal' here? not bold?
        else if style.to_lowercase().as_str() == "normal 16pt times" {
            self.context.set_font_size(16.0);
            self.context.set_font(FONT_TIMES_REGULAR);
        }

        else {
            todo!("unrecognized font style string: {}", style)
        }
    }

    // todo: use csscolorparser instead?
    fn shadow_color(&mut self, color: String) {
        if let Ok(HexColor {r, g, b, a}) = HexColor::parse(&color) {
            self.context.set_shadow_color(ColorU::new(
                r, g, b, a
            ));
        }

        panic!("could not parse hex color string: {}", color)
    }

    fn line_cap(&mut self, cap: String) {
        self.context.set_line_cap({
            match cap.to_lowercase().as_str() {
                "butt" => LineCap::Butt,
                "round" => LineCap::Round,
                "square" => LineCap::Square,
                _ => panic!("unrecognized line cap identifier: {}", cap)
            }
        });
    }

    fn line_dash(&mut self, values: Vec<f64>) {
        self.context.set_line_dash(values.into_iter().map(|v| v as f32).collect());
    }

    fn shadow_blur(&mut self, value: f64) {
        self.context.set_shadow_blur(value as f32);
    }

    fn fill_text(&mut self, text: String, x: f64, y: f64) {
        self.context.fill_text(text.as_str(), vec2f(x as f32, y as f32));
    }

    fn line_width(&mut self, width: f64) {
        self.context.set_line_width(width as f32);
    }

    fn rect(&mut self, x: f64, y: f64, w: f64, h: f64) {
        self.context.stroke_rect(
            RectF::new(
                vec2f(x as f32, y as f32),
                vec2f(w as f32, h as f32)));
    }

    fn fill_rect(&mut self, x: f64, y: f64, w: f64, h: f64) {
        self.context.fill_rect(
            RectF::new(
                vec2f(x as f32, y as f32),
                vec2f(w as f32, h as f32)));
    }

    fn clear_rect(&mut self, x: f64, y: f64, w: f64, h: f64) {
        self.context.clear_rect(RectF::new(
            vec2f(x as f32, y as f32),
            vec2f(w as f32, h as f32)));
    }
}

impl Canvas {
    pub fn get_path_mut(&mut self) -> &mut Path2D {
        self.path.as_mut().expect("tried to get Path2D, but none was created yet!")
    }

    fn take_path(&mut self) -> Path2D {
        self.path.take().expect("cannot take Path2D before begin_path()!")
    }

    fn close_and_take_path(&mut self) -> Path2D {
        let mut path = self.take_path();
        path.close_path();
        path
    }
}

/// https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillStyle
fn parse_fill_style(s: String) -> anyhow::Result<FillStyle> {
    let color = s.parse::<csscolorparser::Color>()
        .map_err(|_| anyhow::anyhow!("could not parse fillStyle string {} to color", s))?
        .to_rgba8();

    Ok(FillStyle::Color(ColorU::new(color[0], color[1], color[2], color[3])))
}

fn parse_stroke_style(s: String) -> anyhow::Result<FillStyle> {
    let color = s.parse::<csscolorparser::Color>()
        .map_err(|_| anyhow::anyhow!("could not parse strokeStyle string {} to color", s))?
        .to_rgba8();

    Ok(FillStyle::Color(ColorU::new(color[0], color[1], color[2], color[3])))
}