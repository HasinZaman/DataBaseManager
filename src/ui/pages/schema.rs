use tui::{Frame,backend::CrosstermBackend, layout::Rect};

use crate::ui::renderable::Renderable;

pub struct SchemaListPage{
}

impl Renderable for SchemaListPage{
    fn render<T: std::io::Write>(&self, display_area: Rect, frame: &mut Frame<CrosstermBackend<T>>) {
        todo!()
    }
}

pub struct QueryPage{
}

impl Renderable for QueryPage{
    fn render<T: std::io::Write>(&self, display_area: Rect, frame: &mut Frame<CrosstermBackend<T>>) {
        todo!()
    }
}