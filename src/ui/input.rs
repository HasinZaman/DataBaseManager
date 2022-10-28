use std::{fmt, cmp::min};

use tui::{widgets::{Block, Borders, Paragraph}, text::Spans};

use super::renderable::Renderable;

pub enum InputErr{
    InvalidCursorIndex
}

pub struct Input{
    content: Vec<char>,
    prompt: Option<String>,
    cursor: usize,
    input_cond: bool,
}

impl Input {
    pub fn new(content: Vec<char>, prompt: Option<String>, cursor: usize, input_cond: bool) -> Result<Input, InputErr>{
        Result::Ok(
            Input{
                cursor: {
                    if content.len() <= cursor{
                        return Result::Err(InputErr::InvalidCursorIndex)
                    }
                    cursor
                },
                content: content,
                prompt: prompt,
                input_cond: input_cond,
            }
        )
    }

    pub fn from(content: String, prompt: Option<String>, input_cond: bool) -> Input {
        Input {
            content: content.chars().collect(),
            prompt: prompt,
            cursor: content.len(),
            input_cond: input_cond,
        }
    }

    pub fn clear(&mut self) -> &mut Self{
        if self.input_cond  {
            self.content.clear();
            self.cursor = 0;
        }
        self
    }

    pub fn cursor_left(&mut self, step: usize) -> &mut Self{
        match self.cursor.checked_sub(step) {
            Some(val) => self.cursor = val,
            None => self.cursor = 0,
        }
        self
    }
    pub fn cursor_right(&mut self, step: usize) -> &mut Self{
        self.cursor = min(self.content.len()-1, self.cursor + step);

        self
    }
    pub fn cursor(&self)  -> usize {
        self.cursor
    }

    pub fn add_char(&mut self, c: char) -> &mut Self {
        if self.input_cond {
            self.content.insert(self.cursor, c);
            self.cursor+=1;
        }
        self
    }

    pub fn del_char(&mut self) -> &mut Self {
        if self.input_cond {
            if self.cursor > 0 {
                self.content.remove(self.cursor);
                self.cursor-=1;
            }
        }
        self
    }
}
impl Default for Input{
    fn default() -> Input {
        Input::from("".to_string(), Option::None, true)
    }
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let input_str: String = self.content.iter().collect();

        write!(f, "str: {}\tcursor:{}", input_str, self.cursor)
    }
}

impl Renderable for Input{
    fn render<T: std::io::Write>(&self, display_area: tui::layout::Rect, frame: &mut tui::Frame<tui::backend::CrosstermBackend<T>>) {
        let text_area = {
            let block: Block = {
                let mut block: Block = Block::default()
                    .borders(Borders::ALL);

                if let Some(val) = &self.prompt {
                    block = block.title(val.clone());
                }

                block
            }; 
                
            let inner = block.inner(display_area);

            frame.render_widget(block, display_area);

            inner
        };

        let content : Paragraph = Paragraph::new(
            {
                vec![
                    {
                        let max_char: usize = text_area.width as usize - 1;

                        let content = self.content.clone();

                        if self.content.len() < self.cursor {
                            panic!("Invalid cursor position");
                        }

                        let content_str: String;

                        if content.len() < max_char {
                            content_str = content.iter()
                                .skip(0)
                                .take(self.cursor)
                                .collect();

                            frame.set_cursor(text_area.x + self.cursor as u16, text_area.y);
                        }
                        else if self.cursor <= max_char {
                            content_str = content.iter()
                                .skip(0)
                                .take(max_char)
                                .collect();

                            frame.set_cursor(text_area.x + text_area.width - 1 as u16, text_area.y);
                        }
                        else {
                            let tmp_content_str : String = content.iter()
                                .skip(self.cursor - max_char)
                                .take(max_char)
                                .collect();
                            
                            content_str = format!("<{}", tmp_content_str);
                        }

                        Spans::from(content_str)
                    }
                    
                ]
            }
        );

        frame.render_widget(content, text_area);
    }
}