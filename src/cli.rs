use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(name = "piglet")]
#[command(about = "🐷 Animated and colorful figlet wrapper", long_about = None)]
pub struct PigletCli {
    /// Text to render with figlet
    #[arg(value_name = "TEXT")]
    pub text: String,
    
    /// Duration of animation (e.g., 3000ms, 0.3s, 0.5h, 5m)
    #[arg(short, long, default_value = "3s")]
    pub duration: String,
    
    /// Color palette (hex or CSS4 colors, comma-separated)
    /// Example: "#FF5733,#33FF57,#3357FF" or "red,green,blue"
    #[arg(short = 'p', long, value_delimiter = ',')]
    pub color_palette: Option<Vec<String>>,
    
    /// Color gradient (CSS4 gradient definition)
    /// Example: "linear-gradient(90deg, red, blue)"
    #[arg(short = 'g', long)]
    pub color_gradient: Option<String>,
    
    /// Motion easing function
    /// Options: linear, ease-in, ease-out, ease-in-out, ease-in-quad, 
    /// ease-out-quad, ease-in-out-quad, ease-in-cubic, ease-out-cubic,
    /// ease-in-out-cubic, ease-in-back, ease-out-back, ease-in-out-back,
    /// ease-in-elastic, ease-out-elastic, ease-in-out-elastic,
    /// ease-in-bounce, ease-out-bounce, ease-in-out-bounce
    #[arg(short = 'i', long, default_value = "ease-in-out")]
    pub motion_ease: String,
    
    /// Motion effect name
    /// Options: fade-in, fade-out, fade-in-out, slide-in-top, slide-in-bottom,
    /// slide-in-left, slide-in-right, scale-up, scale-down, pulse,
    /// bounce-in, bounce-out, typewriter, typewriter-reverse, wave,
    /// jello, color-cycle, rainbow, gradient-flow, rotate-in, rotate-out
    #[arg(short, long, default_value = "fade-in")]
    pub motion_effect: String,
    
    /// Figlet font
    #[arg(short = 'f', long)]
    pub font: Option<String>,
    
    /// Additional figlet options (use after --)
    /// Example: piglet "Text" -- -w 200 -c
    #[arg(last = true)]
    pub figlet_args: Vec<String>,
    
    /// Loop animation infinitely
    #[arg(short, long)]
    pub loop_animation: bool,
    
    /// Frame rate (fps)
    #[arg(long, default_value = "30")]
    pub fps: u32,
    
    /// List all available effects
    #[arg(long)]
    pub list_effects: bool,
    
    /// List all available easing functions
    #[arg(long)]
    pub list_easing: bool,
    
    /// List all available CSS4 colors
    #[arg(long)]
    pub list_colors: bool,
}