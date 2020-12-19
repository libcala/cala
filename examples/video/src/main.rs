use cala::*;

use cala::draw::{
    color::SRgb32, shader, Group, Shader, ShaderBuilder, ShapeBuilder,
    Transform,
};
use cala::input::{GameInput, Input, TextInput, UiInput};

pub struct Context {
    colors: Shader,
    triangle: Group,
    timed: f64,
}

// Initialize & set loop to `init()`.
cala::exec!(init);

async fn init() {
    let timed = 0.0;
    // Load a shader.
    let colors = Shader::new(shader!("color"));

    // Build triangle Shape
    let triangle = Group::new();
    let mut context = Context {
        colors,
        triangle,
        timed,
    };

    // Tasks
    task! {
        let canvas = async { loop { canvas(&mut context).await } };
        let input = async { while input().await { } };
    }

    // Game loop
    poll![canvas, input].await;
}

fn animate_triangle(context: &mut Context, time: f32, aspect: f32) {
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
        .finish(&context.colors);
    let transform = Transform::new()
        .rotate(0.0, 1.0, 0.0, time)
        .scale(0.25, 0.25 * aspect, 0.25)
        .translate(0.5, 0.5 * aspect, 0.0);
    context.triangle.write(0, &triangle_shape, &transform);
}

// Function that runs while your app runs.
pub async fn canvas(context: &mut Context) {
    // Set the background color.
    let mut canvas = pixels::canvas(SRgb32::new(0.0, 0.5, 0.0)).await;

    // Update triangle
    context.timed = (context.timed + canvas.elapsed().as_secs_f64()) % 1.0;
    animate_triangle(context, context.timed as f32, canvas.height());

    // Draw triangle
    canvas.draw(&context.colors, &context.triangle);
}

async fn input<'a>() -> bool {
    match cala::input::input().await {
        Input::Ui(UiInput::Back) => return false,
        Input::Game(_id, GameInput::Back) => return false,
        Input::Text(TextInput::Back) => return false,
        input => println!("{:?}", input),
    }
    true
}
