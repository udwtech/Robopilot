mod keyboard_record;
mod mouse_record;
mod record_db;

use device_query::{DeviceQuery, DeviceState, Keycode};
use keyboard_record::KeyboardRecord;
use mouse_record::MouseRecord;
use record_db::{RecordDb, Recording};
use std::path::Path;

pub fn action(outdir: &Path) {
    let device_state = DeviceState::new();

    let mut db = RecordDb::new(outdir, None).expect("Fail Create File");

    loop {
        let keyboard = device_state.get_keys();

        if keyboard.contains(&Keycode::LControl)
            && keyboard.contains(&Keycode::R)
            && keyboard.contains(&Keycode::M)
        {
            let data = MouseRecord::new(&device_state);
            let data_record = Recording {
                device: data.device,
                x: data.x,
                y: data.y,
                button_pressed: data.button_pressed,
                key_pressed: None,
            };
            println!("Mouse record Data {:#?}", &data_record);

            db.add(data_record);

            db.save_all();

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
