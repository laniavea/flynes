extern crate sdl3;

mod cpu_info;
mod disassembly_info;
pub mod layouts;
mod memory_info;
mod shared;
mod stack_info;
mod view_state;

const DIGITS_COLOR: sdl3::pixels::Color = sdl3::pixels::Color::RGB(255, 255, 255);

const NES_PIXEL_MULT: u32 = 4;
const NES_SCREEN_WIDTH: u32 = 256;
const NES_SCREEN_HEIGHT: u32 = 240;
const NES_SCREEN_SCALED_W: u32 = NES_SCREEN_WIDTH * NES_PIXEL_MULT;
const NES_SCREEN_SCALED_H: u32 = NES_SCREEN_HEIGHT * NES_PIXEL_MULT;

const TEST_UPDATE_TEX: bool = true;

fn main() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("flynes-debug-sdl", 2560, 1440)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();

    canvas.set_draw_color(sdl3::pixels::Color::RGB(60, 60, 60));
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();

    let nes_screen = vec![20u8; (NES_SCREEN_SCALED_H * NES_SCREEN_SCALED_W * 3) as usize];
    let mut nes_screen_texture = texture_creator
        .create_texture_target(
            sdl3::pixels::PixelFormat::RGB24,
            NES_SCREEN_SCALED_W,
            NES_SCREEN_SCALED_H,
        )
        .unwrap();

    nes_screen_texture
        .update(None, &nes_screen, (NES_SCREEN_SCALED_W * 3) as usize)
        .unwrap();

    let nes_screen_target = Some(sdl3::render::FRect::new(
        0.0,
        0.0,
        NES_SCREEN_SCALED_W as f32,
        NES_SCREEN_SCALED_H as f32,
    ));

    let (mut cpu, mut bus) =
        match flynes_core::cartridges::read_nes_file("../../roms/nestest.nes".into()) {
            Ok(modules) => modules,
            Err(err) => {
                println!("Error occured, see log");
                println!("Error: {err}");
                return;
            }
        };
    cpu.set_pc(0xC000);

    let stack_data = bus.memory_mut().stack_as_slice_mut();
    for (stack_value_c, stack_value) in (0_u16..).zip(stack_data.iter_mut()) {
        *stack_value = stack_value_c
            .try_into()
            .expect("Stack slice have more than 256 values");
    }

    let sdl_ttf = sdl3::ttf::init().unwrap();
    let header_font = sdl_ttf
        .load_font("/usr/share/fonts/TTF/OpenSans-Regular.ttf", 30.0)
        .unwrap();

    let p_font = sdl_ttf
        .load_font("/usr/share/fonts/TTF/OpenSans-Regular.ttf", 20.0)
        .unwrap();

    let digits_textures: Vec<sdl3::render::Texture> = (0..256)
        .map(|digit| {
            let digit: u8 = digit.try_into().unwrap();
            let text_to_render = flynes_core::common::number_to_hex(digit, false);
            let text_surface = p_font
                .render(&text_to_render)
                .blended(DIGITS_COLOR)
                .unwrap();
            texture_creator
                .create_texture_from_surface(text_surface)
                .unwrap()
        })
        .collect();

    let mut cpu_render_data =
        cpu_info::CpuRenderData::new(700.0, 400.0, &texture_creator, &header_font, &p_font, &cpu);

    let mut stack_render_data = stack_info::StackRenderData::new(
        (1000.0, 600.0),
        &bus,
        &texture_creator,
        &mut canvas,
        &digits_textures,
        &header_font,
    );

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut frame_num: usize = 0;

    let mut view_state = crate::view_state::ViewState::default();

    'running: loop {
        use std::time::Instant;
        let t = Instant::now();
        canvas.clear();

        canvas
            .copy(&nes_screen_texture, None, nes_screen_target)
            .unwrap();

        for event in event_pump.poll_iter() {
            match event {
                sdl3::event::Event::Quit { .. }
                | sdl3::event::Event::KeyDown {
                    keycode: Some(sdl3::keyboard::Keycode::Escape),
                    ..
                } => break 'running,
                sdl3::event::Event::KeyDown {
                    keycode: Some(key_code),
                    ..
                } => {
                    view_state.update_by_keypress(key_code);
                }
                _ => {}
            }
        }

        if view_state.get_state(view_state::ViewSubjects::Cpu) {
            cpu_info::update_textures(&mut cpu_render_data, &cpu, &p_font, &texture_creator);
            cpu_info::draw_block((1050.0, 0.0), &cpu_render_data, &mut canvas);
        }

        if view_state.get_state(view_state::ViewSubjects::Stack) {
            stack_info::update_textures(
                &mut stack_render_data,
                &bus,
                &texture_creator,
                &mut canvas,
                false,
            );
            stack_info::draw_stack_info((1050.0, 400.0), &mut stack_render_data, &mut canvas);
            stack_info::draw_sp_pointer((1050.0, 400.0), &cpu, &mut stack_render_data, &mut canvas);
        }

        canvas.present();
        let tt = t.elapsed();
        println!("{tt:?}");

        frame_num = frame_num.wrapping_add(1);

        if TEST_UPDATE_TEX {
            update_all_cpu_data(&mut cpu, frame_num);
            update_all_stack_data(&mut bus);
        } else {
            let mut exec_status: bool = false;
            for _ in 0..22000 {
                match cpu.execute_operation(&mut bus, false) {
                    Ok(_) => (),
                    Err(err) => exec_status = true,
                }
            }

            if exec_status {
                println!("Error was encountered")
            }
        }

        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn update_all_cpu_data(cpu: &mut flynes_core::cpu::Cpu, frame_num: usize) {
    const FRAMES_PER_REG_A_UPDATE: usize = 20;
    const FRAMES_PER_REG_X_UPDATE: usize = 10;
    const FRAMES_PER_REG_Y_UPDATE: usize = 3;

    if frame_num.is_multiple_of(FRAMES_PER_REG_A_UPDATE) {
        cpu.set_regs(Some((frame_num % 256) as u8), None, None);
    }

    if frame_num.is_multiple_of(FRAMES_PER_REG_X_UPDATE) {
        cpu.set_regs(None, Some((frame_num % 256) as u8), None);
    }

    if frame_num.is_multiple_of(FRAMES_PER_REG_Y_UPDATE) {
        cpu.set_regs(None, None, Some((frame_num % 256) as u8));
    }

    cpu.set_pc((frame_num % (u16::MAX as usize)) as u16);
    cpu.set_stack_pointer((frame_num % 256) as u8);

    cpu.set_exec_cycles(frame_num);
    if frame_num.is_multiple_of(2) {
        cpu.set_exec_instructions(frame_num / 2);
    }

    cpu.set_cpu_status((frame_num % 256) as u8);
}

fn update_all_stack_data(bus: &mut flynes_core::bus::Bus) {
    let stack_data = bus.memory_mut().stack_as_slice_mut();
    for stack_value in stack_data {
        *stack_value = stack_value.wrapping_add(1);
    }
}
