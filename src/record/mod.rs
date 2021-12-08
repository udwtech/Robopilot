mod keyboard;
mod mouse;

use crate::db::Recording;
use device_query::{DeviceQuery, DeviceState, Keycode};
use keyboard::KeyboardRecord;
use mouse::MouseRecord;
use std::path::Path;

use crate::db::RecordDb;

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

            continue;
        }

        if keyboard.contains(&Keycode::LControl)
            && keyboard.contains(&Keycode::R)
            && keyboard.contains(&Keycode::K)
        {
            let data = KeyboardRecord::new(&device_state);

            if let Some(data) = data {
                let keypress: Vec<String> =
                    data.key_pressed.iter().map(|x| x.to_string()).collect();

                let data_record = Recording {
                    device: data.device,
                    x: None,
                    y: None,
                    button_pressed: None,
                    key_pressed: Some(keypress),
                };

                db.add(data_record);
            }

            continue;
        }

        if keyboard.contains(&Keycode::Escape) {
            println!("Saving Recoreded Data");
            db.save_all();
            return;
        }
    }
}
