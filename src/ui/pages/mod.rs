use super::renderable::Renderable;

pub mod relations;
pub mod schema;
pub mod snapshot;

pub enum Pages {
    RelationList(relations::relation_list::RelationListPage),
    Relation(relations::relation_page::RelationPage),
    Query(schema::QueryPage),
    SnapShot(snapshot::SnapShotPage)
}

impl Renderable for Pages{
    fn render<T: std::io::Write>(&self, display_area: tui::layout::Rect, frame: &mut tui::Frame<tui::backend::CrosstermBackend<T>>) {
        match self {
            Pages::RelationList(val) => val.render(display_area, frame),
            Pages::Relation(val) => val.render(display_area, frame),
            Pages::Query(val) => val.render(display_area, frame),
            Pages::SnapShot(val) => val.render(display_area, frame),
        }
    }
}