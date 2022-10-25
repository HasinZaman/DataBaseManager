
pub enum Tab {
    Schema,
    Relation,
    SnapShot
}

impl Tab{
}

impl Default for Tab {
    fn default() -> Tab {
        Tab::Schema
    }
}

// impl Renderable for Tab {
//     fn render<'a>(frame: &mut Frame<T>)  where T: 'a{

//     }
// }