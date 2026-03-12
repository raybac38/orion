
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::event_loop::ActiveEventLoop;
use winit::window::Cursor;
use winit::window::Window;
use winit::window::WindowAttributes;
use winit::window::WindowButtons;
use winit::window::WindowLevel;


fn main() {
    let event_loop = EventLoop::new().unwrap();

    let mut app: App = App{window : None};
    _ = event_loop.run_app(&mut app);
}


struct App {
    window : Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(<WindowAttributes as std::default::Default>::default().with_title("hell")).unwrap());
        self.window.as_mut().unwrap().request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            // 1. Gérer la fermeture de la fenêtre
            WindowEvent::CloseRequested => {
                println!("Fermeture de l'application...");
                event_loop.exit();
            }
            // 2. Demander un redessinage si nécessaire
            WindowEvent::RedrawRequested => {
                if let Some(window) = self.window.as_ref() {
                    println!("redraw");
                    window.pre_present_notify();
                }
            }
            _ => (),
        }

    }
}


