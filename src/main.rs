use std::{io::Cursor, time::Instant};
use rodio::{OutputStream, Source};

use glium::{winit::{keyboard::{KeyCode, PhysicalKey}, window::Window}, Surface, uniform};
use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;
mod teapot;
mod matrices;

fn lock_cursor(window: &Window) {
    if window.set_cursor_grab(glium::winit::window::CursorGrabMode::Locked).is_err() {
        eprintln!("Failed to lock mouse!")
    }
    window.set_cursor_visible(false);
}

fn unlock_cursor(window: &Window) {
    if window.set_cursor_grab(glium::winit::window::CursorGrabMode::None).is_err() {
        eprintln!("Failed to unlock mouse!")
    }
    window.set_cursor_visible(true);
}

fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");

    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("TEAPOT (but you can wasd to move)")
        .build(&event_loop);

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
        let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
        let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                            &teapot::INDICES).unwrap();

        let program = glium::Program::from_source(&display, include_str!("../shaders/vertex_shader.vert"), include_str!("../shaders/fragment_shader.frag"),
                                                None).unwrap();

    let mut random_positions = Vec::new();

    let mut rng = StdRng::seed_from_u64(0);
    for _ in 0..1000 {
        random_positions.push([
            rng.gen_range(-64.0..64.0),
            rng.gen_range(-64.0..64.0),
            rng.gen_range(-64.0..64.0),
        ]);
    }

    // MUSIC!!!
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let source = rodio::Decoder::new(Cursor::new(include_bytes!("../funkytown.mp3"))).unwrap();
    let _ = stream_handle.play_raw(source.repeat_infinite().convert_samples());

    let mut before = Instant::now(); // used for delta_time
    let start = Instant::now(); // Used to calculate time elapsed since program started

    let mut pos = [0.0, 0.0, 0.0f32];
    let mut move_vector = [0.0, 0.0, 0.0];
    
    let mut yaw = 0.0f32; // Horizontal rotation (in radians)
    let mut pitch = 0.0f32; // Vertical rotation (in radians)
    event_loop.run(move |ev, window_target| {
        match ev {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                },
                glium::winit::event::WindowEvent::RedrawRequested => {
                    let delta_time = Instant::now() - before; // Work out delta time
                    let delta_secs = delta_time.as_secs_f32();
                    println!("{} FPS", 1.0/delta_secs);
                    before = Instant::now(); // Set this for next frame

                    let forward = [
                        yaw.cos(),
                        0.0,
                        yaw.sin(),
                    ];

                    let right = [
                        yaw.sin(),
                        0.0,
                        -yaw.cos(),
                    ];

                    let world_move_vector = [
                        forward[0] * move_vector[2] + right[0] * move_vector[0],
                        move_vector[1],
                        forward[2] * move_vector[2] + right[2] * move_vector[0],
                    ];

                    pos[0] += world_move_vector[0] * delta_secs * 3.0;
                    pos[1] += world_move_vector[1] * delta_secs * 3.0;
                    pos[2] += world_move_vector[2] * delta_secs * 3.0;


                    let direction = [
                        yaw.cos() * pitch.cos(),
                        pitch.sin(),
                        yaw.sin() * pitch.cos(),
                    ];

                    let teapot_yaw = (Instant::now()-start).as_secs_f32()*8.133333333333332;

                    let yaw_sin = teapot_yaw.sin();
                    let yaw_cos = teapot_yaw.cos();

                    let yaw_matrix = [
                        [yaw_cos, 0.0, yaw_sin, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [-yaw_sin, 0.0, yaw_cos, 0.0],
                        [0.0, 0.0, 0.0, 1.0]
                    ];


                    let mut target = display.draw();
                    target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

                    let view = matrices::view_matrix(&pos, &direction, &[0.0,1.0,0.0]);
                    let perspective = matrices::perspective(&target);
                    let light = [-1.0, 0.4, 0.9f32];

                    let params = glium::DrawParameters {
                        depth: glium::Depth {
                            test: glium::draw_parameters::DepthTest::IfLess,
                            write: true,
                            .. Default::default()
                        },
                        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                        .. Default::default()
                    };


                    for pos in random_positions.clone() {
                        target.draw((&positions, &normals), &indices, &program,
                        &uniform! { model: matrices::move_and_scale(pos[0], pos[1], pos[2], 0.01), view: view, perspective: perspective, u_light: light, yaw_matrix: yaw_matrix},
                        &params).unwrap();
                    }
                    // Draw to screen
                    target.finish().unwrap();

                },
                // Because glium doesn't know about windows we need to resize the display
                // when the window's size has changed.
                glium::winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                },
                glium::winit::event::WindowEvent::KeyboardInput { event, .. } => {
                    if event.state == glium::winit::event::ElementState::Pressed && !event.repeat {
                        match event.physical_key {
                            PhysicalKey::Code(KeyCode::KeyW) => {
                                move_vector[2] = 1.0;
                            },
                            PhysicalKey::Code(KeyCode::KeyS) => {
                                move_vector[2] = -1.0;
                            },
                            PhysicalKey::Code(KeyCode::KeyA) => {
                                move_vector[0] = -1.0;
                            },
                            PhysicalKey::Code(KeyCode::KeyD) => {
                                move_vector[0] = 1.0;
                            },
                            PhysicalKey::Code(KeyCode::KeyE) => {
                                move_vector[1] = 1.0;
                            },
                            PhysicalKey::Code(KeyCode::KeyQ) => {
                                move_vector[1] = -1.0;
                            },
                            PhysicalKey::Code(KeyCode::Escape) => {
                                unlock_cursor(&window);
                            },
                            _ => ()
                        }
                    } else if event.state == glium::winit::event::ElementState::Released {
                        match event.physical_key {
                            PhysicalKey::Code(KeyCode::KeyW) => {
                                move_vector[2] = 0.0;
                            },
                            PhysicalKey::Code(KeyCode::KeyS) => {
                                move_vector[2] = 0.0;
                            },
                            PhysicalKey::Code(KeyCode::KeyA) => {
                                move_vector[0] = 0.0;
                            },
                            PhysicalKey::Code(KeyCode::KeyD) => {
                                move_vector[0] = 0.0;
                            },
                            PhysicalKey::Code(KeyCode::KeyE) => {
                                move_vector[1] = 0.0;
                            },
                            PhysicalKey::Code(KeyCode::KeyQ) => {
                                move_vector[1] = 0.0;
                            },
                            _ => ()
                        }
                    }
                },
                glium::winit::event::WindowEvent::Focused(focused) => {
                    if focused {
                        lock_cursor(&window);
                    } else {
                        unlock_cursor(&window);
                    }
                },
                glium::winit::event::WindowEvent::MouseInput { state, button, .. } => {
                    if state == glium::winit::event::ElementState::Pressed && button == glium::winit::event::MouseButton::Left {
                        lock_cursor(&window);
                    }
                },
                _ => (),
            },
            glium::winit::event::Event::DeviceEvent { event, .. } => match event {
                // delta is a tuple of x and y movement
                glium::winit::event::DeviceEvent::MouseMotion { delta } => {
                    // delta x and y
                    let (dx, dy) = delta;
                    let sensitivity = 0.001;
                    // Update yaw and pitch based on mouse movement
                    yaw -= dx as f32 * sensitivity;
                    pitch -= dy as f32 * sensitivity;

                    // Clamp pitch to prevent flipping
                    pitch = pitch.clamp(-std::f32::consts::FRAC_PI_2 + 0.01, std::f32::consts::FRAC_PI_2 - 0.01);
                }
                _ => (),
            },
            // By requesting a redraw in response to a AboutToWait event we get continuous rendering.
            // For applications that only change due to user input you could remove this handler.
            glium::winit::event::Event::AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        }
    })
    .unwrap();
}