use anyhow::{bail, Context, Result};
use std::process::Command;
use which::which;

pub struct FigletWrapper {
    font: Option<String>,
    args: Vec<String>,
}

impl FigletWrapper {
    pub fn new() -> Self {
        Self {
            font: None,
            args: Vec::new(),
        }
    }

    pub fn with_font(mut self, font: Option<&str>) -> Self {
        self.font = font.map(|s| s.to_string());
        self
    }

    pub fn with_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }

    pub fn render(&self, text: &str) -> Result<String> {
        let mut cmd = Command::new("figlet");

        // Add font if specified
        if let Some(font) = &self.font {
            cmd.arg("-f").arg(font);
        }

        // Add additional arguments
        for arg in &self.args {
            cmd.arg(arg);
        }

        // Add the text
        cmd.arg(text);

        // Execute and capture output
        let output = cmd.output().context("Failed to execute figlet")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Figlet error: {}", stderr);
        }

        let result =
            String::from_utf8(output.stdout).context("Figlet output is not valid UTF-8")?;

        Ok(result)
    }

    pub fn check_installed() -> Result<()> {
        which("figlet").context(
            "figlet not found. Please install figlet first.\n\
                     On Ubuntu/Debian: sudo apt-get install figlet\n\
                     On macOS: brew install figlet\n\
                     On Arch: sudo pacman -S figlet",
        )?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn list_fonts() -> Result<Vec<String>> {
        let output = Command::new("figlet")
            .arg("-l")
            .output()
            .context("Failed to list figlet fonts")?;

        if !output.status.success() {
            bail!("Failed to list fonts");
        }

        let result = String::from_utf8_lossy(&output.stdout);
        let fonts: Vec<String> = result
            .lines()
            .skip(1) // Skip header
            .filter_map(|line| line.split_whitespace().next().map(|s| s.to_string()))
            .collect();

        Ok(fonts)
    }
}

impl Default for FigletWrapper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_figlet_installed() {
        // This test will fail if figlet is not installed
        assert!(FigletWrapper::check_installed().is_ok());
    }

    #[test]
    fn test_basic_render() {
        let figlet = FigletWrapper::new();
        let result = figlet.render("Hi");
        assert!(result.is_ok());
        let ascii = result.unwrap();
        assert!(!ascii.is_empty());
        assert!(ascii.contains("H") || ascii.contains("_") || ascii.contains("|"));
    }
}
