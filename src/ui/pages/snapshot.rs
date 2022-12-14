use std::cmp::max;

use tui::{layout::{Constraint, Rect}, Frame, backend::CrosstermBackend, widgets::{Table, Row, Cell, Block, Borders}, text::Span};

use crate::{ui::renderable::Renderable, backend::snapshot::{SnapShotsFile, Error, SnapShot}};

#[derive(Default)]
pub struct SnapShotPage{
    snap_shots_file: SnapShotsFile
}

impl SnapShotPage{
    pub fn new(file: &str) -> Result<SnapShotPage, Error> {
        let file = SnapShotsFile::open(file)?;
        Ok(
            SnapShotPage{
                snap_shots_file: file
            }
        )
    }

    pub fn update(& mut self) {
        self.snap_shots_file.update();
    }

    pub fn add(& mut self, snap_shot: SnapShot) -> & mut Self{
        self.snap_shots_file
            .snap_shots
            .insert(snap_shot);
        
        self.save();
        self
    }

    pub fn del(& mut self, path: &str) -> Result<& mut Self, Error>{

        let del_snap_shot = {
            let val = self.snap_shots_file
            .snap_shots
            .iter()
            .filter(|s| s.path == path)
            .next();

            match val {
                Some(val) => Some(val.clone()),
                None => None
            }
        };

        match del_snap_shot {
            Some(val) => {
                self.snap_shots_file
                    .snap_shots
                    .remove(&val);
            
                    self.save();
                Ok(self)
            },
            None => Err(Error::PathDoesNotExist)
        }
    }

    fn save(&self) {
        if let Err(_err) = self.snap_shots_file.save() {
            //error pop notification
            todo!()
        }
    }

}

impl Renderable for SnapShotPage{
    fn render<T: std::io::Write>(&self, display_area: Rect, frame: &mut Frame<CrosstermBackend<T>>) {
        let mut widths: [Constraint;2] = [Constraint::Length(10), Constraint::Length(4)];

        let table: Table = Table::new(
                {
                    let rows: Vec<Row> =self.snap_shots_file
                        .snap_shots
                        .iter()
                        .map(|snap_shot| {
                            Row::new(
                                vec![
                                    Cell::from(
                                        Span::from(
                                            {
                                                let tmp_str = snap_shot.time_stamp.to_string();

                                                widths[0] = match widths[0] {
                                                    Constraint::Length(l) => {
                                                        Constraint::Length(max(l, tmp_str.len() as u16))
                                                    }
                                                    _ => {
                                                        panic!()
                                                    }
                                                };

                                                tmp_str
                                            }
                                            
                                        )
                                    ),
                                    Cell::from(
                                        Span::from(
                                            {
                                                let tmp_str = snap_shot.path.to_string();
                                                widths[1] = match widths[1] {
                                                    Constraint::Length(l) => {
                                                        Constraint::Length(max(l, tmp_str.len() as u16))
                                                    }
                                                    _ => {
                                                        panic!()
                                                    }
                                                };

                                                tmp_str
                                            }
                                        )
                                    ),
                                ]
                            )
                        })
                        .collect();

                    rows
                }
            ).header(
                Row::new(
                    vec![
                        Cell::from("Time Stamp"),
                        Cell::from("Path")
                    ]
                )
            ).widths(&widths)
            .column_spacing(3)
            .block(
                Block::default()
                    .title("Snap Shots")
                    .borders(Borders::ALL)
            );



        frame.render_widget(table, display_area);
    }
}