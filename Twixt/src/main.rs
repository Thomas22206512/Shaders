use macroquad::prelude::*;
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
    let h = h.fract() * 6.0;
    let i = h.floor();
    let f = h - i;
    let p = v * (1.0 - s);
    let q = v * (1.0 - s * f);
    let t = v * (1.0 - s * (1.0 - f));

    match i as i32 {
        0 => Color::new(v, t, p, 1.0),
        1 => Color::new(q, v, p, 1.0),
        2 => Color::new(p, v, t, 1.0),
        3 => Color::new(p, q, v, 1.0),
        4 => Color::new(t, p, v, 1.0),
        5 => Color::new(v, p, q, 1.0),
        _ => Color::new(0.0, 0.0, 0.0, 1.0), // fallback
    }
}
#[macroquad::main("Post processing")]
async fn main() {
    let mut active: bool = true;
    let mut i_frame : u8 = 0;
    loop {
        let time = get_time() as f32;
        // let texture_0 = load_texture("iChannel0.png").await.unwrap();
        // let texture_1 = load_texture("iChannel1.png").await.unwrap();
        // let texture_2 = load_texture("iChannel2.png").await.unwrap();
        // let texture_3 = load_texture("iChannel3.png").await.unwrap();
        let render_target = render_target(screen_width() as u32, screen_height() as u32);
        render_target.texture.set_filter(FilterMode::Nearest);
        if is_key_pressed(KeyCode::Enter) {
            active = !active;
        }

        let material = load_material(
            ShaderSource::Glsl {
                vertex: include_str!("blur.vert"),
                fragment: include_str!("code.frag"),
            },
            MaterialParams {
                uniforms: vec![
                    UniformDesc::new("iTime", UniformType::Float1),
                    UniformDesc::new("iResolution", UniformType::Float2),
                    UniformDesc::new("mouse", UniformType::Float3),
                    UniformDesc::new("texel_size", UniformType::Float2),
                    // UniformDesc::new("iChannel0",UniformType::Int4),
                    // UniformDesc::new("iChannel1",UniformType::Int4),
                    // UniformDesc::new("iChannel2",UniformType::Int4),
                    // UniformDesc::new("iChannel3",UniformType::Int4),
                    // UniformDesc::new("iChannelResolution",UniformType::Mat4),
                    // UniformDesc::new("iFrame", UniformType::Int1)
                ],
                ..Default::default()
            },
        )
        .unwrap();
        material.set_uniform("iTime", time);
        material.set_uniform("iResolution", vec2(screen_width(), screen_height()));
        material.set_uniform("mouse", vec3(mouse_position().0, mouse_position().1, is_mouse_button_down(MouseButton::Left) as i32 as f32));
        let texel_size = vec2(1.0 / screen_width(), 1.0 / screen_height());
        material.set_uniform("texel_size", texel_size);
        // material.set_uniform("iChannel0", texture_0);  // Channel 0
        // material.set_uniform("iChannel1", texture_1);
        // material.set_uniform("iChannel2", texture_2);  // Channel 0
        // material.set_uniform("iChannel3", texture_3);
        // material.set_uniform("iChannelResolution", vec![
        //     vec3(1075.0, 1077.0, 0.0), // Résolution pour iChannel0
        //     vec3(1492.0, 691.0, 0.0),   // Résolution pour iChannel1
        //     vec3(1074.0, 1530.0, 0.0),   // Résolution pour iChannel2
        //     vec3(234.0, 123.0, 0.0),  // Résolution pour iChannel3
        // ]);
        // material.set_uniform("iFrame", &i_frame);
        // drawing to the texture
        // 0..100, 0..100 camera
        set_camera(&Camera2D {
            zoom: vec2(0.01, 0.01),
            target: vec2(0.0, 0.0),
            render_target: Some(render_target.clone()),
            ..Default::default()
        });

        clear_background(BLACK);
        for x in 0..2*screen_width() as usize {
            let t = x as f32 / (screen_width() + 255.0 as f32);
            let color = hsv_to_rgb(t, 1.25, 1.0); // HSV → dégradé arc-en-ciel
            draw_rectangle(x as f32 - screen_width(), 0.0 - screen_height(), 1.0, 2.*screen_height() as f32, color);
        }
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
            draw_texture(&render_target.texture, 0.0, 0.0, WHITE);
        }
        i_frame = (1 + i_frame)%10;
        next_frame().await;
    }
}
