use crate::gui;
use crate::windows_gui::{button, checkbox};

pub struct WindowsFactory;

impl gui::GuiFactory for WindowsFactory {
    type B = button::Button;

    type C = checkbox::Checkbox;
    
    fn create_button(&self) -> Self::B {
        button::Button {
            width: 200,
            height: 100,
        }
    }

    fn create_checkbox(&self) -> Self::C {
        checkbox::Checkbox { is_selected: false }
    }
}
