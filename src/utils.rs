use enigo::Button;
use iced::theme::{Custom, Palette};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::sync::{Arc, Mutex};
use iced::Theme;

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

pub fn serialize_mouse_button<S>(button: &Arc<Mutex<Button>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let button = button.lock().unwrap();
    match *button {
        Button::Left => "Left".serialize(serializer),
        Button::Middle => "Middle".serialize(serializer),
        Button::Right => "Right".serialize(serializer),
        _ => Err(serde::ser::Error::custom("Unsupported mouse button")),
    }
}

pub fn deserialize_mouse_button<'de, D>(deserializer: D) -> Result<Arc<Mutex<Button>>, D::Error>
where
    D: Deserializer<'de>,
{
    let button_str = String::deserialize(deserializer)?;
    let button = match button_str.as_str() {
        "Left" => Button::Left,
        "Middle" => Button::Middle,
        "Right" => Button::Right,
        _ => return Err(serde::de::Error::custom("Unsupported mouse button")),
    };
    Ok(Arc::new(Mutex::new(button)))
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
