use std::io::{stdout, Stdout};

use crossterm::{cursor::{Hide, MoveTo, Show}, execute, queue, style::{Color, Print}, terminal::{disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen}};
use crate::errors::Result;

#[derive(Clone, Copy)]
struct Cell {
    ch: char,
    fg: Color,
    bg: Color,
}

impl Cell {
    fn new() -> Self {
        Self {
            ch: ' ',
            fg: Color::Reset,
            bg: Color::Reset,
        }
    }
}

pub struct Tui {
    pub width: usize,
    buffer: Vec<Vec<Cell>>,
    stdout: Stdout,
}

impl Tui {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        let stdout = stdout();
        
        execute!(&stdout, EnterAlternateScreen, Hide)?;
        
        let (width, height) = size()?;

        let buffer = vec![vec![Cell::new(); width as usize]; height as usize];

        Ok(Self {
            width: width as usize,
            buffer,
            stdout,
        })
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.buffer = vec![vec![Cell::new(); width as usize]; height as usize];
        self.width = width;
    }

    // pub fn print_ch(&mut self, ch: char, x: usize, y: usize) {
    //     assert!(y < self.buffer.len());
    //     assert!(x < self.buffer[y].len());
        
    //     self.buffer[y][x].ch = ch;
    // }
    
    pub fn print(&mut self, line: &str, x: usize, y: usize) {
        assert!(y < self.buffer.len());
        assert!(x < self.buffer[y].len());
        assert!(x + line.len() < self.buffer[y].len());

        for (i, ch) in line.chars().enumerate() {
            self.buffer[y][x + i].ch = ch;
        }
    }

    // TODO: Coloring

    pub fn flush(&self) -> Result<()> {
        let mut display_line = String::with_capacity(self.width);
        let mut row = 0;

        for line in &self.buffer {
            for cell in line {
                display_line.push(cell.ch);
            }
            queue!(&self.stdout, MoveTo(0, row), Print(&display_line))?;
            display_line.clear();

            row += 1;
        }

        Ok(())
    }

    pub fn clear(&mut self) {
        for line in self.buffer.iter_mut() {
            for cell in line.iter_mut() {
                cell.ch = ' ';
                cell.fg = Color::Reset;
                cell.bg = Color::Reset;
            }
        }
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        execute!(self.stdout, LeaveAlternateScreen, Show).expect("Failed to leave alternate screen.");
        disable_raw_mode().expect("Failed to disable raw mode.");
    }
}