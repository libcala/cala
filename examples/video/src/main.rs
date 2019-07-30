use cala::ShaderBuilder;

#[allow(unused)]
pub struct Context {
    colors: cala::Shader,
    triangle: cala::Shape,
    timed: cala::TimedLoop,
}

// Initialize & set loop to `init()`.
cala::init!(run, {
    // Set the background color.
    cala::background(0.0, 1.0, 0.0);

    // Load a shader.
    let mut colors = cala::shader!("color");

    // Define vertices.
    #[rustfmt::skip]
    let vertices = [
        -0.5, -0.5,  1.0, 0.5, 0.0,
         0.5, -0.5,  0.0, 0.0, 1.0,
         0.0,  0.5,  1.0, 1.0, 1.0,
    ];

    // Build triangle Shape
    let mut triangle =
        cala::Shape::new(cala::shape(&mut colors).vert(&vertices).face([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]));

    triangle.instances(&[
        cala::Transform::new()
            .scale(0.5, 0.5, 0.5)
            .translate(-0.5, 0.0, 0.0),
        cala::Transform::new()
            .scale(0.5, 0.5, 0.5)
            .translate(0.5, 0.0, 0.0),
    ]);

    // Finish building shader.
    cala::build(&colors);

    let timed = cala::TimedLoop::new(1, 0);

    Context {
        colors,
        triangle,
        timed,
    }
});

// Function that runs while your app runs.
pub fn run(context: &mut Context) -> cala::Loop<Context> {
    context.timed.add();
    let delta: f32 = context.timed.into();

    context.triangle.transform(0, cala::Transform::new()
        .scale(0.5, 0.5, 0.5)
        .translate(-0.5, -0.5 + delta, 0.0)
    );

    // Draw triangle
    cala::draw(&context.colors, &context.triangle);

    // Request next frame.
    cala::Continue
}
