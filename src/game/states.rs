use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum States {
    Settings(SettingsScreen),
    Menu(MenuScreen),
    Game(GameScreen),
}
