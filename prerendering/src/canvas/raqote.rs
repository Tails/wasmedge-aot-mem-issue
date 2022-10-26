use raqote::{DrawTarget, PathBuilder};
use crate::r#trait::*;

pub struct RaqoteCanvas {
    surface: DrawTarget,
    path: Option<PathBuilder>
}

impl HtmlCanvasApi for RaqoteCanvas {
    fn new(width: i32, height: i32) -> Self {
        Self {
            surface: DrawTarget::new(width, height),
            path: None
        }
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
        self.path.move_to(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.path.line_to(x, y);
    }

    fn quad_to(&mut self, cpx: f32, cpy: f32, x: f32, y: f32) {
        self.path.quad_to(cpx, cpy, x, y);
    }

    fn bezier_curve_to(&mut self, cp1x: f32, cp1y: f32, cp2x: f32, cp2y: f32, x: f32, y: f32) {
        self.path.c
    }

    fn close_path(&mut self) {
        if let Some(pb) = self.path.take() {
            let path = pb.finish();
            this.surface.push_clip(&path);
        }

        else {
            panic!("cant close non-existing path")
        }
    }

    fn begin_path(&mut self) {
        self.path = Some(PathBuilder::new());
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