use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::WindowBuilder;

fn main() {
    // Building event_loop
    let event_loop = EventLoop::new().unwrap();
    let window_builder = WindowBuilder::new()
        .with_title("New Window")
        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));
    let window = window_builder.build(&event_loop).unwrap();

    // ControlFlow::Poll continuously runs the event loop even without OS dispatch.
    // ControlFlow::Wait pauses the loop and if no events are dispatched.
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run(move |event, elwt| match event {
            Event::AboutToWait => {
                window.request_redraw();
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    println!("Close button was pressed! \n");
                    elwt.exit();
                }
                WindowEvent::RedrawRequested => { /*println!("Redraw was requested! \n");*/ }
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            logical_key: key,
                            state: ElementState::Pressed,
                            repeat: false,
                            ..
                        },
                    ..
                } => match key.as_ref() {
                    Key::Character("1") => {
                        println!("Key 1 was pressed!\n")
                    }
                    Key::Named(NamedKey::Escape) => {
                        println!("Escape key was pressed! \n");
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => (),
        })
        .expect("event_loop: Failed somewhere. \n");
}
