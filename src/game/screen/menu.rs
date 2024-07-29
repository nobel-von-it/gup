use crate::game::consts;

use super::buttons::Buttons;

pub struct MenuScreen {
    pub title: String,
    pub buttons: Vec<Buttons>,
    pub selected: usize,
}
impl MenuScreen {
    pub fn new() -> Self {
        let title = consts::TITLE.to_string();
        let buttons = vec![Buttons::Start, Buttons::Exit];
        let selected = 0;
        Self {
            title,
            buttons,
            selected,
        }
    }
}
