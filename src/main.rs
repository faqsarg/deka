mod terminal;
pub use terminal::Terminal;
mod editor;
use editor::Editor;
pub use editor::Position;

fn main() {
    Editor::default().run();
}
