use crate::gui;
use crate::macos_gui::{button, checkbox};

pub struct MacFactory;

impl gui::GuiFactory for MacFactory {
    type B = button::Button;

    type C = checkbox::Checkbox;

    fn create_button(&self) -> Self::B {
        button::Button {
            width: 300,
            height: 200,
        }
    }

    fn create_checkbox(&self) -> Self::C {
        checkbox::Checkbox { is_selected: true }
    }
}
