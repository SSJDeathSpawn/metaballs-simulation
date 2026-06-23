pub mod graphics;
pub mod vertex;
use std::{
    ffi::CString,
    fs::File,
    io::Write,
    ptr::null,
    time::{Duration, SystemTime},
};

use gl::types::GLint;
use graphics::*;
use sdl2::{VideoSubsystem, event::Event, keyboard::Keycode, video::Window};

use crate::vertex::{Ball, Vertex};

fn create_window(
    video_subsystem: &VideoSubsystem,
    title: &str,
    width: u32,
    height: u32,
) -> Result<Window, String> {
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 3);

    let window = video_subsystem
        .window(title, width, height)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    Ok(window)
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = create_window(&video_subsystem, "Metaballs Simulation", 800, 600)?;

    let (width, height) = window.size();

    let _ctx = window.gl_create_context().map_err(|e| e.to_string())?;
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    let mut event_pump = sdl_context.event_pump()?;

    let mut balls = vec![
        Ball::new(200.0, 150.0, 50.0, [0.7, 0.3, 0.3]),
        Ball::new(600.0, 450.0, 50.0, [0.3, 0.3, 0.7]),
        Ball::new(400.0, 100.0, 50.0, [0.3, 0.7, 0.3]),
    ];

    let mut ball_vel = vec![
        [100.0f32, 300.0f32],
        [300.0f32, 100.0f32],
        [-100.0f32, -50.0f32],
    ];

    let vecs = vec![
        Vertex { pos: [1.0, 1.0] },
        Vertex { pos: [1.0, -1.0] },
        Vertex { pos: [-1.0, -1.0] },
        Vertex { pos: [-1.0, 1.0] },
    ];

    let indices = vec![0, 1, 2, 2, 3, 0];

    let vbo = BufferObject::new(gl::ARRAY_BUFFER);
    vbo.data(&vecs);

    let mut vao = ArrayObject::new();
    vao.add_attrib(VertexAttrib::<f32>::new(gl::FLOAT, 2));
    vao.set();

    let ibo = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER);
    ibo.data(&indices);

    let ssbo = BufferObject::new(gl::SHADER_STORAGE_BUFFER);
    ssbo.data(&balls);
    unsafe {
        gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, ssbo.id);
    }

    let err_no = unsafe { gl::GetError() };
    if err_no != gl::NO_ERROR {
        println!("Error: {:?}", err_no);
    }

    let vertex_shader = Shader::from_source(
        &CString::new(include_str!("../res/shaders/simple.vert")).unwrap(),
        gl::VERTEX_SHADER,
    )?;

    let fragment_shader = Shader::from_source(
        &CString::new(include_str!("../res/shaders/simple.frag")).unwrap(),
        gl::FRAGMENT_SHADER,
    )?;

    let shader_program = Program::from_shaders(&[vertex_shader, fragment_shader])?;

    let mut pixels = vec![0u8; (width * height * 3) as usize];

    let ball_count_loc =
        unsafe { gl::GetUniformLocation(shader_program.id, c"ball_count".as_ptr()) };

    let mut last_time = SystemTime::now();

    let mut idx = 0;

    'running: loop {
        let dt = SystemTime::now()
            .duration_since(last_time)
            .map_err(|e| e.to_string())?
            .as_secs_f32();
        last_time = SystemTime::now();

        unsafe {
            shader_program.set();
            ssbo.data(&balls);
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, ssbo.id);
            gl::Uniform1i(ball_count_loc, balls.len() as GLint);
            gl::ClearColor(0.0, 0.0, 0.16, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, null());
            let err_no = gl::GetError();
            if err_no != gl::NO_ERROR {
                println!("Error: {:?}", err_no);
            }
        }

        window.gl_swap_window();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::ESCAPE),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        for (ball, vel) in balls.iter_mut().zip(ball_vel.iter_mut()) {
            ball.pos[0] += vel[0] * dt;
            if ball.pos[0] < ball.radius || ball.pos[0] > width as f32 - ball.radius {
                ball.pos[0] = ball.pos[0].clamp(ball.radius, width as f32 - ball.radius);
                vel[0] = -vel[0];
            }

            ball.pos[1] += vel[1] * dt;
            if ball.pos[1] < ball.radius || ball.pos[1] > height as f32 - ball.radius {
                ball.pos[1] = ball.pos[1].clamp(ball.radius, height as f32 - ball.radius);
                vel[1] = -vel[1];
            }
        }

        if idx > 500 {
            continue;
        }
        unsafe {
            gl::PixelStorei(gl::PACK_ALIGNMENT, 1);
            gl::ReadPixels(
                0,
                0,
                width as i32,
                height as i32,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                pixels.as_mut_ptr() as *mut _,
            );
            for y in 0..height / 2 {
                let row1 = (y * width * 3) as usize;
                let row2 = ((height - 1 - y) * width * 3) as usize;

                for i in 0..(width * 3) as usize {
                    pixels.swap(row1 + i, row2 + i);
                }
            }
            let mut file = File::create(format!("img{:03}.ppm", idx)).map_err(|e| e.to_string())?;
            write!(file, "P6\n{} {}\n255\n", width, height).map_err(|e| e.to_string())?;
            file.write_all(&pixels).map_err(|e| e.to_string())?;
            idx += 1;
        }

        // std::thread::sleep(Duration::new(0, 1_000_000_000 / 30));
    }

    Ok(())
}
