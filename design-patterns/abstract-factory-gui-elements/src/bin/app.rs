#[path = "../gui/mod.rs"]
mod gui;

#[path = "../app-default/mod.rs"]
mod app_default;

use app_default::render::render;

#[path = "../macos-gui/mod.rs"]
mod macos_gui;
use macos_gui::factory::MacFactory;

#[path = "../windows-gui/mod.rs"]
mod windows_gui;
use windows_gui::factory::WindowsFactory;

fn main() {
    let windows = false;

    if windows {
        render(WindowsFactory);
    } else {
        render(MacFactory);
    }
}