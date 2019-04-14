
extern crate sdl2;

extern crate vectrex_gfx;
extern crate vectrex_logging;


use sdl2::{
        init,
        Sdl,
        VideoSubsystem,
        video::{ Window, WindowBuilder },
        event::{ Event, WindowEvent },
        keyboard::{ Keycode, Scancode, Mod },
        mouse::{ MouseState, MouseButton, MouseWheelDirection }
    };

use vectrex_logging::log::{ info, error };
use vectrex_gfx::render::Graphics;



pub struct WindowProps
{
    pub title: &'static str,
    pub full_screen: bool,
    pub resizable: bool,
    pub width: u32,
    pub height: u32
}



pub struct ShellWindow
{
    sdl_context: Sdl,
    _sdl_video: VideoSubsystem,
    window: Window,

    pub graphics: Graphics
}



impl ShellWindow
{
    pub fn new(props: WindowProps) -> Result<ShellWindow, String>
    {
        let sdl_context = init()?;
        let sdl_video = sdl_context.video()?;

        info!("Creating new host window: ('{}', {}, {}).", props.title, props.width, props.height);
        info!("    Using SDL2 version: {}.", sdl2::version::revision());

        let mut window_builder = sdl_video.window(props.title, props.width, props.height);

        let window =
            ShellWindow::build_resizable(&mut window_builder, &props)
            .position_centered()
            .opengl()
            .build()
            .or_else(|err| Err(err.to_string()) )?;

        info!("    Initializing window graphics.");
        let graphics = Graphics::new(&sdl_video, &window)?;

        Ok(ShellWindow
            {
                sdl_context: sdl_context,
                _sdl_video: sdl_video,
                window: window,
                graphics
            })
    }

    fn build_resizable<'a>(builder: &'a mut WindowBuilder, props: &WindowProps)
        -> &'a mut WindowBuilder
    {
        if props.resizable
        {
            info!("    Making window resizable.");
            return builder.resizable();
        }

        builder
    }

    pub fn event_loop(&mut self)
    {
        let mut event_pump = match self.sdl_context.event_pump()
            {
                Ok(pump)     => pump,
                Err(message) => sdl_fail("Could not acquire context event pump.", &message)
            };

        'main_loop: loop
        {
            for event in event_pump.poll_iter()
            {
                match event
                {
                    Event::Quit {..} =>
                        {
                            info!("Received quit event.");
                            break 'main_loop;
                        },

                    Event::Window
                        {
                            timestamp: _,
                            window_id: _,
                            win_event
                        }
                        => self.handle_window_event(win_event),

                    Event::KeyDown
                        {
                            timestamp: _,
                            window_id: _,
                            keycode, scancode, keymod, repeat
                        }
                        => self.handle_keydown_event(keycode, scancode, keymod, repeat),

                    Event::KeyUp
                        {
                            timestamp: _,
                            window_id: _,
                            keycode, scancode, keymod, repeat
                        }
                        => self.handle_keyup_event(keycode, scancode, keymod, repeat),

                    Event::MouseMotion
                        {
                            timestamp: _, window_id: _, which: _,
                            mousestate, x, y, xrel, yrel
                        }
                        => self.handle_mouse_motion(mousestate, x, y, xrel, yrel),

                    Event::MouseButtonDown
                        {
                            timestamp: _, window_id: _, which: _,
                            mouse_btn, clicks, x, y
                        }
                        => self.handle_mouse_button_down(mouse_btn, clicks, x, y),

                    Event::MouseButtonUp
                        {
                            timestamp: _, window_id: _, which: _,
                            mouse_btn, clicks, x, y
                        }
                        => self.handle_mouse_button_up(mouse_btn, clicks, x, y),

                    Event::MouseWheel
                        {
                            timestamp: _, window_id: _, which: _,
                            x, y, direction
                        }
                        => self.handle_mouse_wheel(x, y, direction),

                    _ => {}
                }

                self.graphics.render();
                self.window.gl_swap_window();
            }
        }
    }

    fn handle_window_event(&mut self, win_event: WindowEvent)
    {
        match win_event
        {
            WindowEvent::Resized(width, height) =>
                {
                    self.graphics.resize_view(width, height);
                }

            _ =>
                {
                }
        }
    }

    fn handle_mouse_motion(&mut self, _mouse_state: MouseState, _x: i32, _y: i32, _xrel: i32, _yrel: i32)
    {
    }

    fn handle_mouse_button_down(&mut self, _mouse_btn: MouseButton, _clicks: u8, _x: i32, _y: i32)
    {
    }

    fn handle_mouse_button_up(&mut self, _mouse_btn: MouseButton, _clicks: u8, _x: i32, _y: i32)
    {
    }

    fn handle_mouse_wheel(&mut self, _x: i32, _y: i32, _direction: MouseWheelDirection)
    {
    }

    fn handle_keydown_event(&mut self, _keycode: Option<Keycode>, _scancode: Option<Scancode>,
                            _keymod: Mod, _repeat: bool)
    {
    }

    fn handle_keyup_event(&mut self, _keycode: Option<Keycode>, _scancode: Option<Scancode>,
                          _keymod: Mod, _repeat: bool)
    {
    }
}



impl Drop for ShellWindow
{
    fn drop(&mut self)
    {
        info!("Main window shutting down.");
    }
}



fn sdl_fail(message: &'static str, reason: &String) -> !
{
    error!("Error: {}, reason: {}, SDL version: {}", message, reason, sdl2::version::revision());

    panic!("Error:       {}\n
            Reason:      {}\n
            SDL Version: {}",
            message, reason, sdl2::version::revision());
}
