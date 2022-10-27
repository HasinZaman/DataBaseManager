use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    Frame
};

// use crossterm::{
//     execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen}
// };
pub trait Renderable {
    fn render<T: std::io::Write>(&self, display_area: Rect, frame: &mut Frame<CrosstermBackend<T>>);
}