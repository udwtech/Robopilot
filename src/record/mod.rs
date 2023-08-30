mod keyboard;
mod mouse;

use crate::db::{Recording, RecordDb};
use device_query::{DeviceQuery, DeviceState, Keycode};
use keyboard::KeyboardRecord;
use mouse::MouseRecord;
use std::path::Path;

pub fn action(outdir: &Path) {
    let device_state = DeviceState::new();
    let mut db = RecordDb::new(outdir, None).expect("Fail Create File");

    loop {
        let keyboard = device_state.get_keys();
        handle_keyboard_input(&keyboard, &device_state, &mut db);
    }
}


fn handle_keyboard_input(keyboard_inputs: &Vec<Keycode>, device_state: &DeviceState, db: &mut RecordDb) {
    // Define the key combinations for mouse and keyboard recording
    let key_combinations = [
        (vec![Keycode::LControl, Keycode::R, Keycode::M], handle_mouse_record),
        (vec![Keycode::LControl, Keycode::R, Keycode::K], handle_keyboard_record),
    ];

    // Iterate over the key combinations
    for (combination, handler) in key_combinations.iter() {
        if keyboard_inputs.iter().all(|k| combination.contains(k)) {
            handler(device_state, db);
            return;
        }
    }

    // If the escape key is pressed, save all recorded data and return
    if keyboard_inputs.contains(&Keycode::Escape) {
        println!("Saving Recoreded Data");
        db.save_all();
        return;
    }
}

// Function to handle mouse records
fn handle_mouse_record(device_state: &DeviceState, db: &mut RecordDb) {
    // Create a new mouse record from the device state
    let data = MouseRecord::new(device_state);
    // Create a new recording with the mouse record data
    let data_record = Recording {
        device: data.device,
        x: data.x,
        y: data.y,
        button_pressed: data.button_pressed,
        key_pressed: None,
    };
    // Print the mouse record data for debugging purposes
    println!("Mouse record Data {:#?}", &data_record);
    // Add the new recording to the database
    db.add(data_record);
}

fn handle_keyboard_record(device_state: &DeviceState, db: &mut RecordDb) {
    // Check if a new keyboard record can be created from the device state
    if let Some(data) = KeyboardRecord::new(device_state) {
        // Convert the key presses into a vector of strings
        let keypress: Vec<String> =
            data.key_pressed.iter().map(|x| x.to_string()).collect();

        // Create a new recording with the keyboard record data
        let data_record = Recording {
            device: data.device,
            x: None,
            y: None,
            button_pressed: None,
            key_pressed: Some(keypress),
        };

        // Add the new recording to the database
        db.add(data_record);
    }
}
