use simple_logger::SimpleLogger;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() -> Result<(), impl std::error::Error> {
    SimpleLogger::new().init().unwrap();
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(640.0, 480.0))
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, elwt| {

        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::RedrawRequested => {
                    // Notify the windowing system that we'll be presenting to the window.
                    window.pre_present_notify();
                }
                _ => {
                    println!("Unhandled WindowEvent: {event:?}");
                },
            },
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {
                println!("Unhandled Event: {event:?}");
            },
        }
    })
}