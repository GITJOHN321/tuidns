use std::time::{Duration, Instant};

pub struct TextCursor {
    pub pos: usize,
    visible: bool,
    timer: Instant,
}

impl TextCursor {
    pub fn new() -> Self {
        Self {
            pos: 0,
            visible: true,
            timer: Instant::now(),
        }
    }

    pub fn tick(&mut self) {
        if self.timer.elapsed() >= Duration::from_millis(500) {
            self.visible = !self.visible;
            self.timer = Instant::now();
        }
    }

    pub fn glyph(&self) -> &'static str {
        if self.visible { "|" } else { " " }
    }

    pub fn render<'a>(&self, input: &'a str) -> String {
        let (before, after) = input.split_at(self.pos);
        format!("{}{}{}", before, self.glyph(), after)
    }
}
