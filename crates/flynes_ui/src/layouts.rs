const fn get_center(block_size: f32) -> f32 {
    block_size / 2.0
}

const fn get_split_size(block_size: f32, split_count: u8) -> f32 {
    block_size / (split_count as f32)
}

const fn get_part_center(target_part: u8, number_of_parts: u8) -> f32 {
    let number_of_parts: f32 = number_of_parts as f32;
    let target_part: f32 = target_part as f32;

    let one_part_size = 1.0 / number_of_parts;
    one_part_size * (target_part - 0.5)
}

#[derive(PartialEq)]
enum CpuLayoutParts {
    Header,
    Regs,
    Pointers,
    ExecInfo,
    CpuStatusVal,
    CpuStatusRow(u8),
}

/// Represents vertical proportions for different parts of CPU block
pub struct CpuLayout {
    header: f32,
    regs: f32,
    pointers: f32,
    exec_info: f32,
    cpu_status: f32,
}

impl CpuLayout {
    /// Creates cpu layout with vertical proportions
    ///
    /// # Example
    ///
    /// ```
    /// use crate::CpuLayout;
    /// // Creates cpu_layout with 30% of vertical space dedicated for header,
    /// // 20% for regs-pointers, and 15% each for everything else
    /// let cpu_layout = CpuLayout::new(0.3, 0.2, 0.2, 0.15, 0.15);
    /// ```
    pub const fn new(
        header: f32,
        regs: f32,
        pointers: f32,
        exec_info: f32,
        cpu_status: f32,
    ) -> CpuLayout {
        assert!(header + regs + pointers + exec_info + cpu_status >= 0.0);
        assert!(header + regs + pointers + exec_info + cpu_status <= 1.0 + 0.005);
        CpuLayout {
            header,
            regs,
            pointers,
            exec_info,
            cpu_status,
        }
    }

    const fn get_sum_ratio_center(&self, level: CpuLayoutParts) -> f32 {
        match level {
            CpuLayoutParts::Header => get_center(self.header),
            CpuLayoutParts::Regs => self.header + get_center(self.regs),
            CpuLayoutParts::Pointers => self.header + self.regs + get_center(self.pointers),
            CpuLayoutParts::ExecInfo => {
                self.header + self.regs + self.pointers + get_center(self.exec_info)
            }
            CpuLayoutParts::CpuStatusVal => {
                self.header
                    + self.regs
                    + self.pointers
                    + self.exec_info
                    + get_center(self.cpu_status)
            }
            CpuLayoutParts::CpuStatusRow(row_id) => {
                self.header
                    + self.regs
                    + self.pointers
                    + self.exec_info
                    + self.cpu_status * get_part_center(row_id, 2)
            }
        }
    }

    pub const fn get_header(&self, size_x: f32, size_y: f32) -> (f32, f32) {
        (
            get_center(size_x),
            size_y * self.get_sum_ratio_center(CpuLayoutParts::Header),
        )
    }

    pub const fn get_regs(&self, size_x: f32, size_y: f32) -> (f32, f32, f32, f32) {
        let pos_y = size_y * self.get_sum_ratio_center(CpuLayoutParts::Regs);
        let pos_x_a = size_x * get_part_center(1, 3);
        let pos_x_x = size_x * get_part_center(2, 3);
        let pos_x_y = size_x * get_part_center(3, 3);
        (pos_y, pos_x_a, pos_x_x, pos_x_y)
    }

    pub const fn get_pointers(&self, size_x: f32, size_y: f32) -> (f32, f32, f32) {
        let pos_y = size_y * self.get_sum_ratio_center(CpuLayoutParts::Pointers);
        let pos_x_pc = size_x * get_part_center(1, 2);
        let pos_x_sp = size_x * get_part_center(2, 2);
        (pos_y, pos_x_pc, pos_x_sp)
    }

    pub const fn get_exec_info(&self, size_x: f32, size_y: f32) -> (f32, f32, f32) {
        let pos_y = size_y * self.get_sum_ratio_center(CpuLayoutParts::ExecInfo);
        let pos_x_inst = size_x * get_part_center(1, 2);
        let pos_x_cycles = size_x * get_part_center(2, 2);
        (pos_y, pos_x_inst, pos_x_cycles)
    }

    pub const fn get_cpu_status(&self, size_x: f32, size_y: f32) -> [(f32, f32); 9] {
        const REGS_PER_ROW: u8 = 4;

        let split_size_x = get_split_size(size_x, 2);
        let mut all_elements: [(f32, f32); 9] = [(0.0, 0.0); 9];

        let mut i = 0;
        while i < 8 {
            let row = (i / REGS_PER_ROW) + 1;
            let col = (i % REGS_PER_ROW) + 1;

            let row_y_value = size_y * self.get_sum_ratio_center(CpuLayoutParts::CpuStatusRow(row));

            all_elements[i as usize] = (
                split_size_x * get_part_center(col, REGS_PER_ROW),
                row_y_value,
            );

            i += 1;
        }

        all_elements[8] = (
            size_x * get_part_center(2, 2),
            size_y * self.get_sum_ratio_center(CpuLayoutParts::CpuStatusVal),
        );

        all_elements
    }
}

pub struct StackLayout {
    header: f32,
    stack_data: f32,
}

#[derive(PartialEq)]
enum StackLayoutParts {
    Header,
    StackDataBlock,
    StackDataRow(u8),
}

impl StackLayout {
    pub const fn new(header: f32, stack_data: f32) -> StackLayout {
        StackLayout { header, stack_data }
    }

    const fn get_sum_ratio_center(&self, level: StackLayoutParts) -> f32 {
        match level {
            StackLayoutParts::Header => get_center(self.header),
            StackLayoutParts::StackDataBlock => self.header + get_center(self.stack_data),
            StackLayoutParts::StackDataRow(row_id) => {
                self.header + self.stack_data * get_part_center(row_id, 16)
            }
        }
    }

    pub const fn get_header(&self, size_x: f32, size_y: f32) -> (f32, f32) {
        (
            get_center(size_x),
            size_y * self.get_sum_ratio_center(StackLayoutParts::Header),
        )
    }

    pub const fn get_stack_first_col_block(
        &self,
        size_x: f32,
        size_y: f32,
    ) -> ((f32, f32), (f32, f32)) {
        let block_center_pos_x = size_x * get_part_center(1, 21);
        let block_center_pos_y =
            size_y * self.get_sum_ratio_center(StackLayoutParts::StackDataBlock);

        let block_size_x = get_split_size(size_x, 21); // 21 = 16(4*4) + 4(spaces) + 1(header)
        let block_size_y = size_y * self.stack_data;

        (
            (block_center_pos_x, block_center_pos_y),
            (block_size_x, block_size_y),
        )
    }

    pub const fn get_stack_first_col_elems(&self, size_x: f32, size_y: f32) -> [(f32, f32); 16] {
        let block_size = self.get_stack_first_col_block(size_x, size_y).1;
        let block_size_x = block_size.0;
        let block_size_y = block_size.1;

        let mut st_headers_pos: [(f32, f32); 16] = [(0.0, 0.0); 16];
        let st_headers_x = get_center(block_size_x);

        let mut row_id = 0;
        while row_id < 16 {
            st_headers_pos[row_id as usize] =
                (st_headers_x, block_size_y * get_part_center(row_id + 1, 16));
            row_id += 1
        }

        st_headers_pos
    }

    pub const fn get_stack_rows_block(
        &self,
        size_x: f32,
        size_y: f32,
    ) -> ((f32, f32), [(f32, f32); 16]) {
        let mut st_row_pos: [(f32, f32); 16] = [(0.0, 0.0); 16];

        let col_size = get_split_size(size_x, 21);
        let st_row_size_x = col_size * (21.0 - 2.0);
        let st_row_size_y = get_split_size(size_y * self.stack_data, 16);
        let row_block_center_x = (col_size * 2.0) + get_center(st_row_size_x);

        let mut row_id = 0;
        while row_id < 16 {
            st_row_pos[row_id as usize] = (
                row_block_center_x,
                size_y * self.get_sum_ratio_center(StackLayoutParts::StackDataRow(row_id + 1)),
            );
            row_id += 1;
        }

        ((st_row_size_x, st_row_size_y), st_row_pos)
    }

    pub const fn get_stack_rows_elems(
        &self,
        block_size_x: f32,
        block_size_y: f32,
    ) -> ((f32, f32), [(f32, f32); 16]) {
        let mut st_values_pos: [(f32, f32); 16] = [(0.0, 0.0); 16];

        let elem_size_x = get_split_size(block_size_x, 19);
        let elem_size_y = block_size_y;

        let mut col_id = 0;
        let mut true_col_id = 0;

        while col_id < 19 {
            if col_id % 5 == 4 {
                col_id += 1;
                continue;
            }

            st_values_pos[true_col_id] = (
                block_size_x * get_part_center(col_id + 1, 19),
                get_center(elem_size_y),
            );

            true_col_id += 1;
            col_id += 1;
        }

        ((elem_size_x, elem_size_y), st_values_pos)
    }
}
