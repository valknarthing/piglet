#[derive(Debug, Clone)]
pub struct AsciiArt {
    lines: Vec<String>,
    width: usize,
    height: usize,
}

impl AsciiArt {
    pub fn new(text: String) -> Self {
        let lines: Vec<String> = text.lines().map(|s| s.to_string()).collect();
        let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
        let height = lines.len();

        Self {
            lines,
            width,
            height,
        }
    }

    pub fn get_lines(&self) -> &[String] {
        &self.lines
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn render(&self) -> String {
        self.lines.join("\n")
    }

    /// Get character at position
    #[allow(dead_code)]
    pub fn char_at(&self, x: usize, y: usize) -> Option<char> {
        self.lines.get(y)?.chars().nth(x)
    }

    /// Count non-whitespace characters
    pub fn char_count(&self) -> usize {
        self.lines
            .iter()
            .flat_map(|line| line.chars())
            .filter(|c| !c.is_whitespace())
            .count()
    }

    /// Get all character positions
    pub fn char_positions(&self) -> Vec<(usize, usize, char)> {
        let mut positions = Vec::new();

        for (y, line) in self.lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if !ch.is_whitespace() {
                    positions.push((x, y, ch));
                }
            }
        }

        positions
    }

    /// Apply fade effect (0.0 = invisible, 1.0 = visible)
    pub fn apply_fade(&self, opacity: f64) -> String {
        if opacity >= 1.0 {
            return self.render();
        }

        if opacity <= 0.0 {
            return " ".repeat(self.width).repeat(self.height);
        }

        // For ASCII, we can simulate fade by replacing chars with lighter ones
        let fade_chars = [' ', '.', '·', '-', '~', '=', '+', '*', '#', '@'];
        let index = (opacity * (fade_chars.len() - 1) as f64) as usize;
        let fade_char = fade_chars[index];

        self.lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|ch| if ch.is_whitespace() { ch } else { fade_char })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Scale the ASCII art
    pub fn scale(&self, factor: f64) -> Self {
        if factor <= 0.0 {
            return Self::new(String::new());
        }

        if (factor - 1.0).abs() < 0.01 {
            return self.clone();
        }

        // Simple scaling by character repetition
        let lines: Vec<String> = if factor > 1.0 {
            self.lines
                .iter()
                .flat_map(|line| {
                    let scaled_line: String = line
                        .chars()
                        .flat_map(|ch| std::iter::repeat_n(ch, factor as usize))
                        .collect();
                    std::iter::repeat_n(scaled_line, factor as usize)
                })
                .collect()
        } else {
            self.lines
                .iter()
                .step_by((1.0 / factor) as usize)
                .map(|line| line.chars().step_by((1.0 / factor) as usize).collect())
                .collect()
        };

        Self::new(lines.join("\n"))
    }
}
