pub fn create_frect(pos: (f32, f32), size: (u32, u32)) -> sdl3::render::FRect {
    sdl3::render::FRect::new(
        pos.0 - (size.0 as f32 / 2.0),
        pos.1 - (size.1 as f32 / 2.0),
        size.0 as f32,
        size.1 as f32,
    )
}

pub fn draw_bg(
    pos: (f32, f32),
    size: (f32, f32),
    bg_texture: &sdl3::render::Texture,
    canvas: &mut sdl3::render::Canvas<sdl3::video::Window>,
) {
    let bg_target = sdl3::render::FRect::new(pos.0, pos.1, size.0, size.1);

    canvas.copy(bg_texture, None, bg_target).unwrap();
}

pub fn create_tex<'tex>(
    text_to_render: &str,
    text_color: &sdl3::pixels::Color,
    texture_creator: &'tex sdl3::render::TextureCreator<sdl3::video::WindowContext>,
    font: &sdl3::ttf::Font,
) -> sdl3::render::Texture<'tex> {
    let text_surface = font.render(text_to_render).blended(*text_color).unwrap();
    texture_creator
        .create_texture_from_surface(text_surface)
        .unwrap()
}

pub fn generate_bg_tex<'tex>(
    sizes: (u32, u32),
    color_to_fill: sdl3::pixels::Color,
    texture_creator: &'tex sdl3::render::TextureCreator<sdl3::video::WindowContext>,
) -> sdl3::render::Texture<'tex> {
    let is_rgba = color_to_fill.a != 255;

    let (pixel_format, color_mult) = if is_rgba {
        (sdl3::pixels::PixelFormat::RGBA32, 4u32)
    } else {
        (sdl3::pixels::PixelFormat::RGB24, 3u32)
    };

    let color_vec: Vec<u8> = if is_rgba {
        let color_arr: [u8; 4] = color_to_fill.rgba().into();
        color_arr.to_vec()
    } else {
        let color_arr: [u8; 3] = color_to_fill.rgb().into();
        color_arr.to_vec()
    };

    let mut bg_texture = texture_creator
        .create_texture_target(pixel_format, sizes.0, sizes.1)
        .unwrap();

    let color_bytes_num = (sizes.0 * sizes.1 * color_mult) as usize;
    let mut bg_vec = Vec::with_capacity(color_bytes_num);
    bg_vec.extend(color_vec.iter().cycle().take(color_bytes_num));

    bg_texture
        .update(None, &bg_vec, (sizes.0 * color_mult) as usize)
        .unwrap();

    bg_texture
}

pub fn draw_on_screen(
    pos: (f32, f32),
    local_pos: (f32, f32),
    texture: &sdl3::render::Texture,
    canvas: &mut sdl3::render::Canvas<sdl3::video::Window>,
) {
    let final_pos_x = pos.0 + local_pos.0;
    let final_pos_y = pos.1 + local_pos.1;

    let texture_size = (texture.width(), texture.height());
    let render_target = Some(create_frect((final_pos_x, final_pos_y), texture_size));

    canvas.copy(texture, None, render_target).unwrap();
}

pub fn create_empty_tex<'tex>(
    texture_creator: &'tex sdl3::render::TextureCreator<sdl3::video::WindowContext>,
) -> sdl3::render::Texture<'tex> {
    texture_creator
        .create_texture_target(sdl3::pixels::PixelFormat::RGB24, 1, 1)
        .unwrap()
}
