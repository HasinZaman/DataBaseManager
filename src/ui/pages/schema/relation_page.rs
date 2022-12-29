use tui::{layout::Rect, Frame, backend::CrosstermBackend, text::{Spans, Span}, widgets::{Paragraph, Block, Borders}};

use crate::{backend::relation::{Relation}, ui::renderable::Renderable};

/// RelationPage struct handles the state required in order render a single relation
pub struct RelationPage{
    pub relation: Relation
}

impl RelationPage {
    /// new associative function initializes RelationPage
    pub fn new(relation: &Relation) -> RelationPage{
        RelationPage{ relation: relation.clone() }
    }
}

impl Renderable for RelationPage{
    fn render<T: std::io::Write>(&self, display_area: Rect, frame: &mut Frame<CrosstermBackend<T>>) {
        let lines : Vec<Spans> = {
            vec![
                //define schema name line
                vec![Spans::from(self.relation.name())],

                //define relation definition lines
                match &self.relation {
                    Relation::Table(table) => table_ui(table),
                    Relation::View(view) => view_ui(display_area, view)
                }
            ].concat()
        };

        

        frame.render_widget(
            Paragraph::new(lines)
                .block(
                    Block::default().title("Relation Schema").borders(Borders::ALL)),
            display_area
        );
    }
}

fn view_ui(display_area: Rect, view: &crate::backend::relation::view::View) -> Vec<Spans> {
    let max_width = display_area.width - 2 - 2;
    let tmp = view.query.to_string()
        .split(" ")
        .map(
            |str: &str| {
                Span::from(str.to_string())
            }
        )
        .fold(
            vec![Spans::default()],
            |lines, word| {
                let mut lines = lines;
                let mut last =  lines.pop().unwrap();

                if last.width() + word.width() + 1 < max_width as usize {
                    last.0.push(Span::from(" "));
                    last.0.push(word);
                    lines.push(last);
                }
                else {
                    lines.push(last);
                    let last = Spans::from(vec![Span::from(" "), word]);
                    lines.push(last);
                }
                
                lines
            }
        );
    tmp
}

fn table_ui(table: &crate::backend::relation::table::Table) -> Vec<Spans> {
    table.attributes
        .iter()
        .enumerate()
        .map(|(index, attr)| {
            Spans::from(vec![ 
                {
                    if table.attributes.len() - 1 == index {//last attribute
                        Span::from("└")
                    }
                    else{
                        Span::from("├")
                    }
                },
                Span::from(" "),

                Span::from(attr.name.clone()),
                Span::from(" "),

                Span::from(attr.data_type.to_string()),
                Span::from(" "),

                {
                    if Some(index) == table.primary_key {
                        Span::from("Primary Key ")
                    }
                    else{
                        Span::from("")
                    }
                },
                Span::from({
                    let tmp: Vec<String> = attr.constraint
                        .iter()
                        .map(|c| format!("{}", c.to_string()))
                        .collect();

                    tmp.join(", ")
                })

            ])
        })
        .collect()
}