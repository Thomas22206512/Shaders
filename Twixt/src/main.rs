use macroquad::prelude::*;

#[macroquad::main("Post processing")]
async fn main() {
    let mut active : bool = true;
    loop {
        let time = get_time() as f32;
        let render_target = render_target(screen_width() as u32, screen_height() as u32);
        render_target.texture.set_filter(FilterMode::Nearest);
        if is_key_pressed(KeyCode::Enter) {
            active = !active;
        }

        let material = load_material(
            ShaderSource::Glsl {
                vertex: include_str!("blur.vert"),
                fragment: include_str!("blur1.frag"),
            },
            MaterialParams {
                uniforms: vec![UniformDesc::new("time", UniformType::Float1)],
                ..Default::default()
            },
        )
        .unwrap();
        material.set_uniform("time", time);
        // drawing to the texture
        // 0..100, 0..100 camera
        set_camera(&Camera2D {
            zoom: vec2(0.01, 0.01),
            target: vec2(0.0, 0.0),
            render_target: Some(render_target.clone()),
            ..Default::default()
        });

        clear_background(LIGHTGRAY);
        draw_line(-30.0, 45.0, 30.0, 45.0, 3.0, BLUE);
        draw_circle(-45.0, -35.0, 20.0, YELLOW);
        draw_circle(45.0, -35.0, 20.0, GREEN);
        set_default_camera();
        // drawing to the screen
        if active {

            clear_background(WHITE);
            gl_use_material(&material);
            draw_texture_ex(
                &render_target.texture,
                0.,
                0.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height())),
                    ..Default::default()
                },
            );
            gl_use_default_material();
        } else {
            clear_background(WHITE);
            draw_texture(
                &render_target.texture,
                0.0,
                0.0,
                WHITE,
            );
        }

        next_frame().await;
    }
}
