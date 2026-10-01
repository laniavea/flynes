pub enum ViewSubjects {
    Cpu,
    Stack,
}

pub struct ViewState {
    cpu_view_status: bool,
    stack_view_status: bool,
}

impl std::default::Default for ViewState {
    fn default() -> Self {
        ViewState {
            cpu_view_status: true,
            stack_view_status: true,
        }
    }
}

impl ViewState {
    pub fn get_state(&self, view_subject: ViewSubjects) -> bool {
        match view_subject {
            ViewSubjects::Cpu => self.cpu_view_status,
            ViewSubjects::Stack => self.stack_view_status,
        }
    }

    pub fn invert_state(&mut self, view_subject: ViewSubjects) {
        match view_subject {
            ViewSubjects::Cpu => self.cpu_view_status = !self.cpu_view_status,
            ViewSubjects::Stack => self.stack_view_status = !self.stack_view_status,
        }
    }

    pub fn update_by_keypress(&mut self, key_code: sdl3::keyboard::Keycode) {
        match key_code {
            sdl3::keyboard::Keycode::_1 => {
                self.invert_state(ViewSubjects::Cpu);
            },
            sdl3::keyboard::Keycode::_2 => {
                self.invert_state(ViewSubjects::Stack);
            }
            _ => ()
        }
    }
}
