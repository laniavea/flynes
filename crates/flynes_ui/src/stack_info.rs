use crate::layouts::StackLayout;
use crate::shared::{
    create_empty_tex, create_frect, create_tex, draw_bg, draw_on_screen, generate_bg_tex,
};

const STACK_LAYOUT: StackLayout = StackLayout::new(0.2, 0.8);
const TEXT_COLOR: sdl3::pixels::Color = sdl3::pixels::Color::RGBA(255, 255, 255, 255);
const BG_COLOR: sdl3::pixels::Color = sdl3::pixels::Color::RGB(80, 80, 80);
const MARKER_BG_COLOR: sdl3::pixels::Color = sdl3::pixels::Color::RGBA(0, 255, 0, 127);

const STACK_SIZE: usize = 256;
const STACK_ROWS: usize = 16;
const STACK_ELEMS_IN_ROW: usize = STACK_SIZE / STACK_ROWS;

pub struct StackLocalPos {
    header: (f32, f32),
    first_col_block_pos: (f32, f32),
    first_col_block_size: (f32, f32),
    first_col_elems_pos: [(f32, f32); STACK_ELEMS_IN_ROW],
    stack_rows_block_pos: [(f32, f32); STACK_ROWS],
    stack_rows_block_size: (f32, f32),
    stack_rows_elems_pos: [(f32, f32); STACK_ELEMS_IN_ROW],
    stack_rows_elems_size: (f32, f32),
}

impl StackLocalPos {
    fn new(size_x: f32, size_y: f32) -> StackLocalPos {
        let header = STACK_LAYOUT.get_header(size_x, size_y);

        let (first_col_block_pos, first_col_block_size) =
            STACK_LAYOUT.get_stack_first_col_block(size_x, size_y);
        let first_col_elems_pos = STACK_LAYOUT.get_stack_first_col_elems(size_x, size_y);

        let (stack_rows_block_size, stack_rows_block_pos) =
            STACK_LAYOUT.get_stack_rows_block(size_x, size_y);
        let (stack_rows_elems_size, stack_rows_elems_pos) =
            STACK_LAYOUT.get_stack_rows_elems(stack_rows_block_size.0, stack_rows_block_size.1);

        StackLocalPos {
            header,
            first_col_block_pos,
            first_col_block_size,
            first_col_elems_pos,
            stack_rows_block_pos,
            stack_rows_block_size,
            stack_rows_elems_pos,
            stack_rows_elems_size,
        }
    }

    fn get_marker_local_pos(&self, sp_row: usize, sp_el: usize) -> (f32, f32) {
        let sp_mark_x = self.stack_rows_block_pos[sp_row].0 - (self.stack_rows_block_size.0 / 2.0)
            + self.stack_rows_elems_pos[sp_el].0;

        let sp_mark_y = self.stack_rows_block_pos[sp_row].1 - (self.stack_rows_block_size.1 / 2.0)
            + self.stack_rows_elems_pos[sp_el].1;

        (sp_mark_x, sp_mark_y)
    }
}

pub struct CachedStackState {
    stack: [u8; 256],
}

impl CachedStackState {
    fn new(bus: &flynes_core::bus::Bus) -> CachedStackState {
        let stack_values_slice = bus.memory().stack_as_slice();
        assert!(stack_values_slice.len() == STACK_SIZE);

        let mut stack_values: [u8; STACK_SIZE] = [0u8; STACK_SIZE];
        stack_values.copy_from_slice(stack_values_slice);

        CachedStackState {
            stack: stack_values,
        }
    }
}

pub struct StackTextures<'tex> {
    bg: sdl3::render::Texture<'tex>,
    header: sdl3::render::Texture<'tex>,
    first_col: sdl3::render::Texture<'tex>,
    data_row: [sdl3::render::Texture<'tex>; 16],
    marker_bg: sdl3::render::Texture<'tex>,
    digits: &'tex Vec<sdl3::render::Texture<'tex>>,
}

impl<'tex> StackTextures<'tex> {
    fn init(
        digits: &'tex Vec<sdl3::render::Texture<'tex>>,
        texture_creator: &'tex sdl3::render::TextureCreator<sdl3::video::WindowContext>,
    ) -> StackTextures<'tex> {
        StackTextures {
            bg: create_empty_tex(texture_creator),
            header: create_empty_tex(texture_creator),
            first_col: create_empty_tex(texture_creator),
            data_row: std::array::from_fn(|_| create_empty_tex(texture_creator)),
            marker_bg: create_empty_tex(texture_creator),
            digits,
        }
    }

    pub fn new(
        sizes: (f32, f32),
        stack_info: (&mut CachedStackState, &StackLocalPos),
        bus: &flynes_core::bus::Bus,
        digits: &'tex Vec<sdl3::render::Texture<'tex>>,
        header_font: &sdl3::ttf::Font,
        texture_creator: &'tex sdl3::render::TextureCreator<sdl3::video::WindowContext>,
        canvas: &mut sdl3::render::Canvas<sdl3::video::Window>,
    ) -> StackTextures<'tex> {
        let mut textures = StackTextures::init(digits, texture_creator);

        let (stack_state, stack_local_pos) = (stack_info.0, stack_info.1);

        update_static(
            sizes,
            stack_local_pos,
            &mut textures,
            header_font,
            texture_creator,
            canvas,
        );

        update_stack_data(
            stack_local_pos,
            &mut textures,
            stack_state,
            bus,
            texture_creator,
            canvas,
            true,
        );

        textures
    }
}

fn update_static<'tex>(
    sizes: (f32, f32),
    stack_local_pos: &StackLocalPos,
    textures: &mut StackTextures<'tex>,
    header_font: &sdl3::ttf::Font,
    texture_creator: &'tex sdl3::render::TextureCreator<sdl3::video::WindowContext>,
    canvas: &mut sdl3::render::Canvas<sdl3::video::Window>,
) {
    let sizes_u32 = (sizes.0 as u32, sizes.1 as u32);
    let bg_texture = generate_bg_tex(sizes_u32, BG_COLOR, texture_creator);

    let pointer_size_x = stack_local_pos.stack_rows_elems_size.0 as u32;
    let pointer_size_y = stack_local_pos.stack_rows_elems_size.1 as u32;
    let marker_bg_texture = generate_bg_tex(
        (pointer_size_x, pointer_size_y),
        MARKER_BG_COLOR,
        texture_creator,
    );

    let header_texture = create_tex("STACK[2]", &TEXT_COLOR, texture_creator, header_font);
    let first_col_block_size = stack_local_pos.first_col_block_size;

    let mut first_col_texture = texture_creator
        .create_texture_target(
            sdl3::pixels::PixelFormat::RGB24,
            first_col_block_size.0 as u32,
            first_col_block_size.1 as u32,
        )
        .unwrap();

    canvas
        .with_texture_canvas(&mut first_col_texture, |texture_canvas| {
            let old_color = texture_canvas.draw_color();
            texture_canvas.set_draw_color(BG_COLOR);
            texture_canvas.clear();
            for (row_id, row_local_pos) in stack_local_pos.first_col_elems_pos.iter().enumerate() {
                let row_header_tex = &textures.digits[row_id * 16];
                let pos_x = row_local_pos.0;
                let pos_y = row_local_pos.1;

                let header_size = (row_header_tex.width(), row_header_tex.height());
                let header_target = Some(create_frect((pos_x, pos_y), header_size));

                texture_canvas
                    .copy(row_header_tex, None, header_target)
                    .unwrap();
            }
            texture_canvas.set_draw_color(old_color);
        })
        .unwrap();

    textures.bg = bg_texture;
    textures.header = header_texture;
    textures.first_col = first_col_texture;
    textures.marker_bg = marker_bg_texture;
}

pub struct StackRenderData<'tex> {
    size_x: f32,
    size_y: f32,
    stack_state: CachedStackState,
    local_pos: StackLocalPos,
    textures: StackTextures<'tex>,
}

impl<'tex> StackRenderData<'tex> {
    pub fn new(
        stack_size: (f32, f32),
        bus: &flynes_core::bus::Bus,
        texture_creator: &'tex sdl3::render::TextureCreator<sdl3::video::WindowContext>,
        canvas: &mut sdl3::render::Canvas<sdl3::video::Window>,
        digits_textures: &'tex Vec<sdl3::render::Texture>,
        header_font: &sdl3::ttf::Font,
    ) -> StackRenderData<'tex> {
        let (size_x, size_y) = stack_size;

        let local_pos = StackLocalPos::new(size_x, size_y);
        let mut stack_state = CachedStackState::new(bus);
        let textures = StackTextures::new(
            stack_size,
            (&mut stack_state, &local_pos),
            bus,
            digits_textures,
            header_font,
            texture_creator,
            canvas,
        );

        StackRenderData {
            size_x,
            size_y,
            stack_state,
            local_pos,
            textures,
        }
    }
}

fn update_stack_data<'tex>(
    stack_local_pos: &StackLocalPos,
    textures: &mut StackTextures<'tex>,
    stack_state: &mut CachedStackState,
    bus: &flynes_core::bus::Bus,
    texture_creator: &'tex sdl3::render::TextureCreator<sdl3::video::WindowContext>,
    canvas: &mut sdl3::render::Canvas<sdl3::video::Window>,
    force_rerender: bool,
) {
    let mut stack_data_tex: [Option<sdl3::render::Texture<'tex>>; 16] = [const { None }; 16];
    let actual_stack = bus.memory().stack_as_slice();

    let mut rows_to_rerender: Vec<usize> = Vec::new();
    let mut stack_value_id = 0usize;
    while stack_value_id < STACK_SIZE {
        if stack_state.stack[stack_value_id] != actual_stack[stack_value_id] {
            let row_id = stack_value_id / STACK_ELEMS_IN_ROW;
            rows_to_rerender.push(row_id);
            stack_value_id = STACK_ELEMS_IN_ROW * (row_id + 1);
            continue;
        }
        stack_value_id += 1;
    }
    stack_state.stack.copy_from_slice(actual_stack);
    if force_rerender {
        rows_to_rerender = (0..STACK_ROWS).collect();
    }

    for row_id in rows_to_rerender {
        let mut row_texture = texture_creator
            .create_texture_target(
                sdl3::pixels::PixelFormat::RGB24,
                stack_local_pos.stack_rows_block_size.0 as u32,
                stack_local_pos.stack_rows_block_size.1 as u32,
            )
            .unwrap();

        canvas
            .with_texture_canvas(&mut row_texture, |texture_canvas| {
                let old_color = texture_canvas.draw_color();
                texture_canvas.set_draw_color(BG_COLOR);
                texture_canvas.clear();
                for (value_id, new_stack_value) in stack_state.stack
                    [row_id * STACK_ELEMS_IN_ROW..(row_id + 1) * STACK_ELEMS_IN_ROW]
                    .iter()
                    .enumerate()
                {
                    let value_tex = &textures.digits[*new_stack_value as usize];
                    let pos_x = stack_local_pos.stack_rows_elems_pos[value_id].0;
                    let pos_y = stack_local_pos.stack_rows_elems_pos[value_id].1;

                    let value_size = (value_tex.width(), value_tex.height());
                    let value_target = Some(create_frect((pos_x, pos_y), value_size));

                    texture_canvas.copy(value_tex, None, value_target).unwrap();
                }
                texture_canvas.set_draw_color(old_color);
            })
            .unwrap();

        stack_data_tex[row_id] = Some(row_texture);
    }

    for (stack_row_id, stack_row_tex) in stack_data_tex.into_iter().enumerate() {
        if let Some(new_stack_row_tex) = stack_row_tex {
            textures.data_row[stack_row_id] = new_stack_row_tex
        }
    }
}

pub fn update_textures<'tex>(
    stack_render_data: &mut StackRenderData<'tex>,
    bus: &flynes_core::bus::Bus,
    texture_creator: &'tex sdl3::render::TextureCreator<sdl3::video::WindowContext>,
    canvas: &mut sdl3::render::Canvas<sdl3::video::Window>,
    force_rerender: bool,
) {
    let stack_local_pos = &stack_render_data.local_pos;
    let textures = &mut stack_render_data.textures;
    let stack_state = &mut stack_render_data.stack_state;

    update_stack_data(
        stack_local_pos,
        textures,
        stack_state,
        bus,
        texture_creator,
        canvas,
        force_rerender,
    );
}

pub fn draw_stack_info<'tex>(
    block: (f32, f32),
    stack_render_data: &mut StackRenderData<'tex>,
    canvas: &mut sdl3::render::Canvas<sdl3::video::Window>,
) {
    let bg_size = (stack_render_data.size_x, stack_render_data.size_y);
    draw_bg(block, bg_size, &stack_render_data.textures.bg, canvas);

    let old_draw_color = canvas.draw_color();
    canvas.set_draw_color(BG_COLOR);

    draw_on_screen(
        block,
        stack_render_data.local_pos.header,
        &stack_render_data.textures.header,
        canvas,
    );

    draw_on_screen(
        block,
        stack_render_data.local_pos.first_col_block_pos,
        &stack_render_data.textures.first_col,
        canvas,
    );

    for (row_local_pos, row_tex) in stack_render_data
        .local_pos
        .stack_rows_block_pos
        .iter()
        .zip(&stack_render_data.textures.data_row)
    {
        draw_on_screen(block, *row_local_pos, row_tex, canvas);
    }

    canvas.set_draw_color(old_draw_color);
}

pub fn draw_sp_pointer(
    block: (f32, f32),
    cpu: &flynes_core::cpu::Cpu,
    stack_render_data: &mut StackRenderData,
    canvas: &mut sdl3::render::Canvas<sdl3::video::Window>,
) {
    let sp = cpu.stack_pointer();
    let sp_row = (sp as usize) / STACK_ELEMS_IN_ROW;
    let sp_el = (sp as usize) % STACK_ELEMS_IN_ROW;

    let sp_mark_local_pos = stack_render_data
        .local_pos
        .get_marker_local_pos(sp_row, sp_el);

    draw_on_screen(
        block,
        sp_mark_local_pos,
        &stack_render_data.textures.marker_bg,
        canvas,
    );
}
