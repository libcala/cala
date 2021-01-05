use cala::video::rgb::SRgb32;
use cala::graphics::{
    shader, Group, Shader, ShaderBuilder, ShapeBuilder,
    Transform, Canvas
};
use cala::window::{Frame, input};
use cala::input::{Input, Key};
use cala::task::{exec, wait};

enum Event {
    Redraw(Frame),
    Input(Input),
}

struct State {
    colors: Shader,
    triangle: Group,
    timed: f64,
}

impl State {
    fn event(&mut self, event: Event) {
        match event {
            Event::Redraw(canvas) => self.redraw(canvas),
            Event::Input(input) => self.input(input),
        }
    }

    fn animate_triangle(&mut self, time: f32, aspect: f32) {
        #[rustfmt::skip]
        let vertices = [
             -1.0,  1.0,  1.0, 0.5, 0.0,
              1.0,  1.0,  0.0, 0.0, 1.0,
              0.0, -1.0,  1.0, 1.0, 1.0,
              
              0.0, -1.0,  1.0, 0.7, 0.0,
              1.0,  1.0,  1.0, 0.7, 0.0,
             -1.0,  1.0,  1.0, 0.7, 0.0,
        ];

        let triangle_shape = ShapeBuilder::new()
            .vert(&vertices)
            .face(Transform::new())
            .finish(&self.colors);
        let transform = Transform::new()
            .rotate(0.0, 1.0, 0.0, time)
            .scale(0.25, 0.25 * aspect, 0.25)
            .translate(0.5, 0.5 * aspect, 0.0);
        self.triangle.write(0, &triangle_shape, &transform);
    }

    fn redraw(&mut self, mut frame: Frame) {
        // Update triangle
        self.timed = (self.timed + frame.elapsed().as_secs_f64()) % 1.0;
        self.animate_triangle(self.timed as f32, frame.height());

        // Draw triangle
        frame.draw(&self.colors, &self.triangle);
    }

    fn input(&mut self, input: Input) {
        match input {
            Input::Key(_mods, Key::Back, true) => std::process::exit(0),
            input => println!("{:?}", input),
        }
    }
}

fn start() {
    let timed = 0.0;
    // Load a shader.
    let colors = Shader::new(shader!("color"));

    // Build triangle Shape
    let triangle = Group::new();
    let mut state = State {
        colors,
        triangle,
        timed,
    };

    exec!(state.event(wait!(
        Event::Redraw(Frame::new(SRgb32::new(0.0, 0.5, 0.0)).await),
        Event::Input(input().await),
    )));
}

fn main() {
    // Start the async thread.
    std::thread::spawn(start);
    // Start the draw thread.
    cala::graphics::draw_thread();
}
