use cala::*;

use cala::input::{Input, UiInput, GameInput, TextInput};
use cala::draw::{Group, Shader, ShaderBuilder, ShapeBuilder, shader, Transform, color::SRgb32};

#[allow(unused)]
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

    // Define vertices.
    #[rustfmt::skip]
    let vertices = [
         0.25, 0.75,  1.0, 0.5, 0.0,
         0.75, 0.75,  0.0, 0.0, 1.0,
         0.50, 0.25,  1.0, 1.0, 1.0,
    ];

    // Build triangle Shape
    let mut triangle = Group::new();
    let triangle_shape = ShapeBuilder::new()
        .vert(&vertices)
        .face(Transform::new())
        .finish(&colors);
    triangle.push(
        &triangle_shape,
        &Transform::new()
    );
    /*triangle.push(
        &triangle_shape,
        &Transform::new()
            .scale(0.5, 0.5, 0.5)
            .translate(-0.5, 0.5, 0.0),
    );
    triangle.push(
        &triangle_shape,
        &Transform::new()
            .scale(0.5, 0.5, 0.5)
            .translate(0.5, 0.5, 0.0),
    );*/

    let context = Context {
        colors,
        triangle,
        timed,
    };
    [canvas(context).fut(), input().fut()].select().await;
}

// Function that runs while your app runs.
pub async fn canvas(mut context: Context) {
    loop {
        // Set the background color.
        let mut canvas = pixels::canvas(SRgb32::new(0.0, 1.0, 0.0)).await;

        context.timed = (context.timed + canvas.elapsed().as_secs_f64()) % 1.0;
        let delta = context.timed as f32;

        /*context.triangle.transform(0, Transform::new()
            .scale(0.5, 0.5, 0.5)
            .translate(-0.5, -0.5 + delta, 0.0)
        );*/

        // Draw triangle
        canvas.draw(&context.colors, &context.triangle);
    }
}

async fn input<'a>() {
    loop {
        match cala::input::input().await {
            Input::Ui(UiInput::Back) => break,
            Input::Game(_id, GameInput::Back) => break,
            Input::Text(TextInput::Back) => break,
            input => println!("{:?}", input),
        }
    }
}
