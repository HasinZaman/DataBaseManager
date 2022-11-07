use std::{fmt, cmp::min};

use tui::{widgets::{Block, Borders, Paragraph}, text::Spans};

use super::renderable::Renderable;

/// InputErr defines all the possible error from the Input widget
#[derive(Debug)]
pub enum InputErr{
    InvalidCursorIndex
}

/// Input struct defines the state required to maintain a text input section
#[derive(Debug)]
pub struct Input{
    content: Vec<char>,
    pub prompt: Option<String>,
    cursor: usize,
    pub input_cond: bool,
}

impl Input {
    /// new associative function generates Input struct
    pub fn new(content: Vec<char>, prompt: Option<String>, cursor: usize, input_cond: bool) -> Result<Input, InputErr>{
        Result::Ok(
            Input{
                cursor: {
                    if content.len() < cursor{
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

    /// from associative function generates Input struct from String
    pub fn from(content: String, prompt: Option<String>, input_cond: bool) -> Input {
        Input {
            content: content.chars().collect(),
            prompt: prompt,
            cursor: content.len(),
            input_cond: input_cond,
        }
    }

    /// clear method empties input
    pub fn clear(&mut self) -> &mut Self{
        if self.input_cond  {
            self.content.clear();
            self.cursor = 0;
        }
        self
    }

    /// cursor_left method moves the cursor to the left by step
    pub fn cursor_left(&mut self, step: usize) -> &mut Self{
        if self.input_cond {
            match self.cursor.checked_sub(step) {
                Some(val) => self.cursor = val,
                None => self.cursor = 0,
            }
        }
        
        self
    }

    /// cursor_right method moves the cursor to the right by step
    pub fn cursor_right(&mut self, step: usize) -> &mut Self{
        if self.input_cond {
            self.cursor = min(self.content.len(), self.cursor + step);
        }

        self
    }

    /// cursor method returns cursor position
    pub fn cursor(&self)  -> usize {
        self.cursor
    }

    /// add_char method adds char left of the cursor
    pub fn add_char(&mut self, c: char) -> &mut Self {
        if self.input_cond {
            self.content.insert(self.cursor, c);
            self.cursor+=1;
        }
        self
    }

    /// del_char method removes char left of the cursor
    pub fn del_char(&mut self) -> &mut Self {
        if self.input_cond {
            if let Some(target) = self.cursor.checked_sub(1) {
                self.content.remove(target);
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

        match &self.prompt {
            Some(val) => write!(f, "{:?} - \"{}\"\tcursor:{}", val, input_str, self.cursor),
            None => write!(f, "\"{}\"\tcursor:{}", input_str, self.cursor),
        }
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

mod tests{
    use super::*;

    #[test]
    fn new_1_test() {
        let actual = Input::new(Vec::new(), None, 0, true);
        let expected:Result<Input, InputErr> = Ok(Input { 
                content: Vec::new(),
                prompt: None,
                cursor: 0,
                input_cond: true,
            });

        assert_eq!(
            format!("{:?}", expected),
            format!("{:?}", actual)
        );
    }

    #[test]
    fn new_2_test() {
        let actual = Input::new(vec!['a','b','c'], None, 0, true);
        let expected:Result<Input, InputErr> = Ok(Input { 
                content: vec!['a','b','c'],
                prompt: None,
                cursor: 0,
                input_cond: true,
            });

        assert_eq!(
            format!("{:?}", expected),
            format!("{:?}", actual)
        );
    }

    #[test]
    fn new_3_test() {
        let actual = Input::new(vec!['a','b','c'], None, 2, true);
        let expected:Result<Input, InputErr> = Ok(Input { 
                content: vec!['a','b','c'],
                prompt: None,
                cursor: 2,
                input_cond: true,
            });

        assert_eq!(
            format!("{:?}", expected),
            format!("{:?}", actual)
        );
    }
    
    #[test]
    fn new_4_test() {
        let actual = Input::new(vec!['a','b','c'], None, 3, true);
        let expected:Result<Input, InputErr> = Ok(Input { 
                content: vec!['a','b','c'],
                prompt: None,
                cursor: 3,
                input_cond: true,
            });

        assert_eq!(
            format!("{:?}", expected),
            format!("{:?}", actual)
        );
    }

    #[test]
    fn new_5_test() {
        let actual = Input::new(vec!['a','b','c'], Some("prompt".to_string()), 3, true);
        let expected:Result<Input, InputErr> = Ok(Input { 
                content: vec!['a','b','c'],
                prompt: Some("prompt".to_string()),
                cursor: 3,
                input_cond: true,
            });

        assert_eq!(
            format!("{:?}", expected),
            format!("{:?}", actual)
        );
    }

    #[test]
    fn new_6_test() {
        let actual = Input::new(vec!['a','b','c'], None, 4, true);
        let expected:Result<Input, InputErr> = Err(InputErr::InvalidCursorIndex);

        assert_eq!(
            format!("{:?}", expected),
            format!("{:?}", actual)
        );
    }

    #[test]
    fn from_1_test() {
        let actual = Input::from("abc".to_string(), None, true);
        let expected = Input { 
            content: vec!['a','b','c'],
            prompt: None,
            cursor: 3,
            input_cond: true,
        };

        assert_eq!(
            format!("{:?}", expected),
            format!("{:?}", actual)
        )
    }
    
    #[test]
    fn from_2_test() {
        let actual = Input::from("abc".to_string(), Some("prompt".to_string()), true);
        let expected = Input { 
            content: vec!['a','b','c'],
            prompt: Some("prompt".to_string()),
            cursor: 3,
            input_cond: true,
        };

        assert_eq!(
            format!("{:?}", expected),
            format!("{:?}", actual)
        )
    }

    #[test]
    fn from_3_test() {
        let actual = Input::from("".to_string(), Some("prompt".to_string()), true);
        let expected = Input { 
            content: Vec::new(),
            prompt: Some("prompt".to_string()),
            cursor: 0,
            input_cond: true,
        };

        assert_eq!(
            format!("{:?}", expected),
            format!("{:?}", actual)
        )
    }

    #[test]
    fn clear_1_test() {
        let mut actual = Input::new(Vec::new(), None, 0, true).unwrap();
        let expected: Input = Input { 
                content: Vec::new(),
                prompt: None,
                cursor: 0,
                input_cond: true,
            };

        assert_eq!(
            format!("{:?}", expected),
            format!("{:?}", actual.clear())
        );
    }
    
    #[test]
    fn clear_2_test() {
        let mut actual = Input::new(vec!['a','b','c'], None, 0, true).unwrap();
        let expected: Input = Input { 
                content: Vec::new(),
                prompt: None,
                cursor: 0,
                input_cond: true,
            };

        assert_eq!(
            format!("{:?}", expected),
            format!("{:?}", actual.clear())
        );
    }

    #[test]
    fn clear_3_test() {
        let mut actual = Input::new(vec!['a','b','c'], None, 0, false).unwrap();
        let expected: Input = Input { 
                content: vec!['a','b','c'],
                prompt: None,
                cursor: 0,
                input_cond: false,
            };

        assert_eq!(
            format!("{:?}", expected),
            format!("{:?}", actual.clear())
        );
    }

    #[test]
    fn cursor_1_test() {
        let mut actual = Input::from("Hello World".to_string(), None, true);

        assert_eq!(actual.cursor(), 11);

        actual.cursor_left(1);
        assert_eq!(actual.cursor(), 10);

        actual.cursor_right(1);
        assert_eq!(actual.cursor(), 11);

        actual.cursor_left(2);
        assert_eq!(actual.cursor(), 9);

        actual.cursor_left(5);
        assert_eq!(actual.cursor(), 4);
    }

    #[test]
    fn cursor_2_test() {
        let mut actual = Input::from("Hello World".to_string(), None, true);
        actual.cursor_left(actual.cursor());
        assert_eq!(actual.cursor(), 0);

        actual.cursor_left(1);
        assert_eq!(actual.cursor(), 0);
    }
    
    #[test]
    fn cursor_3_test() {
        let mut actual = Input::from("Hello World".to_string(), None, true);
        actual.cursor_right(1);
        assert_eq!(actual.cursor(), 11);
    }
    #[test]
    fn cursor_4_test() {
        let mut actual = Input::from("Hello World".to_string(), None, false);

        assert_eq!(actual.cursor(), 11);

        actual.cursor_left(1);
        assert_eq!(actual.cursor(), 11);

        actual.cursor_right(1);
        assert_eq!(actual.cursor(), 11);
    }

    #[test]
    fn add_char_1_test() {
        let mut actual = Input::from("".to_string(), None, true);

        assert_eq!(
            "\"\"\tcursor:0",
            actual.to_string()
        );

        actual.add_char('a');

        assert_eq!(
            "\"a\"\tcursor:1",
            actual.to_string()
        );

        actual.add_char('a').add_char('a');

        assert_eq!(
            "\"aaa\"\tcursor:3",
            actual.to_string()
        );

        actual.add_char('a')
            .cursor_left(2)
            .add_char('b');

        assert_eq!(
            "\"aabaa\"\tcursor:3",
            actual.to_string()
        );

        actual.cursor_left(3)
            .add_char('c')
            .cursor_left(1)
            .add_char('d');

        assert_eq!(
            "\"dcaabaa\"\tcursor:1",
            actual.to_string()
        );

        actual.input_cond = false;
        actual.add_char('e');

        assert_eq!(
            "\"dcaabaa\"\tcursor:1",
            actual.to_string()
        );
    }

    #[test]
    fn del_char_1_test() {
        let mut actual = Input::from("123456".to_string(), None, true);

        assert_eq!(
            "\"123456\"\tcursor:6",
            actual.to_string()
        );

        actual.del_char();
        assert_eq!(
            "\"12345\"\tcursor:5",
            actual.to_string()
        );

        actual.del_char();
        assert_eq!(
            "\"1234\"\tcursor:4",
            actual.to_string()
        );

        actual.cursor_left(2).del_char();
        assert_eq!(
            "\"134\"\tcursor:1",
            actual.to_string()
        );
        
        actual.cursor_left(2).del_char();
        assert_eq!(
            "\"134\"\tcursor:0",
            actual.to_string()
        );
        
        actual.cursor_right(2).del_char();
        assert_eq!(
            "\"14\"\tcursor:1",
            actual.to_string()
        );

        actual.input_cond = false;
        actual.del_char();
        assert_eq!(
            "\"14\"\tcursor:1",
            actual.to_string()
        );
    }
}