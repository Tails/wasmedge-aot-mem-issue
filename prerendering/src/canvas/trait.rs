use crate::canvas::rpc::CanvasRPC;

pub trait HtmlCanvasApi {
    fn new(width: i32, height: i32) -> Self;
    fn export(self, output: &str) -> anyhow::Result<()>;

    fn save(&mut self);
    fn restore(&mut self);

    fn translate(&mut self, dx: f32, dy: f32);
    fn scale(&mut self, sx: f32, sy: f32);
    fn resize(&mut self, w: f64, h: f64);

    fn move_to(&mut self, x: f32, y: f32);
    fn line_to(&mut self, x: f32, y: f32);
    fn bezier_curve_to(&mut self, cp1x: f32, cp1y: f32, cp2x: f32, cp2y: f32, x: f32, y: f32);
    fn quadratic_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64);
    fn arc(&mut self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64, anti_clockwise: bool);

    fn close_path(&mut self);
    fn begin_path(&mut self);
    
    fn stroke(&mut self);
    fn fill(&mut self);
    
    fn fill_style(&mut self, style: String);
    fn background_fill_style(&mut self, style: String);
    fn stroke_style(&mut self, style: String);
    fn font(&mut self, style: String);
    // todo: make more strongly typed
    fn shadow_color(&mut self, color: String);
    // todo: make more strongly typed
    fn line_cap(&mut self, cap: String);
    // pointer to array
    fn line_dash(&mut self, values: Vec<f64>);
    fn shadow_blur(&mut self, value: f64);
    fn fill_text(&mut self, text: String, x: f64, y: f64);
    fn line_width(&mut self, width: f64);

    fn rect(&mut self, x: f64, y: f64, w: f64, h: f64);
    fn fill_rect(&mut self, x: f64, y: f64, w: f64, h: f64);
    fn clear_rect(&mut self, x: f64, y: f64, w: f64, h: f64);

    fn call(&mut self, cmd: CanvasRPC) {
        match cmd {
            CanvasRPC::Quit
                => {},
            CanvasRPC::Init(_)
                => {}
            CanvasRPC::Save
                => self.save(),
            CanvasRPC::Restore
                => self.restore(),
            CanvasRPC::BeginPath
                => self.begin_path(),
            CanvasRPC::ClosePath
                => self.close_path(),
            CanvasRPC::Stroke
                => self.stroke(),
            CanvasRPC::Fill
                => self.fill(),
            CanvasRPC::Scale(w, h)
                => self.scale(w as f32, h as f32),
            CanvasRPC::Resize(w, h)
                => self.resize(w, h),
            CanvasRPC::MoveTo(x, y)
                => self.move_to(x as f32, y as f32),
            CanvasRPC::LineTo(x, y)
                => self.line_to(x as f32, y as f32),
            CanvasRPC::ShadowBlur(factor)
                => self.shadow_blur(factor),
            CanvasRPC::LineWidth(w)
                => self.line_width(w),
            CanvasRPC::FillRect(x, y, w, h)
                => self.fill_rect(x, y, w, h),
            CanvasRPC::Rect(x, y, w, h)
                => self.rect(x, y, w, h),
            CanvasRPC::ClearRect(x, y, w, h)
                => self.clear_rect(x, y, w, h),
            CanvasRPC::Arc(a, b, c, d, e, reverse)
                => self.arc(a, b, c, d, e, reverse),
            CanvasRPC::BezierCurveTo(x1, y1, x2, y2, x3, y3)
                => self.bezier_curve_to(x1 as f32, y1 as f32, x2 as f32, y2 as f32, x3 as f32, y3 as f32),
            CanvasRPC::QuadraticCurveTo(x1, y1, x2, y2)
                => self.quadratic_curve_to(x1, y1, x2, y2),
            CanvasRPC::FillText(text, x, y)
                => self.fill_text(text, x, y),
            CanvasRPC::FillStyle(style)
                => self.fill_style(style),
            CanvasRPC::BackgroundFillStyle(style)
                => self.background_fill_style(style),
            CanvasRPC::StrokeStyle(style)
                => self.stroke_style(style),
            CanvasRPC::Font(style)
                => self.font(style),
            CanvasRPC::ShadowColor(color)
                => self.shadow_color(color),
            CanvasRPC::LineCap(cap)
                => self.line_cap(cap),
            CanvasRPC::LineDash(values)
                => self.line_dash(values),
        }
    }
}