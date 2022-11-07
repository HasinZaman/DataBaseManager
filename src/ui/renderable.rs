use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    Frame
};

/// Renderable trait defines methods required to render a widget in a display area
pub trait Renderable {
    fn render<T: std::io::Write>(&self, display_area: Rect, frame: &mut Frame<CrosstermBackend<T>>);
}