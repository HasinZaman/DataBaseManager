use tui::widgets::Widget;

pub trait Renderable {
    fn render(&self) -> Vec<Box<dyn Widget>>;
}