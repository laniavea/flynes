use crate::layouts::InstructionsLogLayout;
use crate::shared::{create_empty_tex, create_tex, draw_bg, draw_on_screen, generate_bg_tex};

const INSTUCTION_LOG_LAYOUT: InstructionsLogLayout = InstructionsLogLayout::new(0.1, 0.3, 0.1, 0.3);
const TEXT_COLOR: sdl3::pixels::Color = sdl3::pixels::Color::RGBA(255, 255, 255, 255);
const BG_COLOR: sdl3::pixels::Color = sdl3::pixels::Color::RGB(100, 100, 100);

pub struct InstructionsLogLocalPos {
    header: (f32, f32),
}

impl InstructionsLogLocalPos {
    fn new(size_x: f32, size_y: f32) -> InstructionsLogLocalPos {
        let header = INSTUCTION_LOG_LAYOUT.get_header(size_x, size_y);

        InstructionsLogLocalPos { header }
    }
}

pub struct CachedInstructionLogState {}

impl CachedInstructionLogState {
    fn new() -> CachedInstructionLogState {
        CachedInstructionLogState {}
    }
}

pub struct InstructionsLogTextures<'tex> {
    bg: sdl3::render::Texture<'tex>,
    header: sdl3::render::Texture<'tex>,
}

impl<'tex> InstructionsLogTextures<'tex> {
    fn init(
        texture_creator: &'tex sdl3::render::TextureCreator<sdl3::video::WindowContext>,
    ) -> InstructionsLogTextures<'tex> {
        InstructionsLogTextures {
            bg: create_empty_tex(texture_creator),
            header: create_empty_tex(texture_creator),
        }
    }

    pub fn new(
        sizes: (f32, f32),
        header_font: &sdl3::ttf::Font,
        texture_creator: &'tex sdl3::render::TextureCreator<sdl3::video::WindowContext>,
    ) -> InstructionsLogTextures<'tex> {
        let mut textures = InstructionsLogTextures::init(texture_creator);

        update_static(sizes, &mut textures, header_font, texture_creator);

        textures
    }
}

fn update_static<'tex>(
    sizes: (f32, f32),
    textures: &mut InstructionsLogTextures<'tex>,
    header_font: &sdl3::ttf::Font,
    texture_creator: &'tex sdl3::render::TextureCreator<sdl3::video::WindowContext>,
) {
    let sizes_u32 = (sizes.0 as u32, sizes.1 as u32);
    let bg_texture = generate_bg_tex(sizes_u32, BG_COLOR, texture_creator);

    let header_texture = create_tex(
        "INSTRUCTION LOG[3]",
        &TEXT_COLOR,
        texture_creator,
        header_font,
    );

    textures.bg = bg_texture;
    textures.header = header_texture;
}

pub struct InstructionsLogRenderData<'tex> {
    size_x: f32,
    size_y: f32,
    cached_state: CachedInstructionLogState,
    local_pos: InstructionsLogLocalPos,
    textures: InstructionsLogTextures<'tex>,
}

impl<'tex> InstructionsLogRenderData<'tex> {
    pub fn new(
        size_x: f32,
        size_y: f32,
        texture_creator: &'tex sdl3::render::TextureCreator<sdl3::video::WindowContext>,
        header_font: &sdl3::ttf::Font,
    ) -> InstructionsLogRenderData<'tex> {
        let cached_state = CachedInstructionLogState::new();
        let local_pos = InstructionsLogLocalPos::new(size_x, size_y);

        let sizes = (size_x, size_y);
        let textures = InstructionsLogTextures::new(sizes, header_font, texture_creator);

        InstructionsLogRenderData {
            size_x,
            size_y,
            cached_state,
            local_pos,
            textures,
        }
    }
}

pub fn draw_block<'tex>(
    block: (f32, f32),
    render_data: &InstructionsLogRenderData<'tex>,
    canvas: &mut sdl3::render::Canvas<sdl3::video::Window>,
) {
    let bg_size = (render_data.size_x, render_data.size_y);
    draw_bg(block, bg_size, &render_data.textures.bg, canvas);

    let header_local_pos = render_data.local_pos.header;
    draw_on_screen(
        block,
        header_local_pos,
        &render_data.textures.header,
        canvas,
    );
}
