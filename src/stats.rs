use serde::Deserialize;
use tui::{layout::Constraint, style::Color};

#[derive(Deserialize)]
#[serde(remote = "Constraint")]
enum ConstraintDef {
    Percentage(u16),
    Ratio(u32, u32),
    Length(u16),
    Max(u16),
    Min(u16),
}

#[derive(Deserialize)]
#[serde(remote = "Color")]
enum ColorDef {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
}

// https://github.com/serde-rs/serde/issues/723#issuecomment-382501277
//fn vec_constraint<'de, D>(deserializer: D) -> Result<Vec<Constraint>, D::Error>
//where
//    D: Deserializer<'de>,
//{
//    #[derive(Deserialize)]
//    struct Wrapper(#[serde(with = "ConstraintDef")] Constraint);
//
//    let v = Vec::deserialize(deserializer)?;
//    Ok(v.into_iter().map(|Wrapper(a)| a).collect())
//}

fn default_color() -> Color {
    Color::Green
}

#[derive(Deserialize, Debug, Clone)]
pub struct StatsCfg {
    pub title: String,
    pub id: String,
    pub idx: usize,
    #[serde(with = "ColorDef", default = "default_color")]
    pub color: Color,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HeaderElement {
    pub name: String,
    #[serde(with = "ConstraintDef")]
    pub constraint: Constraint,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Cfg {
    pub header: Vec<HeaderElement>,
    #[serde(default = "Vec::<_>::new")]
    pub skips: Vec<String>,
    #[serde(default = "Vec::<_>::new")]
    pub stats: Vec<StatsCfg>,
    #[serde(default)]
    pub log: bool,
}
