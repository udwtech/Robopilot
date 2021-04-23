use device_query::{DeviceState, Keycode, DeviceQuery};

/*

*/
#[derive(Debug)]
pub struct KeyboardRecord {
    pub device:String,
    pub key_pressed: Vec<Keycode>,
}

impl KeyboardRecord {
    pub fn new (device_state: &DeviceState) -> Option<KeyboardRecord> {
        let mut all_inputs: Vec<Keycode> = Vec::new();
        std::thread::sleep(std::time::Duration::from_millis(500));
    
        loop {
            std::thread::sleep(std::time::Duration::from_millis(100));
            let mut keyboard = device_state.get_keys();
    
            if keyboard.contains(&Keycode::F12) {
                println!("Completed batch of keypress.");
                println!("Total KeyPress :: {:?}", &all_inputs);
                return Some(KeyboardRecord {
                     device: String::from("keyboard"),
                     key_pressed: all_inputs });
            }
    
            if keyboard.contains(&Keycode::Escape) {
                println!("Cancel recording any key press!");
                return None;
            }
    
            if keyboard.len() > 0 {
                println!("Add keypress :: {:?}", &keyboard);
                all_inputs.append(&mut keyboard)
            }
        }
    }
}

