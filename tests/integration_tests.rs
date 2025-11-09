use anyhow::Result;
use piglet::{
    animation::easing::get_easing_function,
    animation::effects::get_effect,
    color::{palette::ColorPalette, ColorEngine},
    figlet::FigletWrapper,
    parser::{color::Color, duration::parse_duration, gradient::Gradient},
};

#[test]
fn test_figlet_wrapper() -> Result<()> {
    let figlet = FigletWrapper::new();
    let result = figlet.render("Test")?;
    assert!(!result.is_empty());
    Ok(())
}

#[test]
fn test_duration_parser() -> Result<()> {
    assert_eq!(parse_duration("1000ms")?, 1000);
    assert_eq!(parse_duration("1s")?, 1000);
    assert_eq!(parse_duration("1m")?, 60000);
    assert_eq!(parse_duration("1h")?, 3600000);
    assert_eq!(parse_duration("0.5s")?, 500);
    Ok(())
}

#[test]
fn test_color_parser() -> Result<()> {
    let color = Color::parse("#FF5733")?;
    assert_eq!(color.r, 255);
    assert_eq!(color.g, 87);
    assert_eq!(color.b, 51);

    let color = Color::parse("red")?;
    assert_eq!(color.r, 255);
    assert_eq!(color.g, 0);
    assert_eq!(color.b, 0);

    Ok(())
}

#[test]
fn test_gradient_parser() -> Result<()> {
    let gradient = Gradient::parse("linear-gradient(90deg, red, blue)")?;
    assert_eq!(gradient.stops.len(), 2);

    let gradient =
        Gradient::parse("linear-gradient(to right, #FF5733 0%, #33FF57 50%, #3357FF 100%)")?;
    assert_eq!(gradient.stops.len(), 3);
    assert_eq!(gradient.stops[0].position, 0.0);
    assert_eq!(gradient.stops[1].position, 0.5);
    assert_eq!(gradient.stops[2].position, 1.0);

    Ok(())
}

#[test]
fn test_color_interpolation() {
    let red = Color::new(255, 0, 0);
    let blue = Color::new(0, 0, 255);
    let purple = red.interpolate(&blue, 0.5);

    assert_eq!(purple.r, 127);
    assert_eq!(purple.g, 0);
    assert_eq!(purple.b, 127);
}

#[test]
fn test_color_palette() -> Result<()> {
    let palette =
        ColorPalette::from_strings(&["red".to_string(), "green".to_string(), "blue".to_string()])?;

    assert_eq!(palette.len(), 3);

    let color = palette.get_color(0);
    assert_eq!(color.r, 255);
    assert_eq!(color.g, 0);
    assert_eq!(color.b, 0);

    Ok(())
}

#[test]
fn test_easing_functions() -> Result<()> {
    let linear = get_easing_function("linear")?;
    assert_eq!(linear.ease(0.5), 0.5);

    let ease_in = get_easing_function("ease-in")?;
    let result = ease_in.ease(0.5);
    assert!((0.0..=1.0).contains(&result));

    let ease_out_bounce = get_easing_function("ease-out-bounce")?;
    let result = ease_out_bounce.ease(0.5);
    assert!((0.0..=1.5).contains(&result)); // Bounce can overshoot

    Ok(())
}

#[test]
fn test_effects() -> Result<()> {
    let fade_in = get_effect("fade-in")?;
    assert_eq!(fade_in.name(), "fade-in");

    let typewriter = get_effect("typewriter")?;
    assert_eq!(typewriter.name(), "typewriter");

    let bounce = get_effect("bounce-in")?;
    assert_eq!(bounce.name(), "bounce-in");

    Ok(())
}

#[test]
fn test_color_engine() -> Result<()> {
    let engine = ColorEngine::new().with_palette(Some(&["red".to_string(), "blue".to_string()]))?;

    assert!(engine.has_colors());

    let color = engine.get_color(0.0, 0);
    assert!(color.is_some());

    Ok(())
}

#[test]
fn test_gradient_color_at() -> Result<()> {
    let gradient = Gradient::parse("linear-gradient(red, blue)")?;

    let color_start = gradient.color_at(0.0);
    assert_eq!(color_start.r, 255);
    assert_eq!(color_start.b, 0);

    let color_end = gradient.color_at(1.0);
    assert_eq!(color_end.r, 0);
    assert_eq!(color_end.b, 255);

    let color_mid = gradient.color_at(0.5);
    assert!(color_mid.r > 0 && color_mid.r < 255);
    assert!(color_mid.b > 0 && color_mid.b < 255);

    Ok(())
}

#[test]
fn test_invalid_duration() {
    assert!(parse_duration("invalid").is_err());
    assert!(parse_duration("10").is_err());
    assert!(parse_duration("10x").is_err());
}

#[test]
fn test_invalid_color() {
    assert!(Color::parse("notacolor").is_err());
    assert!(Color::parse("#GGGGGG").is_err());
}

#[test]
fn test_invalid_effect() {
    assert!(get_effect("not-an-effect").is_err());
}

#[test]
fn test_invalid_easing() {
    assert!(get_easing_function("not-an-easing").is_err());
}
