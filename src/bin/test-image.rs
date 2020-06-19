// Kvasir - System Interface - Image test
// Desmond Germans, 2020

use kvasir::*;

use std::fs;
use std::io::prelude::*;

fn main() {
    let mut video = match Video::new(kvasir::VideoConfig {
        window: WindowConfig::High,
        framebuffer: FramebufferConfig::Low,
    }) {
        Ok(video) => video,
        Err(_) => { panic!("Cannot open video."); },
    };
    video.set_window_title("Image Test");
    let layer = Layer::new(0,0,320,180);
    let mut file = fs::File::open("try/test.png").expect("cannot open file");
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).expect("unable to read file");
    let image = decode(&buffer).expect("unable to decode");
    let mut texture = Texture2D::new(320,180);
    texture.upload(0,0,&image);

    let vs = r#"
        #version 420 core
        layout(location = 0) in vec2 v_pos;
        out vec2 f_tex;
        void main() {
            f_tex = vec2(v_pos.x,v_pos.y);
            gl_Position = vec4(-1.0 + 2.0 * v_pos.x,-1.0 + 2.0 * v_pos.y,0.0,1.0);
        }
    "#;
    let fs = r#"
        #version 420 core
        uniform sampler2D u_texture;
        in vec2 f_tex;
        out vec4 fs_output;
        void main() {
            fs_output = texture2D(u_texture,f_tex);
        }
    "#;
    let shader = Shader::new(vs,None,fs);
    unsafe {
        println!("rendering test image");
        layer.bind();
        gl::ClearColor(1.0,1.0,0.0,1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        texture.bind();
        shader.bind();
        shader.set_uniform("u_texture",0);
        gl::BindBuffer(gl::ARRAY_BUFFER,video.quad_vbo);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0,2,gl::FLOAT,gl::FALSE,0,0 as *const gl::types::GLvoid);
        gl::DrawArrays(gl::TRIANGLE_FAN,0,4);
        gl::DisableVertexAttribArray(0);
        gl::Flush();
        layer.unbind();
    }
    video.layers.push(layer);
    loop {
        while let Some(event) = video.next_event() {
            match event {
                Event::Close => {
                    return;
                },
                _ => { },
            }    
        }
    }
}