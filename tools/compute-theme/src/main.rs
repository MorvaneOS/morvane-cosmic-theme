//! Computes the MorvaneOS Cosmic theme with Cosmic's own theme code.
//!
//!     compute-theme OUT ACCENT BACKGROUND NEUTRAL_TINT TEXT_TINT
//!
//! Colours are #RRGGBB. Writes the dark theme builder and the theme Cosmic computes
//! from it (every shade: hover, pressed, text on accent and so on) to
//! OUT/com.system76.CosmicTheme.Dark[.Builder]/v*/, in Cosmic's own config layout,
//! the way Cosmic Settings would when a user picks these colours.

use cosmic_config::{Config, CosmicConfigEntry};
use cosmic_theme::{Theme, ThemeBuilder, DARK_THEME_BUILDER_ID, DARK_THEME_ID};
use palette::Srgb;
use std::path::PathBuf;
use std::str::FromStr;

fn colour(hex: &str) -> Srgb {
	Srgb::<u8>::from_str(hex)
		.unwrap_or_else(|_| panic!("not a #RRGGBB colour: {hex}"))
		.into_format()
}

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let [_, out, accent, background, neutral_tint, text_tint] = args.as_slice() else {
		eprintln!("usage: compute-theme OUT ACCENT BACKGROUND NEUTRAL_TINT TEXT_TINT");
		std::process::exit(2);
	};
	let out = PathBuf::from(out);

	let builder = ThemeBuilder::dark()
		.accent(colour(accent))
		.bg_color(colour(background).into())
		.neutral_tint(colour(neutral_tint))
		.text_tint(colour(text_tint));
	let theme: Theme = builder.clone().build();

	let config = Config::with_custom_path(DARK_THEME_BUILDER_ID, ThemeBuilder::VERSION, out.clone())
		.expect("opening the builder config");
	builder.write_entry(&config).expect("writing the builder");
	let config = Config::with_custom_path(DARK_THEME_ID, Theme::VERSION, out).expect("opening the theme config");
	theme.write_entry(&config).expect("writing the theme");
}
