use tui::{layout::Rect, Frame, backend::CrosstermBackend, text::{Spans, Span}, widgets::{Paragraph, Block, Borders}};

use crate::{backend::relation::{Relation}, ui::renderable::Renderable};

/// RelationPage struct handles the state required in order render a single relation
pub struct RelationPage<'a>{
    pub relation: &'a Relation
}

impl <'a>RelationPage<'a> {
    /// new associative function initializes RelationPage
    pub fn new(relation: &'a Relation) -> RelationPage<'a> {
        RelationPage{ relation: relation }
    }
}

impl <'a>Renderable for RelationPage<'a>{
    fn render<T: std::io::Write>(&self, display_area: Rect, frame: &mut Frame<CrosstermBackend<T>>) {
        let lines : Vec<Spans> = {
            vec![
                //define schema name line
                vec![Spans::from(self.relation.name())],

                //define relation definition lines
                match &self.relation {
                    Relation::Table(table) => {
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
                    },
                    Relation::View(_veiw) => todo!()
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