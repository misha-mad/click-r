use iced::theme::{Custom, Palette};
use iced::Theme;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
#[serde(remote = "Theme")]
pub enum ThemeDef {
    Light,
    Dark,
    Dracula,
    Nord,
    SolarizedLight,
    SolarizedDark,
    GruvboxLight,
    GruvboxDark,
    CatppuccinLatte,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    CatppuccinMocha,
    TokyoNight,
    TokyoNightStorm,
    TokyoNightLight,
    KanagawaWave,
    KanagawaDragon,
    KanagawaLotus,
    Moonfly,
    Nightfly,
    Oxocarbon,
    #[serde(
        serialize_with = "serialize_arc_custom_theme",
        deserialize_with = "deserialize_arc_custom_theme"
    )]
    Custom(Arc<Custom>),
}

pub fn serialize_arc_custom_theme<S>(_: &Arc<Custom>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    "Rust".serialize(serializer)
}

pub fn deserialize_arc_custom_theme<'de, D>(_: D) -> Result<Arc<Custom>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Arc::new(Custom::new(
        "Rust".to_string(),
        Palette {
            background: Default::default(),
            text: Default::default(),
            primary: Default::default(),
            success: Default::default(),
            danger: Default::default(),
        },
    )))
}
