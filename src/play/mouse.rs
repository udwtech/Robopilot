use crate::db::Recording;
use enigo::{self, Enigo, MouseButton, MouseControllable};

pub fn play(recording: &Recording) {
    let x = recording
        .x
        .expect("No Coordination X value exist in recoding data");

    let y = recording
        .y
        .expect("No Coordination Y value exist in recoding data");

    match &recording.button_pressed {
        Some(pressed) => move_and_press(x, y, pressed),
        None => move_cursor_only(x, y),
    }

    std::thread::sleep(std::time::Duration::from_secs(1));
}

fn move_cursor_only(x: i32, y: i32) {
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(x, y);
}

fn move_and_press(x: i32, y: i32, button_press: &[bool]) {
    let mut enigo = Enigo::new();

    let mouse_click = match button_press.get(1).unwrap().eq(&true) {
        true => MouseButton::Left,
        false => MouseButton::Right,
    };

    move_cursor_only(x, y);
    enigo.mouse_click(mouse_click);
}

#[cfg(test)]
mod mouse_test {

    #[test]
    fn test1(){
        assert_eq!(1,1);
    }

}