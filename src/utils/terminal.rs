use anyhow::Result;
use crossterm::{
    cursor, execute,
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};

pub struct TerminalManager {
    width: u16,
    height: u16,
    original_state: bool,
}

impl TerminalManager {
    pub fn new() -> Result<Self> {
        let (width, height) = terminal::size()?;
        Ok(Self {
            width,
            height,
            original_state: false,
        })
    }

    pub fn setup(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        execute!(stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;
        self.original_state = true;
        Ok(())
    }

    pub fn cleanup(&mut self) -> Result<()> {
        if self.original_state {
            execute!(stdout(), cursor::Show, terminal::LeaveAlternateScreen)?;
            terminal::disable_raw_mode()?;
            self.original_state = false;
        }
        Ok(())
    }

    pub fn clear(&self) -> Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        Ok(())
    }

    pub fn move_to(&self, x: u16, y: u16) -> Result<()> {
        execute!(stdout(), cursor::MoveTo(x, y))?;
        Ok(())
    }

    pub fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    pub fn refresh_size(&mut self) -> Result<()> {
        let (width, height) = terminal::size()?;
        self.width = width;
        self.height = height;
        Ok(())
    }

    pub fn print_at(&self, x: u16, y: u16, text: &str) -> Result<()> {
        self.move_to(x, y)?;
        print!("{}", text);
        stdout().flush()?;
        Ok(())
    }

    pub fn print_centered(&self, text: &str) -> Result<()> {
        let lines: Vec<&str> = text.lines().collect();
        let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0) as u16;
        let height = lines.len() as u16;

        let start_x = (self.width.saturating_sub(max_width)) / 2;
        let start_y = (self.height.saturating_sub(height)) / 2;

        for (i, line) in lines.iter().enumerate() {
            let line_width = line.len() as u16;
            let x = start_x + (max_width.saturating_sub(line_width)) / 2;
            let y = start_y + i as u16;
            self.print_at(x, y, line)?;
        }

        Ok(())
    }
}

impl Drop for TerminalManager {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}
