mod mouse_record;

mod keyboard_record;
mod record_db;

use std::path::Path;

use device_query::{DeviceQuery, DeviceState, Keycode};
use mouse_record::MouseRecord;
use keyboard_record::KeyboardRecord;

use self::record_db::RecordDb;

pub fn action(outdir: &Path) {
    let device_state = DeviceState::new();

    let db = RecordDb::new(outdir,None)
    .expect("Fail Create File");

    loop {
        let keyboard = device_state.get_keys();

        if keyboard.contains(&Keycode::LControl)
            && keyboard.contains(&Keycode::R)
            && keyboard.contains(&Keycode::M)
        {
            let data = MouseRecord::new(&device_state);
            continue;
        }

        if keyboard.contains(&Keycode::LControl)
            && keyboard.contains(&Keycode::R)
            && keyboard.contains(&Keycode::K)
        {
            KeyboardRecord::new(&device_state);
            continue;
        }

        if keyboard.contains(&Keycode::Escape) {
            return;
        }
    }
}