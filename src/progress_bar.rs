use std::{borrow::Cow, time::Duration};

use console::Emoji;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

const TICKS: &str = "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ";

pub fn create_spinner(msg: impl Into<Cow<'static, str>>, m: &MultiProgress) -> ProgressBar {
    let spinner_style = ProgressStyle::with_template("{prefix} {spinner} {wide_msg}")
        .expect("Template must be correct")
        .tick_chars(TICKS);

    let pb = m.add(ProgressBar::new_spinner());
    pb.set_style(spinner_style);
    pb.set_prefix(format!("{} ", Emoji("🖼️", "Processing...")));
    pb.set_message(msg);
    pb.enable_steady_tick(Duration::from_millis(100));

    pb
}
