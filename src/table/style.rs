#[derive(Debug, Clone, Copy)]
pub enum Style {
    Plane,
    Ascii,
    Sharp,
    Rounded,
    Markdown,
}

impl Style {
    pub fn new(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "plane" => Self::Plane,
            "ascii" => Self::Ascii,
            "sharp" => Self::Sharp,
            "rounded" => Self::Rounded,
            "markdown" => Self::Markdown,
            _ => Self::Ascii,
        }
    }
}

pub struct Frame {
    pub has_cover: bool,
    pub border: String,
    pub separator: String,
    pub center: String,
    pub top: String,
    pub left: String,
    pub bottom: String,
    pub right: String,
    pub top_left: String,
    pub top_right: String,
    pub bottom_left: String,
    pub bottom_right: String,
}

impl From<Style> for Frame {
    fn from(style: Style) -> Self {
        match style {
            Style::Plane => Self {
                has_cover: false,
                border: "".into(),
                separator: "\t".into(),
                center: "".into(),
                top: "".into(),
                left: "".into(),
                bottom: "".into(),
                right: "".into(),
                top_left: "".into(),
                top_right: "".into(),
                bottom_left: "".into(),
                bottom_right: "".into(),
            },
            Style::Ascii => Self {
                has_cover: true,
                border: "-".into(),
                separator: "|".into(),
                center: "+".into(),
                top: "+".into(),
                left: "+".into(),
                bottom: "+".into(),
                right: "+".into(),
                top_left: "+".into(),
                top_right: "+".into(),
                bottom_left: "+".into(),
                bottom_right: "+".into(),
            },
            Style::Sharp => Self {
                has_cover: true,
                border: "─".into(),
                separator: "│".into(),
                center: "┼".into(),
                top: "┬".into(),
                left: "├".into(),
                bottom: "┴".into(),
                right: "┤".into(),
                top_left: "┌".into(),
                top_right: "┐".into(),
                bottom_left: "└".into(),
                bottom_right: "┘".into(),
            },
            Style::Rounded => Self {
                has_cover: true,
                border: "─".into(),
                separator: "│".into(),
                center: "┼".into(),
                top: "┬".into(),
                left: "├".into(),
                bottom: "┴".into(),
                right: "┤".into(),
                top_left: "╭".into(),
                top_right: "╮".into(),
                bottom_left: "╰".into(),
                bottom_right: "╯".into(),
            },
            Style::Markdown => Self {
                has_cover: false,
                border: "-".into(),
                separator: "|".into(),
                center: "|".into(),
                top: "".into(),
                left: "|".into(),
                bottom: "".into(),
                right: "|".into(),
                top_left: "".into(),
                top_right: "".into(),
                bottom_left: "".into(),
                bottom_right: "".into(),
            },
        }
    }
}

// fn border(&self, width: usize) -> String {
//     match self {
//         Align::None => "-".repeat(width),
//         Align::Left => format!(":{}", "-".repeat(width - 1)),
//         Align::Center => format!(":{}:", "-".repeat(width - 2)),
//         Align::Right => format!("{}:", "-".repeat(width - 1)),
//     }
// }
