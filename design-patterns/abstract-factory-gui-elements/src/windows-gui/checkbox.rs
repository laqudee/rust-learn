use crate::gui;

pub struct Checkbox {
    pub is_selected: bool,
}

impl Checkbox {
    pub fn _new() -> Self {
        Self { is_selected: false }
    }
}

impl gui::Checkbox for Checkbox {
    fn switch(&self) {
        println!("Windows checkbox has switched, {}", self.is_selected);
    }
}
