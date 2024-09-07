use enigo::Button;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::sync::{Arc, Mutex};

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
