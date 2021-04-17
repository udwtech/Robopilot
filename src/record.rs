
use device_query::{DeviceQuery, DeviceState, Keycode};

pub fn action(outdir: &str) {
    let device_state = DeviceState::new();

    loop {
        
        let keyboard = device_state.get_keys();

        if keyboard.contains(&Keycode::LControl)
            && keyboard.contains(&Keycode::R)
            && keyboard.contains(&Keycode::M)
        {
            record_mouse(&device_state);
            continue;
        }

        if keyboard.contains(&Keycode::LControl)
            && keyboard.contains(&Keycode::R)
            && keyboard.contains(&Keycode::K)
        {
            record_keyboard(&device_state);
            continue;
        }

        if keyboard.contains(&Keycode::Escape) {
            return;
        }
    }
}


pub struct MouseRecord{
    position:Option<(i32,i32)>,
    click:Option<Vec<bool>>
}

fn record_mouse(device_state: &DeviceState) -> MouseRecord {
    let position: Option<(i32, i32)>;
    let click: Option<Vec<bool>>;

    println!("Start Mouse Recording");

    loop {
        let keyboard = device_state.get_keys();
        let mouse = device_state.get_mouse();

        if keyboard.contains(&Keycode::P) {
            println!("Start Cursor Position Recording");
            position = Some(mouse.coords);
            println!("Cursor Position {:?}", position);
            return MouseRecord{position, click: None};
        }

        if keyboard.contains(&Keycode::K) {
            println!("Start Mouse Button Click Recording");
            loop {
                let mouse = device_state.get_mouse();

                if mouse.button_pressed != vec![false; 6] {
                    position = Some(mouse.coords);
                    click = Some(mouse.button_pressed);
                    println!("Clicked {:?}",click.as_ref().unwrap());
                    println!("Cursor Position {:?}", position);
                    return MouseRecord{position,click}
                }
            }
        }

        if keyboard.contains(&Keycode::Escape) {
            println!("No recording on mouse position or clicks");
            return MouseRecord{position:None,click:None}
        }

    }
}

pub struct KeyboardInputs{
    pub keys:Vec<Keycode>
}

fn record_keyboard(device_state: &DeviceState) ->Option<KeyboardInputs> {
    let mut all_inputs:Vec<Keycode> = Vec::new();
    std::thread::sleep(std::time::Duration::from_millis(500));

    loop{
        std::thread::sleep(std::time::Duration::from_millis(100));
        let mut keyboard = device_state.get_keys();

        if keyboard.contains(&Keycode::F12){
            println!("Completed batch of keypress.");
            println!("Total KeyPress :: {:?}", &all_inputs);
            return Some(KeyboardInputs{keys:all_inputs})
        }

        if keyboard.contains(&Keycode::Escape){
            println!("Cancel recording any key press!");
            return None;
        }

        
        if keyboard.len() > 0{
            println!("Add keypress :: {:?}", &keyboard);
            all_inputs.append(&mut keyboard)
        }
        
    }
}
