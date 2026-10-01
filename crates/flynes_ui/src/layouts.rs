pub struct CpuLayout {
    header: f32,
    regs: f32,
    pointers: f32,
    exec_info: f32,
    cpu_status: f32,
}

impl CpuLayout {
    pub const fn new(
        header: f32,
        regs: f32,
        pointers: f32,
        exec_info: f32,
        cpu_status: f32,
    ) -> CpuLayout {
        CpuLayout {
            header,
            regs,
            pointers,
            exec_info,
            cpu_status,
        }
    }

    pub const fn get_header(&self, size_x: f32, size_y: f32) -> (f32, f32) {
        (size_x / 2.0, (size_y * self.header) / 2.0)
    }

    pub const fn get_regs(&self, size_x: f32, size_y: f32) -> (f32, f32, f32, f32) {
        let pos_y = size_y * (self.header + (self.regs / 2.0));
        let pos_x_a = size_x / 3.0 / 2.0;
        let pos_x_x = size_x / 3.0 * 1.5;
        let pos_x_y = size_x / 3.0 * 2.5;
        (pos_y, pos_x_a, pos_x_x, pos_x_y)
    }

    pub const fn get_pointers(&self, size_x: f32, size_y: f32) -> (f32, f32, f32) {
        let pos_y = size_y * (self.header + self.regs + (self.pointers / 2.0));
        let pos_x_pc = size_x / 2.0 / 2.0;
        let pos_x_sp = size_x / 2.0 * 1.5;
        (pos_y, pos_x_pc, pos_x_sp)
    }

    pub const fn get_exec_info(&self, size_x: f32, size_y: f32) -> (f32, f32, f32) {
        let pos_y = size_y * (self.header + self.regs + self.pointers + (self.exec_info / 2.0));
        let pos_inst = size_x / 2.0 / 2.0;
        let pos_cycles = size_x / 2.0 * 1.5;
        (pos_y, pos_inst, pos_cycles)
    }

    pub const fn get_cpu_status(&self, size_x: f32, size_y: f32) -> [(f32, f32); 9] {
        let pr_blocks = self.header + self.regs + self.pointers + self.exec_info;
        let first_row_y = size_y * (pr_blocks + (self.cpu_status / 4.0));
        let second_row_y = size_y * (pr_blocks + (self.cpu_status / 4.0 * 3.0));

        let step_elem_x = (size_x / 2.0) / 4.0;
        let first_elem_x = step_elem_x / 2.0;

        let mut all_elements: [(f32, f32); 9] = [(0.0, 0.0); 9];

        let mut i = 0;
        while i < 8 {
            if i < 4 {
                all_elements[i] = (first_elem_x + (step_elem_x * (i as f32)), first_row_y);
            } else {
                all_elements[i] = (
                    first_elem_x + (step_elem_x * ((i - 4) as f32)),
                    second_row_y,
                );
            }
            i += 1;
        }

        all_elements[8] = (
            (size_x / 2.0) * 1.5,
            size_y * (pr_blocks + (self.cpu_status / 2.0)),
        );

        all_elements
    }
}

pub struct StackLayout {
    header: f32,
    stack_data: f32,
}

impl StackLayout {
    pub const fn new(header: f32, stack_data: f32) -> StackLayout {
        StackLayout { header, stack_data }
    }

    pub const fn get_header(&self, size_x: f32, size_y: f32) -> (f32, f32) {
        (size_x / 2.0, (size_y * self.header) / 2.0)
    }

    pub const fn get_stack_first_col_block(
        &self,
        size_x: f32,
        size_y: f32,
    ) -> ((f32, f32), (f32, f32)) {
        let block_start_x = (size_x / 21.0) / 2.0;
        let block_start_y = size_y * (self.header + (self.stack_data / 2.0));

        let block_size_x = size_x / 21.0; // 21 = 16(4*4) + 4(spaces) + 1(header)
        let block_size_y = size_y * self.stack_data;

        ((block_start_x, block_start_y), (block_size_x, block_size_y))
    }

    pub const fn get_stack_first_col_elems(&self, size_x: f32, size_y: f32) -> [(f32, f32); 16] {
        let block_size = self.get_stack_first_col_block(size_x, size_y).1;
        let block_size_x = block_size.0;
        let block_size_y = block_size.1;

        let mut st_headers_pos: [(f32, f32); 16] = [(0.0, 0.0); 16];
        let st_headers_y_step = block_size_y / 16.0;
        let st_headers_x = block_size_x / 2.0;
        let mut row_id = 0;
        while row_id < 16 {
            st_headers_pos[row_id] = (
                st_headers_x,
                ((block_size_y / 16.0) / 2.0) + (st_headers_y_step * (row_id as f32)),
            );
            row_id += 1
        }

        st_headers_pos
    }

    pub const fn get_stack_rows_block(&self, size_x: f32, size_y: f32) -> ((f32, f32), [(f32, f32); 16]) {
        let mut st_row_pos: [(f32, f32); 16] = [(0.0, 0.0); 16];

        let st_row_size_x = size_x - ((size_x / 21.0) * 2.0);
        let st_row_size_y = (size_y * self.stack_data) / 16.0;

        let mut row_id = 0;
        while row_id < 16 {
            st_row_pos[row_id] = (
                ((size_x / 21.0) * 2.0) + st_row_size_x / 2.0,
                size_y * self.header + ((st_row_size_y / 2.0) * ((row_id * 2 + 1) as f32)),
            );
            row_id += 1;
        }

        ((st_row_size_x, st_row_size_y), st_row_pos)
    }

    pub const fn get_stack_rows_elems(&self, size_x: f32, size_y: f32) -> ((f32, f32), [(f32, f32); 16]) {
        let mut st_values_pos: [(f32, f32); 16] = [(0.0, 0.0); 16];

        let mut col_id = 0;
        let mut true_col_id = 0;

        let elem_size_x = size_x / 19.0;
        let elem_size_y = size_y;

        while col_id < 19 {
            if col_id % 5 == 4 {
                col_id += 1;
                continue;
            }

            st_values_pos[true_col_id] = (
                ((size_x / 19.0) / 2.0) * ((col_id * 2 + 1) as f32),
                size_y / 2.0,
            );

            true_col_id += 1;
            col_id += 1;
        }

        ((elem_size_x, elem_size_y), st_values_pos)
    }
}
