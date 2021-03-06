// G - Image test
// Desmond Germans, 2020

use e::*;
use g::*;
use std::{
    rc::Rc,
    time::Instant,
};

//const MS_PER_US: f32 = 0.001;

fn main() {
    // open system
    let system = Rc::new(System::new().expect("Cannot open system."));

    // create GPU graphics context
    let graphics = Rc::new(gpu::Graphics::new(&system).expect("Cannot open graphics."));

    // create game engine
    let engine = Rc::new(Engine::new(&system,&graphics,vec2!(1024,576),vec2!(256,144)).expect("Cannot open engine."));
    
    // load image
    let image = imageformats::load("try/256x144.png").expect("cannot read image");

    // create static layer for game engine
    let layer = Rc::new(StaticLayer::new_from_mat(&engine,image).expect("cannot create layer"));

    // create layer collection
    let layers: Vec<Rc<dyn Layer>> = vec![layer];

    // main loop
    let time = Instant::now();
    let mut prev_frame_us = 0;
    
    while engine.is_running() {
        let frame_us = time.elapsed().as_micros();
        let delta = (frame_us - prev_frame_us) as u32;  // calculate duration of previous frame (in us)

        engine.update(&layers);

        let update_us = time.elapsed().as_micros();

        engine.render(&layers);

        let render_us = time.elapsed().as_micros();

        engine.present();

        let present_us = time.elapsed().as_micros();

        println!("update: {:5}  draw: {:5}  ({:5} remaining)  {:2} Hz",
            update_us - frame_us,
            render_us - update_us,
            present_us - render_us,
            (1000000.0 / (delta as f32)) as u32,
        );
        
        prev_frame_us = frame_us;
    }
}
