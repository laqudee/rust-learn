mod copy;
mod paste;
mod cut;

pub use copy::CopyCommand;
pub use paste::PasteCommand;
pub use cut::CutCommand;

use cursive::Cursive;

/// Command Interface
pub trait Command {
    fn execute(&mut self, app: &mut Cursive) -> bool;
    fn undo(&mut self, app: &mut Cursive);
}