use macroquad::prelude::*;

use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Label, TreeNode},
};

use macroquad::color;

enum Uniform {
    Float1(String),
    Float2(String, String),
    Float3(String, String, String),
    Color(Vec3),
}

impl Uniform {
    fn uniform_type(&self) -> UniformType {
        match self {
            Uniform::Float1(_) => UniformType::Float1,
            Uniform::Float2(_, _) => UniformType::Float2,
            Uniform::Float3(_, _, _) => UniformType::Float3,
            Uniform::Color(_) => UniformType::Float3,
        }
    }
}

fn color_picker_texture(w: usize, h: usize) -> (Texture2D, Image) {
    let ratio = 1.0 / h as f32;

    let mut image = Image::gen_image_color(w as u16, h as u16, WHITE);
    let image_data = image.get_image_data_mut();

    for j in 0..h {
        for i in 0..w {
            let lightness = 1.0 - i as f32 * ratio;
            let hue = j as f32 * ratio;

            image_data[i + j * w] = color::hsl_to_rgb(hue, 1.0, lightness).into();
        }
    }

    (Texture2D::from_image(&image), image)
}

#[macroquad::main("Shadertoy")]
async fn main() {
    let ferris = load_texture("image.png").await.unwrap();
    let (color_picker_texture, color_picker_image) = color_picker_texture(200, 200);

    let mut fragment_shader = DEFAULT_FRAGMENT_SHADER.to_string();
    let mut vertex_shader = DEFAULT_VERTEX_SHADER.to_string();

    let pipeline_params = PipelineParams {
        depth_write: true,
        depth_test: Comparison::LessOrEqual,
        ..Default::default()
    };

    let mut material = load_material(
        ShaderSource::Glsl {
            vertex: &vertex_shader,
            fragment: &fragment_shader,
        },
        MaterialParams {
            pipeline_params,
            ..Default::default()
        },
    )
    .unwrap();
    let mut error: Option<String> = None;

    enum Mesh {
        Sphere,
        Cube,
        Plane,
    }
    let mut mesh = Mesh::Sphere;

    let mut camera = Camera3D {
        position: vec3(-15., 15., -5.),
        up: vec3(0., 1., 0.),
        target: vec3(0., 0., 0.),
        ..Default::default()
    };

    let mut colorpicker_window = false;

    let mut new_uniform_window = false;
    let mut new_uniform_name = String::new();
    let mut uniforms: Vec<(String, Uniform)> = vec![];

    loop {
        set_camera(&Camera2D {
            zoom: vec2(1., screen_width() / screen_height()),
            ..Default::default()
        });
        if is_key_down(KeyCode::Up) {
            camera.position.x = camera.position.x + 0.1;
        }
        if is_key_down(KeyCode::Down) {
            camera.position.x = camera.position.x - 0.1;
        }
        if is_key_down(KeyCode::Space) {
            camera.position.y = camera.position.y + 0.1;
        }
        if is_key_down(KeyCode::LeftShift) {
            camera.position.y = camera.position.y - 0.1;
        }
        if is_key_down(KeyCode::Right) {
            camera.position.z = camera.position.z + 0.1;
        }
        if is_key_down(KeyCode::Left) {
            camera.position.z = camera.position.z - 0.1;
        }
        clear_background(WHITE);

        set_camera(&camera);

        // draw_grid(
        //     20,
        //     1.,
        //     Color::new(0.55, 0.55, 0.55, 0.75),
        //     Color::new(0.75, 0.75, 0.75, 0.75),
        // );

        gl_use_material(&material);
        match mesh {
            Mesh::Plane => draw_plane(vec3(0., 2., 0.), vec2(5., 5.), Some(&ferris), WHITE),
            Mesh::Sphere => draw_sphere(vec3(0., 6., 0.), 5., Some(&ferris), WHITE),
            Mesh::Cube => draw_cube(vec3(0., 5., 0.), vec3(10., 10., 10.), Some(&ferris), WHITE),
        }
        gl_use_default_material();

        // set_default_camera();

        draw_circle(-10., 20., 2., BLUE);

        next_frame().await
    }
}


const DEFAULT_FRAGMENT_SHADER: &'static str = "#version 100
precision lowp float;


varying vec4 color;

varying vec2 uv;


uniform sampler2D Texture;


// https://www.shadertoy.com/view/XtlSD7


vec2 CRTCurveUV(vec2 uv)

{

    uv = uv * 2.0 - 1.0;

    vec2 offset = abs( uv.yx ) / vec2( 6.0, 4.0 );

    uv = uv + uv * offset * offset;

    uv = uv * 0.5 + 0.5;

    return uv;

}


void DrawVignette( inout vec3 color, vec2 uv )

{

    float vignette = uv.x * uv.y * ( 1.0 - uv.x ) * ( 1.0 - uv.y );

    vignette = clamp( pow( 16.0 * vignette, 0.3 ), 0.0, 1.0 );

    color *= vignette;

}



void DrawScanline( inout vec3 color, vec2 uv )

{

    float iTime = 0.1;

    float scanline 	= clamp( 0.95 + 0.05 * cos( 3.14 * ( uv.y + 0.008 * iTime ) * 240.0 * 1.0 ), 0.0, 1.0 );

    float grille 	= 0.85 + 0.15 * clamp( 1.5 * cos( 3.14 * uv.x * 640.0 * 1.0 ), 0.0, 1.0 );

    color *= scanline * grille * 1.2;

}


void main() {

    vec2 crtUV = CRTCurveUV(uv);

    vec3 res = texture2D(Texture, uv).rgb * color.rgb;

    if (crtUV.x < 0.0 || crtUV.x > 1.0 || crtUV.y < 0.0 || crtUV.y > 1.0)

    {

        res = vec3(0.0, 0.0, 0.0);

    }

    DrawVignette(res, crtUV);

    DrawScanline(res, uv);

    gl_FragColor = vec4(res, 1.0);


}
";

const DEFAULT_VERTEX_SHADER: &'static str = "#version 100
attribute vec3 position;

attribute vec2 texcoord;

attribute vec4 color0;


varying lowp vec2 uv;

varying lowp vec4 color;


uniform mat4 Model;

uniform mat4 Projection;


void main() {

    gl_Position = Projection * Model * vec4(position, 1);

    color = color0 / 255.0;

    uv = texcoord;

}
";