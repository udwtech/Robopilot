use device_query::{DeviceState, Keycode, DeviceQuery};

#[derive(Debug)]
pub struct MouseRecord {
    pub device:String,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub button_pressed: Option<Vec<bool>>,
}

impl MouseRecord {
    pub fn new(device_state: &DeviceState) -> MouseRecord {
        let position: Option<(i32, i32)>;
        let click: Option<Vec<bool>>;
    
        println!("Start Mouse Recording");
    
        loop {
            let keyboard = device_state.get_keys();
    
            if keyboard.contains(&Keycode::P) {
                let mouse = device_state.get_mouse();
                println!("Start Cursor Position Recording");
                position = Some(mouse.coords);
                println!("Cursor Position {:?}", position);
                return MouseRecord {
                    device: String::from("mouse"),
                    x:Some(mouse.coords.0),
                    y:Some(mouse.coords.1),
                    button_pressed: None,
                };
            }
    
            if keyboard.contains(&Keycode::K) {
                println!("Start Mouse Button Click Recording");
                loop {
                    let mouse = device_state.get_mouse();
    
                    if mouse.button_pressed != vec![false; 6] {
                        position = Some(mouse.coords);
                        click = Some(mouse.button_pressed.clone());
                        println!("Clicked {:?}", click.as_ref().unwrap());
                        println!("Cursor Position {:?}", position);
                        return MouseRecord { 
                            device:String::from("mouse"),
                            x:Some(mouse.coords.0),
                            y:Some(mouse.coords.1),
                            button_pressed: click };
                    }
                }
            }
    
            if keyboard.contains(&Keycode::F12) {
                println!("No recording on mouse position or clicks");
                return MouseRecord {
                    device: String::from("mouse"),
                    x:None,
                    y:None,
                    button_pressed: None,
                };
            }
        }
    }
}
