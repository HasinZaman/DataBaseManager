use tui::{Frame,backend::CrosstermBackend, layout::Rect};
use crate::ui::renderable::Renderable;

struct SnapShotPage{
}

impl Renderable for SnapShotPage{
    fn render<T: std::io::Write>(&self, display_area: Rect, frame: &mut Frame<CrosstermBackend<T>>) {
        todo!()
    }
}