extern crate gl;
extern crate glutin;

use glutin::{event::*, event_loop::ControlFlow, window::WindowBuilder, ContextBuilder};
use std::ffi::CString;
use std::ptr;
use std::str;

fn main() {
    // 创建事件循环
    let event_loop = glutin::event_loop::EventLoop::new();
    
    // 创建窗口和 OpenGL 上下文
    let window_builder = WindowBuilder::new().with_title("Rust + OpenGL + GLSL");
    let windowed_context = ContextBuilder::new().build_windowed(window_builder, &event_loop).unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    // 加载 OpenGL 函数
    gl::load_with(|s| windowed_context.get_proc_address(s));

    // 定义 GLSL 顶点着色器代码
    let vertex_shader_source = CString::new(include_str!("shader.vert")).unwrap();
    // 定义 GLSL 片段着色器代码
    let fragment_shader_source = CString::new(include_str!("shader.frag")).unwrap();

    // 编译顶点着色器
    let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
    unsafe {
        gl::ShaderSource(vertex_shader, 1, &vertex_shader_source.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);
    }
    check_shader_compile_status(vertex_shader);

    // 编译片段着色器
    let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
    unsafe {
        gl::ShaderSource(fragment_shader, 1, &fragment_shader_source.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);
    }
    check_shader_compile_status(fragment_shader);

    // 链接着色器程序
    let shader_program = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
    }
    check_program_link_status(shader_program);

    // 主事件循环
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                // 清除屏幕
                unsafe {
                    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                    gl::UseProgram(shader_program);
                }

                // 交换缓冲区
                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}

// 检查着色器编译状态
fn check_shader_compile_status(shader: u32) {
    let mut success = 0;
    let mut info_log = vec![0; 512];
    unsafe {
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            gl::GetShaderInfoLog(shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut i8);
            println!("Shader Compilation Failed: {:?}", str::from_utf8(&info_log).unwrap());
        }
    }
}

// 检查着色器程序链接状态
fn check_program_link_status(program: u32) {
    let mut success = 0;
    let mut info_log = vec![0; 512];
    unsafe {
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            gl::GetProgramInfoLog(program, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut i8);
            println!("Program Linking Failed: {:?}", str::from_utf8(&info_log).unwrap());
        }
    }
}
