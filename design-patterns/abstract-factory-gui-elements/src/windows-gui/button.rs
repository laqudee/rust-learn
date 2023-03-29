use crate::gui;

pub struct Button {
    pub width: u32,
    pub height: u32,
}

impl Button {
    pub fn _new(w: u32, h: u32) -> Self {
        Self {
            width: w,
            height: h,
        }
    }
}

impl gui::Button for Button {
    fn press(&self) {
        println!(
            "Window button has pressed, {}, {}",
            self.width, self.height
        )
    }
}
