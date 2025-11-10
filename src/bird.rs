use raylib::prelude::*;

pub struct Bird {
    pub pos: Vector2,
    pub size: f32,
}

impl Bird {
    pub fn new(pos: Vector2) -> Self {
        Self {
            pos,
            size: 5.0
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        // V shape

        let top_left = Vector2::new(40.0, 20.0) + self.pos;
        let bottom = Vector2::new(55.0, 30.0) + self.pos;
        let top_right = Vector2::new(top_left.x + 30.0, top_left.y);

        let thickness = 4.0;
        d.draw_line_ex(top_left, bottom, thickness, Color::WHITE);
        d.draw_line_ex(top_right, bottom, thickness, Color::WHITE);
    }
}
