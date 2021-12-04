use winit::{
    event::{
        Event,
        WindowEvent,
    },
    event_loop::ControlFlow,
};

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
pub fn main() {
    println!("Starting...");

    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .build(&event_loop).unwrap();
    
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Exit Requested. Exiting Sound Galaxy...");
                *control_flow = ControlFlow::Exit
            },
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::Resized(physical_size) => {
                        println!("Resized; w: {}, h: {}", physical_size.width, physical_size.height);
                    },
                    WindowEvent::ScaleFactorChanged {
                        new_inner_size,
                        ..
                    } => {
                        println!("ScaleFactorChanged; w: {}, h: {}", new_inner_size.width, new_inner_size.height);
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    });
}