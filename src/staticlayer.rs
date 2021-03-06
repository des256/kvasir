// G - static layer
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;

pub struct StaticLayer {
    pub(crate) _engine: Rc<Engine>,
    pub(crate) framebuffer: Rc<e::Framebuffer>,
}

impl StaticLayer {
    pub fn new_from_mat(engine: &Rc<Engine>,image: Mat<pixel::ARGB8>) -> Result<StaticLayer,EngineError> {
        let framebuffer = Rc::new(e::Framebuffer::new(&engine.graphics,engine.framebuffer.size).expect("StaticLayer::new_from_mat: Unable to create framebuffer."));
        let texture = e::Texture2D::new_from_mat(&engine.graphics,image).expect("StaticLayer::new_from_mat: Unabel to create texture.");
        engine.graphics.bind_target(&*framebuffer);
        engine.graphics.clear(0xFFFFFF00);
        engine.graphics.bind_texture(0,&texture);
        engine.graphics.bind_shader(&engine.static_shader);
        engine.graphics.set_uniform("u_texture",0);
        engine.graphics.bind_vertexbuffer(&engine.quad_vertexbuffer);
        engine.graphics.draw_triangle_fan(4);
        Ok(StaticLayer {
            _engine: Rc::clone(engine),
            framebuffer: framebuffer,
        })
    }
}

impl Layer for StaticLayer {
    fn framebuffer(&self) -> &gpu::Framebuffer {
        &*self.framebuffer
    }

    fn render(&self) {
    }
}
