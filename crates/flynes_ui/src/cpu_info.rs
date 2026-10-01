use crate::layouts::CpuLayout;
use crate::shared::{create_tex, draw_bg, draw_on_screen, generate_bg_tex, create_empty_tex};

const TEXT_COLOR: sdl3::pixels::Color = sdl3::pixels::Color::RGB(255, 255, 255);
const BG_COLOR: sdl3::pixels::Color = sdl3::pixels::Color::RGB(40, 40, 40);
const CPU_LAYOUT: CpuLayout = CpuLayout::new(0.3, 0.13, 0.13, 0.14, 0.3);

const FLAG_TEXT: [(&str, &str); 8] = [
    ("C:0", "C:V"),
    ("Z:0", "Z:V"),
    ("I:0", "I:V"),
    ("D:0", "D:V"),
    ("B:0", "B:V"),
    ("U:0", "U:V"),
    ("O:0", "O:V"),
    ("N:0", "N:V"),
];

pub struct CachedCpuState {
    reg_a: u8,
    reg_x: u8,
    reg_y: u8,
    pc: u16,
    sp: u8,
    cpu_status: u8,
    exec_cycles: usize,
    exec_instructions: usize,
}

impl CachedCpuState {
    fn new(cpu: &flynes_core::cpu::Cpu) -> CachedCpuState {
        let regs_values = cpu.get_registers_state();
        let pc_value = cpu.program_counter();
        let sp_value = cpu.stack_pointer();
        let cpu_status_value = cpu.cpu_status();
        let exec_cycles = cpu.exec_cycles();
        let exec_instructions = cpu.exec_instructions();

        CachedCpuState {
            reg_a: regs_values[0],
            reg_x: regs_values[1],
            reg_y: regs_values[2],
            pc: pc_value,
            sp: sp_value,
            cpu_status: cpu_status_value,
            exec_cycles,
            exec_instructions,
        }
    }
}

pub struct CpuLocalPos {
    header: (f32, f32),
    regs: [(f32, f32); 3],
    pc: (f32, f32),
    sp: (f32, f32),
    exec_cycles: (f32, f32),
    exec_instructions: (f32, f32),
    cpu_status: [(f32, f32); 9],
}

impl CpuLocalPos {
    fn new(size_x: f32, size_y: f32) -> CpuLocalPos {
        let header_local_pos = CPU_LAYOUT.get_header(size_x, size_y);

        let pos_regs = CPU_LAYOUT.get_regs(size_x, size_y);
        let regs_local_pos: [(f32, f32); 3] = [
            (pos_regs.1, pos_regs.0),
            (pos_regs.2, pos_regs.0),
            (pos_regs.3, pos_regs.0),
        ];

        let pos_pointers = CPU_LAYOUT.get_pointers(size_x, size_y);
        let pointers_local_pos: [(f32, f32); 2] = [
            (pos_pointers.1, pos_pointers.0),
            (pos_pointers.2, pos_pointers.0),
        ];

        let pos_exec_info = CPU_LAYOUT.get_exec_info(size_x, size_y);
        let exec_info_local_pos: [(f32, f32); 2] = [
            (pos_exec_info.1, pos_exec_info.0),
            (pos_exec_info.2, pos_exec_info.0),
        ];

        let cpu_status_pos = CPU_LAYOUT.get_cpu_status(size_x, size_y);

        CpuLocalPos {
            header: header_local_pos,
            regs: regs_local_pos,
            pc: pointers_local_pos[0],
            sp: pointers_local_pos[1],
            exec_cycles: exec_info_local_pos[0],
            exec_instructions: exec_info_local_pos[1],
            cpu_status: cpu_status_pos,
        }
    }
}

pub struct CpuTextures<'tex> {
    bg: sdl3::render::Texture<'tex>,
    header: sdl3::render::Texture<'tex>,
    regs: [sdl3::render::Texture<'tex>; 3],
    pc: sdl3::render::Texture<'tex>,
    sp: sdl3::render::Texture<'tex>,
    exec_cycles: sdl3::render::Texture<'tex>,
    exec_inst: sdl3::render::Texture<'tex>,
    cpu_status: [sdl3::render::Texture<'tex>; 9],
}

impl<'tex> CpuTextures<'tex> {
    fn init(
        texture_creator: &'tex sdl3::render::TextureCreator<sdl3::video::WindowContext>,
    ) -> CpuTextures<'tex> {
        CpuTextures {
            bg: create_empty_tex(texture_creator),
            header: create_empty_tex(texture_creator),
            regs: std::array::from_fn(|_| create_empty_tex(texture_creator)),
            pc: create_empty_tex(texture_creator),
            sp: create_empty_tex(texture_creator),
            exec_cycles: create_empty_tex(texture_creator),
            exec_inst: create_empty_tex(texture_creator),
            cpu_status: std::array::from_fn(|_| create_empty_tex(texture_creator)),
        }
    }

    pub fn new(
        sizes: (f32, f32),
        cpu_state: &mut CachedCpuState,
        cpu: &flynes_core::cpu::Cpu,
        header_font: &sdl3::ttf::Font,
        p_font: &sdl3::ttf::Font,
        texture_creator: &'tex sdl3::render::TextureCreator<sdl3::video::WindowContext>,
    ) -> CpuTextures<'tex> {
        let mut textures = CpuTextures::init(texture_creator);

        update_static(sizes, &mut textures, header_font, texture_creator);
        update_regs(&mut textures, cpu_state, cpu, p_font, texture_creator, true);
        update_pointers(&mut textures, cpu_state, cpu, p_font, texture_creator, true);
        update_exec_info(&mut textures, cpu_state, cpu, p_font, texture_creator, true);
        update_cpu_status(&mut textures, cpu_state, cpu, p_font, texture_creator, true);

        textures
    }
}

pub struct CpuRenderData<'tex> {
    size: (f32, f32),
    cpu_state: CachedCpuState,
    local_pos: CpuLocalPos,
    textures: CpuTextures<'tex>,
}

impl<'tex> CpuRenderData<'tex> {
    pub fn new(
        size_x: f32,
        size_y: f32,
        texture_creator: &'tex sdl3::render::TextureCreator<sdl3::video::WindowContext>,
        header_font: &sdl3::ttf::Font,
        p_font: &sdl3::ttf::Font,
        cpu: &flynes_core::cpu::Cpu,
    ) -> CpuRenderData<'tex> {
        let mut cpu_state = CachedCpuState::new(cpu);
        let local_pos = CpuLocalPos::new(size_x, size_y);

        let sizes = (size_x, size_y);
        let cpu_textures = CpuTextures::new(
            sizes,
            &mut cpu_state,
            cpu,
            header_font,
            p_font,
            texture_creator,
        );

        CpuRenderData {
            size: (size_x, size_y),
            cpu_state,
            textures: cpu_textures,
            local_pos,
        }
    }
}

fn update_static<'tex>(
    sizes: (f32, f32),
    textures: &mut CpuTextures<'tex>,
    header_font: &sdl3::ttf::Font,
    texture_creator: &'tex sdl3::render::TextureCreator<sdl3::video::WindowContext>,
) {
    let sizes_u32 = (sizes.0 as u32, sizes.1 as u32);
    let bg_texture = generate_bg_tex(sizes_u32, BG_COLOR, texture_creator);
    let header_texture = create_tex("CPU[1]", &TEXT_COLOR, texture_creator, header_font);

    textures.bg = bg_texture;
    textures.header = header_texture;
}

fn update_regs<'tex>(
    textures: &mut CpuTextures<'tex>,
    cpu_state: &mut CachedCpuState,
    cpu: &flynes_core::cpu::Cpu,
    font: &sdl3::ttf::Font,
    texture_creator: &'tex sdl3::render::TextureCreator<sdl3::video::WindowContext>,
    force_rerender: bool,
) {
    let regs = cpu.get_registers_state();
    let (reg_a, reg_x, reg_y) = (regs[0], regs[1], regs[2]);

    let mut reg_a_state = cpu_state.reg_a != reg_a;
    let mut reg_x_state = cpu_state.reg_x != reg_x;
    let mut reg_y_state = cpu_state.reg_y != reg_y;

    if force_rerender {
        (reg_a_state, reg_x_state, reg_y_state) = (true, true, true);
    }

    if reg_a_state {
        let reg_a_text = format!("REG A: {} - {:X}", reg_a, reg_a);
        let reg_tex = create_tex(&reg_a_text, &TEXT_COLOR, texture_creator, font);
        cpu_state.reg_a = reg_a;
        textures.regs[0] = reg_tex;
    }

    if reg_x_state {
        let reg_x_text = format!("REG X: {} - {:X}", reg_x, reg_x);
        let reg_tex = create_tex(&reg_x_text, &TEXT_COLOR, texture_creator, font);
        cpu_state.reg_x = reg_x;
        textures.regs[1] = reg_tex;
    }

    if reg_y_state {
        let reg_y_text = format!("REG Y: {} - {:X}", reg_y, reg_y);
        let reg_tex = create_tex(&reg_y_text, &TEXT_COLOR, texture_creator, font);
        cpu_state.reg_y = reg_y;
        textures.regs[2] = reg_tex;
    }
}

fn update_pointers<'tex>(
    textures: &mut CpuTextures<'tex>,
    cpu_state: &mut CachedCpuState,
    cpu: &flynes_core::cpu::Cpu,
    font: &sdl3::ttf::Font,
    texture_creator: &'tex sdl3::render::TextureCreator<sdl3::video::WindowContext>,
    force_rerender: bool,
) {
    let pc = cpu.program_counter();
    let sp = cpu.stack_pointer();

    let mut pc_status = cpu_state.pc != pc;
    let mut sp_status = cpu_state.sp != sp;

    if force_rerender {
        (pc_status, sp_status) = (true, true);
    }

    if pc_status {
        let pc_text = format!("PC: {} - {:X}", pc, pc);
        let pc_tex = create_tex(&pc_text, &TEXT_COLOR, texture_creator, font);
        textures.pc = pc_tex;
        cpu_state.pc = pc;
    }

    if sp_status {
        let sp_text = format!("SP: {} - {:X}", sp, sp);
        let sp_tex = create_tex(&sp_text, &TEXT_COLOR, texture_creator, font);
        textures.sp = sp_tex;
        cpu_state.sp = sp;
    }
}

fn update_exec_info<'tex>(
    textures: &mut CpuTextures<'tex>,
    cpu_state: &mut CachedCpuState,
    cpu: &flynes_core::cpu::Cpu,
    font: &sdl3::ttf::Font,
    texture_creator: &'tex sdl3::render::TextureCreator<sdl3::video::WindowContext>,
    force_rerender: bool,
) {
    let exec_cycles = cpu.exec_cycles();
    let exec_instructions = cpu.exec_instructions();

    let mut exec_cycles_status = cpu_state.exec_cycles != exec_cycles;
    let mut exec_inst_status = cpu_state.exec_instructions != exec_instructions;

    if force_rerender {
        (exec_cycles_status, exec_inst_status) = (true, true);
    }

    if exec_cycles_status {
        let cyc_text = format!("CYC: {}", exec_cycles);
        let cyc_tex = create_tex(&cyc_text, &TEXT_COLOR, texture_creator, font);
        cpu_state.exec_cycles = exec_cycles;
        textures.exec_cycles = cyc_tex;
    }

    if exec_inst_status {
        let inst_text = format!("INST: {}", exec_instructions);
        let inst_tex = create_tex(&inst_text, &TEXT_COLOR, texture_creator, font);
        cpu_state.exec_instructions = exec_instructions;
        textures.exec_inst = inst_tex;
    }
}

fn update_cpu_status<'tex>(
    textures: &mut CpuTextures<'tex>,
    cpu_state: &mut CachedCpuState,
    cpu: &flynes_core::cpu::Cpu,
    font: &sdl3::ttf::Font,
    texture_creator: &'tex sdl3::render::TextureCreator<sdl3::video::WindowContext>,
    force_rerender: bool,
) {
    let mut cpu_status_textures = [const { None }; 9];
    let new_cpu_status = cpu.cpu_status();
    let old_cpu_status = cpu_state.cpu_status;

    if (new_cpu_status == old_cpu_status) && !force_rerender {
        return;
    }

    let mut mask = 0b0000_0001u8;
    let mut elem_id: usize = 0;
    while mask != 0 {
        if (new_cpu_status & mask != old_cpu_status & mask) || force_rerender {
            let elem_text = if new_cpu_status & mask == mask {
                FLAG_TEXT[elem_id].1
            } else {
                FLAG_TEXT[elem_id].0
            };

            cpu_status_textures[elem_id] =
                Some(create_tex(elem_text, &TEXT_COLOR, texture_creator, font));
        }
        elem_id += 1;
        mask <<= 1;
    }

    let cpu_status_text = format!("CPU status: {} - {:X}", new_cpu_status, new_cpu_status);
    cpu_status_textures[8] = Some(create_tex(
        &cpu_status_text,
        &TEXT_COLOR,
        texture_creator,
        font,
    ));
    cpu_state.cpu_status = new_cpu_status;

    for (cpu_status_id, cpu_status_tex) in cpu_status_textures.into_iter().enumerate() {
        if let Some(new_cpu_status_tex) = cpu_status_tex {
            textures.cpu_status[cpu_status_id] = new_cpu_status_tex 
        }
    }
}

pub fn update_textures<'tex>(
    render_data: &mut CpuRenderData<'tex>,
    cpu: &flynes_core::cpu::Cpu,
    p_font: &sdl3::ttf::Font,
    texture_creator: &'tex sdl3::render::TextureCreator<sdl3::video::WindowContext>,
) {
    let cpu_state = &mut render_data.cpu_state;
    let cpu_textures = &mut render_data.textures;

    update_regs(cpu_textures, cpu_state, cpu, p_font, texture_creator, false);
    update_pointers(cpu_textures, cpu_state, cpu, p_font, texture_creator, false);
    update_exec_info(cpu_textures, cpu_state, cpu, p_font, texture_creator, false);
    update_cpu_status(cpu_textures, cpu_state, cpu, p_font, texture_creator, false);
}

pub fn draw_block<'tex>(
    block: (f32, f32),
    render_data: &CpuRenderData<'tex>,
    canvas: &mut sdl3::render::Canvas<sdl3::video::Window>,
) {
    let cpu_textures = &render_data.textures;
    draw_bg(block, render_data.size, &cpu_textures.bg, canvas);

    let header_local_pos = render_data.local_pos.header;
    draw_on_screen(block, header_local_pos, &cpu_textures.header, canvas);

    assert!(render_data.local_pos.regs.len() == cpu_textures.regs.len());
    for (reg_local_pos, reg_tex) in render_data.local_pos.regs.iter().zip(&cpu_textures.regs) {
        draw_on_screen(block, *reg_local_pos, reg_tex, canvas);
    }

    let pc_local_pos = render_data.local_pos.pc;
    draw_on_screen(block, pc_local_pos, &cpu_textures.pc, canvas);

    let sp_local_pos = render_data.local_pos.sp;
    draw_on_screen(block, sp_local_pos, &cpu_textures.sp, canvas);

    let cyc_local_pos = render_data.local_pos.exec_cycles;
    draw_on_screen(block, cyc_local_pos, &cpu_textures.exec_cycles, canvas);

    let inst_local_pos = render_data.local_pos.exec_instructions;
    draw_on_screen(block, inst_local_pos, &cpu_textures.exec_inst, canvas);

    assert!(render_data.local_pos.cpu_status.len() == cpu_textures.cpu_status.len());
    for (status_local_pos, status_tex) in render_data
        .local_pos
        .cpu_status
        .iter()
        .zip(&cpu_textures.cpu_status)
    {
        draw_on_screen(block, *status_local_pos, status_tex, canvas);
    }
}
