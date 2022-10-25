use tui::{Frame, backend::Backend};

pub trait Renderable<B> where B: Backend {
    fn render<'a>(frame: &mut Frame<B>)  where B: 'a;
}