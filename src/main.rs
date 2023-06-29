#![warn(clippy::all, clippy::pedantic)]

mod editor;
mod terminal;
use editor::Editor;
pub use terminal::Terminal; // re-exporting the `Terminal` here.

fn main() {
	Editor::default().run();
}
