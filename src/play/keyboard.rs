use std::str::FromStr;

use device_query::Keycode;
use enigo::{Enigo, Key, KeyboardControllable};

use crate::db::Recording;

pub fn play(recording: &Recording) {
    let key_pressed = recording
        .key_pressed
        .as_ref()
        .expect("No key pressed data present in the recording.");

    let mut enigo = Enigo::new();

    key_pressed.iter().for_each(|key| {
        let key: Option<Key> = string_to_key(&key);

        match key {
            Some(key) => {
                enigo.key_click(key);
            }
            None => {}
        }
    })
}

fn string_to_key(str_key: &str) -> Option<Key> {
    let str_key = Keycode::from_str(str_key).unwrap();

    match str_key {
        Keycode::F1 => Some(Key::F1),
        Keycode::F2 => Some(Key::F2),
        Keycode::F3 => Some(Key::F3),
        Keycode::F4 => Some(Key::F4),
        Keycode::F5 => Some(Key::F5),
        Keycode::F6 => Some(Key::F6),
        Keycode::F7 => Some(Key::F7),
        Keycode::F8 => Some(Key::F8),
        Keycode::F9 => Some(Key::F9),
        Keycode::F10 => Some(Key::F10),
        Keycode::F11 => Some(Key::F11),
        Keycode::F12 => Some(Key::F12),
        Keycode::Escape => Some(Key::Escape),
        Keycode::Space => Some(Key::Space),
        Keycode::LControl => Some(Key::Control),
        Keycode::RControl => Some(Key::Control),
        Keycode::LShift => Some(Key::Shift),
        Keycode::RShift => Some(Key::Shift),
        Keycode::LAlt => Some(Key::Alt),
        Keycode::RAlt => Some(Key::Alt),
        Keycode::Meta => Some(Key::Meta),
        Keycode::Enter => Some(Key::Return),
        Keycode::Up => Some(Key::UpArrow),
        Keycode::Down => Some(Key::DownArrow),
        Keycode::Left => Some(Key::LeftArrow),
        Keycode::Right => Some(Key::RightArrow),
        Keycode::Backspace => Some(Key::Backspace),
        Keycode::CapsLock => Some(Key::CapsLock),
        Keycode::Tab => Some(Key::Tab),
        Keycode::Home => Some(Key::Home),
        Keycode::End => Some(Key::End),
        Keycode::PageUp => Some(Key::PageUp),
        Keycode::PageDown => Some(Key::PageDown),
        Keycode::Insert => Some(Key::Raw(0x2D)),
        Keycode::Delete => Some(Key::Delete),
        Keycode::Numpad0 => Some(Key::Raw(0x60)),
        Keycode::Numpad1 => Some(Key::Raw(0x61)),
        Keycode::Numpad2 => Some(Key::Raw(0x62)),
        Keycode::Numpad3 => Some(Key::Raw(0x63)),
        Keycode::Numpad4 => Some(Key::Raw(0x64)),
        Keycode::Numpad5 => Some(Key::Raw(0x65)),
        Keycode::Numpad6 => Some(Key::Raw(0x66)),
        Keycode::Numpad7 => Some(Key::Raw(0x67)),
        Keycode::Numpad8 => Some(Key::Raw(0x68)),
        Keycode::Numpad9 => Some(Key::Raw(0x69)),
        Keycode::NumpadSubtract => Some(Key::Raw(0x6D)),
        Keycode::NumpadAdd => Some(Key::Raw(0x6B)),
        Keycode::NumpadDivide => Some(Key::Raw(0x6F)),
        Keycode::NumpadMultiply => Some(Key::Raw(0x6A)),
        Keycode::Grave => None,
        Keycode::Minus => Some(Key::Raw(0xBD)),
        Keycode::Equal => None,
        Keycode::LeftBracket => None,
        Keycode::RightBracket => None,
        Keycode::BackSlash => None,
        Keycode::Semicolon => Some(Key::Raw(0xBA)),
        Keycode::Apostrophe => None,
        Keycode::Comma => Some(Key::Raw(0xBC)),
        Keycode::Dot => Some(Key::Raw(0xBE)),
        Keycode::Slash => Some(Key::Raw(0xBF)),
        Keycode::Key0 => Some(Key::Layout('0')),
        Keycode::Key1 => Some(Key::Layout('1')),
        Keycode::Key2 => Some(Key::Layout('2')),
        Keycode::Key3 => Some(Key::Layout('3')),
        Keycode::Key4 => Some(Key::Layout('4')),
        Keycode::Key5 => Some(Key::Layout('5')),
        Keycode::Key6 => Some(Key::Layout('6')),
        Keycode::Key7 => Some(Key::Layout('7')),
        Keycode::Key8 => Some(Key::Layout('8')),
        Keycode::Key9 => Some(Key::Layout('9')),
        Keycode::A => Some(Key::Raw(0x41)),
        Keycode::B => Some(Key::Raw(0x42)),
        Keycode::C => Some(Key::Raw(0x43)),
        Keycode::D => Some(Key::Raw(0x44)),
        Keycode::E => Some(Key::Raw(0x45)),
        Keycode::F => Some(Key::Raw(0x46)),
        Keycode::G => Some(Key::Raw(0x47)),
        Keycode::H => Some(Key::Raw(0x48)),
        Keycode::I => Some(Key::Raw(0x49)),
        Keycode::J => Some(Key::Raw(0x4A)),
        Keycode::K => Some(Key::Raw(0x4B)),
        Keycode::L => Some(Key::Raw(0x4C)),
        Keycode::M => Some(Key::Raw(0x4D)),
        Keycode::N => Some(Key::Raw(0x4E)),
        Keycode::O => Some(Key::Raw(0x4F)),
        Keycode::P => Some(Key::Raw(0x50)),
        Keycode::Q => Some(Key::Raw(0x51)),
        Keycode::R => Some(Key::Raw(0x52)),
        Keycode::S => Some(Key::Raw(0x53)),
        Keycode::T => Some(Key::Raw(0x54)),
        Keycode::U => Some(Key::Raw(0x55)),
        Keycode::V => Some(Key::Raw(0x56)),
        Keycode::W => Some(Key::Raw(0x57)),
        Keycode::X => Some(Key::Raw(0x58)),
        Keycode::Y => Some(Key::Raw(0x59)),
        Keycode::Z => Some(Key::Raw(0x5A)),
    }
}
