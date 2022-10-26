use crate::canvas::r#trait::HtmlCanvasApi;

pub struct DummyCanvas {
    nothing: u32
}

impl HtmlCanvasApi for DummyCanvas {
    fn new(width: i32, height: i32) -> Self {
        todo!()
    }

    fn save(&mut self) {
        todo!()
    }

    fn translate(&mut self, dx: f32, dy: f32) {
        todo!()
    }

    fn scale(&mut self, sx: f32, sy: f32) {
        todo!()
    }

    fn move_to(&mut self, x: f32, y: f32) {
        todo!()
    }

    fn line_to(&mut self, x: f32, y: f32) {
        todo!()
    }

    fn quad_to(&mut self, cpx: f32, cpy: f32, x: f32, y: f32) {
        todo!()
    }

    fn bezier_curve_to(&mut self, cp1x: f32, cp1y: f32, cp2x: f32, cp2y: f32, x: f32, y: f32) {
        todo!()
    }

    fn close_path(&mut self) {
        todo!()
    }

    fn begin_path(&mut self) {
        todo!()
    }

    fn stroke(&mut self) {
        todo!()
    }

    fn fill(&mut self) {
        todo!()
    }

    fn set_line_width(&mut self, width: f32) {
        todo!()
    }
}