use crate::parser::color::Color;
use crossterm::style::Color as CrosstermColor;

pub fn apply_color_to_char(ch: char, color: Color) -> String {
    use crossterm::style::Stylize;
    
    let crossterm_color = CrosstermColor::Rgb {
        r: color.r,
        g: color.g,
        b: color.b,
    };
    
    format!("{}", ch.to_string().with(crossterm_color))
}

pub fn apply_color_to_line(line: &str, colors: &[Color]) -> String {
    if colors.is_empty() {
        return line.to_string();
    }
    
    line.chars()
        .enumerate()
        .map(|(i, ch)| {
            if ch.is_whitespace() {
                ch.to_string()
            } else {
                let color = colors[i % colors.len()];
                apply_color_to_char(ch, color)
            }
        })
        .collect()
}

pub fn apply_gradient_to_text(text: &str, colors: &[Color]) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let total_chars: usize = lines.iter().map(|l| l.chars().count()).sum();
    
    if total_chars == 0 || colors.is_empty() {
        return text.to_string();
    }
    
    let mut result = String::new();
    let mut char_index = 0;
    
    for (line_idx, line) in lines.iter().enumerate() {
        for ch in line.chars() {
            if ch.is_whitespace() {
                result.push(ch);
            } else {
                let color_index = (char_index * colors.len()) / total_chars.max(1);
                let color = colors[color_index.min(colors.len() - 1)];
                result.push_str(&apply_color_to_char(ch, color));
                char_index += 1;
            }
        }
        
        if line_idx < lines.len() - 1 {
            result.push('\n');
        }
    }
    
    result
}