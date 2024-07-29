use ratatui::Frame;

pub trait Draw {
    fn draw(&self, f: &mut Frame);
}

pub trait MoveSelect {
    fn up(&mut self);
    fn down(&mut self);
}
