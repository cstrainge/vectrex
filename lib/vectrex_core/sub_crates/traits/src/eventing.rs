
pub trait WindowHandler
{
    fn on_resize(&mut self, width: i32, height: i32);
}



pub enum MouseButton
{
    Left,
    Middle,
    Right
}

pub enum MouseWheel
{
    Up,
    Down
}

pub trait MouseHandler
{
    fn on_mouse_move(&mut self, x: i32, y: i32, x_rel: i32, y_rel: i32);

    fn on_mouse_down(&mut self, x: i32, y: i32, button: MouseButton);
    fn on_mouse_up(&mut self, x: i32, y: i32, button: MouseButton);

    fn on_mouse_wheel(&mut self, x: i32, y: i32, wheel: MouseWheel);
}



pub struct KeyFlags
{
    ctrl: bool,
    alt: bool,
    logo: bool
}

pub struct KeyMod
{
    left: KeyFlags,
    right: KeyFlags
}

pub trait KeyboardHandler
{
    fn on_key_down(&mut self, key: u8, key_mod: &KeyMod, repeat: bool);
    fn on_key_up(&mut self, key: u8, key_mod: &KeyMod);
}
