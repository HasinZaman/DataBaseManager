use super::renderable::Renderable;

pub mod schema;
pub mod query;
pub mod snapshot;

pub enum Pages {
    RelationList(schema::relation_list::RelationListPage),
    Relation(schema::relation_page::RelationPage),
    Query(query::QueryPage),
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