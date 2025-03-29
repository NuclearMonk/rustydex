#[derive(Debug, Clone, Copy)]
pub enum NavDirection
{
    Up,
    Down,
    Tab,
    BackTab
}

pub trait Navigation 
{
    fn handle_navigation_input(self, direction: NavDirection)-> bool;

    fn focus(self);
    fn unfocus(self);
}