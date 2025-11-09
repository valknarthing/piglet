mod animation;
mod cli;
mod color;
mod figlet;
mod parser;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::PigletCli;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments
    let args = PigletCli::parse();

    // Show banner on first run
    if std::env::args().len() == 1 {
        show_welcome();
        return Ok(());
    }

    // Verify figlet is installed
    figlet::FigletWrapper::check_installed()?;

    // Run the piglet magic
    run_piglet(args).await?;

    Ok(())
}

async fn run_piglet(args: PigletCli) -> Result<()> {
    use crate::animation::AnimationEngine;
    use crate::color::ColorEngine;
    use crate::utils::terminal::TerminalManager;

    // Parse duration
    let duration_ms = parser::duration::parse_duration(&args.duration)?;

    // Create figlet wrapper and render base ASCII art
    let figlet = figlet::FigletWrapper::new()
        .with_font(args.font.as_deref())
        .with_args(args.figlet_args);

    let ascii_art = figlet.render(&args.text)?;

    // Setup color engine
    let color_engine = ColorEngine::new()
        .with_palette(args.color_palette.as_deref())?
        .with_gradient(args.color_gradient.as_deref())?;

    // Setup animation engine
    let animation_engine = AnimationEngine::new(ascii_art, duration_ms, args.fps)
        .with_effect(&args.motion_effect)?
        .with_easing(&args.motion_ease)?
        .with_color_engine(color_engine);

    // Setup terminal
    let mut terminal = TerminalManager::new()?;
    terminal.setup()?;

    // Run animation
    loop {
        animation_engine.run(&mut terminal).await?;

        if !args.loop_animation {
            break;
        }
    }

    // Cleanup
    terminal.cleanup()?;

    Ok(())
}

fn show_welcome() {
    println!(
        r"
    ____  _       __     __
   / __ \(_)___ _/ /__  / /_
  / /_/ / / __ `/ / _ \/ __/
 / ____/ / /_/ / /  __/ /_
/_/   /_/\__, /_/\___/\__/
        /____/

🐷 Piglet - Animated Figlet Wrapper

Usage: piglet [TEXT] [OPTIONS]

Examples:
  piglet Hello -p red,blue,green
  piglet World -g linear-gradient(90deg, red, blue) -e fade-in
  piglet Cool! -e typewriter -d 2s -i ease-out

Run 'piglet --help' for more information.
"
    );
}
