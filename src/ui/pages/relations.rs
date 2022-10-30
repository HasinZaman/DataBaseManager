use tui::{Frame,backend::CrosstermBackend, layout::Rect};

use crate::ui::renderable::Renderable;
use crate::backend::relation::Relation;

pub struct RelationListPage{
    relations: Vec<Relation>
}

impl Renderable for RelationListPage{
    fn render<T: std::io::Write>(&self, display_area: Rect, frame: &mut Frame<CrosstermBackend<T>>) {
        todo!()
    }
}

pub struct RelationPage{
    relation: Relation
}

impl Renderable for RelationPage{
    fn render<T: std::io::Write>(&self, display_area: Rect, frame: &mut Frame<CrosstermBackend<T>>) {
        todo!()
    }
}