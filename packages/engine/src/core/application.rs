use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use super::{
    engine::{Engine, EngineState},
    event::{
        event_handler::{EventHandler, RawCallback},
        Event,
    },
    Error,
};

pub struct Application {
    window: Option<Window>,
    engine: Engine,
    title: String,
    event_handler: EventHandler<Engine>,
}

impl Application {
    pub fn new(title: &str) -> Self {
        let engine = Engine::default();

        let window = None;

        let event_handler = EventHandler::default();

        Self {
            window,
            engine,
            title: title.to_string(),
            event_handler,
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        // Start the engine
        self.engine.run()?;

        // Creates the event loop and sets it to `ControlFlow::Poll`, that way
        // we continously run the event loop
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        event_loop.run_app(self)?;

        Ok(())
    }

    /// Sets the `EventHandler`.
    pub fn set_event_handler(&mut self, handler: RawCallback<Engine>) {
        self.event_handler.set_callback(handler);
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // Sets initial attributes for our window
        let mut window_attributes = Window::default_attributes();
        window_attributes.title = self.title.clone();

        self.window =
            Some(event_loop.create_window(window_attributes).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                let _ = self.engine.shutdown();
            }
            WindowEvent::RedrawRequested => {
                // Redraw the application
                //
                // It's preferable for applications that do not render
                // continously to render in this event rather than in
                // AboutToWait, since rendering in here allows the program to
                // gracefully handle redraws requested by the OS.

                // Step the engine
                self.engine.step();
                // Handle events at other areas
                while let Some(event) = self.engine.require_event() {
                    self.event_handler.step(&mut self.engine, &event);
                }

                // Stop the event loop when the engine gets at the
                // `EngineState::Stopped` state
                if self.engine.state() == EngineState::Stopped {
                    event_loop.exit();
                }

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need
                // to redraw in applications which do not always need to.
                // Applications that redraw continously can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => match event.state {
                winit::event::ElementState::Pressed => {
                    self.engine.dispatch(Event::Keyboard(
                        super::event::keyboard_event::KeyboardEvent::Pressed(
                            event.physical_key,
                        ),
                    ));
                }
                winit::event::ElementState::Released => {
                    self.engine.dispatch(Event::Keyboard(
                        super::event::keyboard_event::KeyboardEvent::Released(
                            event.physical_key,
                        ),
                    ));
                }
            },
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => match state {
                winit::event::ElementState::Pressed => {
                    self.engine.dispatch(Event::Mouse(
                        super::event::mouse_event::MouseEvent::Pressed(button),
                    ));
                }
                winit::event::ElementState::Released => {
                    self.engine.dispatch(Event::Mouse(
                        super::event::mouse_event::MouseEvent::Released(button),
                    ));
                }
            },
            WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => {
                self.engine.dispatch(Event::Mouse(
                    super::event::mouse_event::MouseEvent::Moved(
                        position.x as u32,
                        position.y as u32,
                    ),
                ));
            }
            _ => (),
        }
    }
}
