use tui::{layout::Rect, Frame, backend::CrosstermBackend};

use crate::{backend::relation::Relation, ui::renderable::Renderable};

pub struct RelationPage{
    pub relation: Relation
}

impl RelationPage {
    fn new(relation: Relation) -> RelationPage {
        RelationPage { relation: relation }
    }
}

impl Renderable for RelationPage{
    fn render<T: std::io::Write>(&self, display_area: Rect, frame: &mut Frame<CrosstermBackend<T>>) {
        todo!()
    }
}