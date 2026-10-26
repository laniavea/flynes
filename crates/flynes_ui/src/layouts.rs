const STACK_ROWS: u8 = 16; // 16 rows, each containing 16 elements
pub const STACK_ROWS_USIZE: usize = STACK_ROWS as usize;
const STACK_COLS: u8 = 16; // 16 stack values per row
pub const STACK_COLS_USIZE: usize = STACK_COLS as usize; // 16 stack values per row
const STACK_COLS_GROUP_SIZE: u8 = 4;

// 21 = 16 data columns + header(+space) + 3 spacing columns
const STACK_DRAW_COLS: u8 = STACK_COLS + 2 + ((STACK_COLS - 1) / STACK_COLS_GROUP_SIZE);
const STACK_COLS_HEADERLESS: u8 = STACK_DRAW_COLS - 2; // 19 = 21 drawing columns - 2 header columns

/// Calculates the center based on block's full size
const fn get_center(block_size: f32) -> f32 {
    block_size / 2.0
}

/// Calculates the size of single part, assuming all parts have equal length
const fn get_split_size(block_size: f32, split_count: u8) -> f32 {
    assert!(split_count != 0);
    block_size / (split_count as f32)
}

/// Calculates the relative position of the center of the target_part among the specified number of
/// parts, assuming all parts have equal lengths.
const fn get_part_center(target_part: u8, number_of_parts: u8) -> f32 {
    assert!(target_part != 0);
    assert!(target_part <= number_of_parts);
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
    /// The CPU status value itself
    CpuStatusVal,
    /// CPU status represented as rows of flags; row_num must be 1 or 2
    CpuStatusRow(u8),
}

/// Represents the vertical proportions of the different parts of the CPU block
pub struct CpuLayout {
    header: f32,
    regs: f32,
    pointers: f32,
    exec_info: f32,
    cpu_status: f32,
}

impl CpuLayout {
    /// Creates a CPU layout with specified vertical proportions
    pub const fn new(
        header: f32,
        regs: f32,
        pointers: f32,
        exec_info: f32,
        cpu_status: f32,
    ) -> CpuLayout {
        assert!(header >= 0.0);
        assert!(regs >= 0.0);
        assert!(pointers >= 0.0);
        assert!(exec_info >= 0.0);
        assert!(cpu_status >= 0.0);
        assert!(header + regs + pointers + exec_info + cpu_status <= 1.0 + 0.005);
        CpuLayout {
            header,
            regs,
            pointers,
            exec_info,
            cpu_status,
        }
    }

    /// Returns the normalized vertical position of the specified CPU layout element
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

/// Represents the vertical proportions of the different sections of the stack block
pub struct StackLayout {
    header: f32,
    stack_data: f32,
}

#[derive(PartialEq)]
enum StackLayoutParts {
    Header,
    /// Represent the layout of the entire stack data block
    StackDataBlock,
    /// Represent the layout of each row of stack data
    StackDataRow(u8),
}

impl StackLayout {
    /// Creates a stack layout with specified vertical proportions
    pub const fn new(header: f32, stack_data: f32) -> StackLayout {
        assert!(header >= 0.0);
        assert!(stack_data >= 0.0);
        assert!(header + stack_data <= 1.0 + 0.005);
        StackLayout { header, stack_data }
    }

    /// Returns the normalized vertical position of the center of the specified stack layout element
    const fn get_sum_ratio_center(&self, level: StackLayoutParts) -> f32 {
        match level {
            StackLayoutParts::Header => get_center(self.header),
            StackLayoutParts::StackDataBlock => self.header + get_center(self.stack_data),
            StackLayoutParts::StackDataRow(row_id) => {
                self.header + self.stack_data * get_part_center(row_id, STACK_ROWS)
            }
        }
    }

    /// Returns the center of the stack header
    pub const fn get_header(&self, size_x: f32, size_y: f32) -> (f32, f32) {
        (
            get_center(size_x),
            size_y * self.get_sum_ratio_center(StackLayoutParts::Header),
        )
    }

    /// Returns the center position and size of the first column in the stack
    pub const fn get_stack_header_col_block(
        &self,
        size_x: f32,
        size_y: f32,
    ) -> ((f32, f32), (f32, f32)) {
        let block_center_pos_x = size_x * get_part_center(1, STACK_DRAW_COLS);
        let block_center_pos_y =
            size_y * self.get_sum_ratio_center(StackLayoutParts::StackDataBlock);

        let block_size_x = get_split_size(size_x, STACK_DRAW_COLS);
        let block_size_y = size_y * self.stack_data;

        (
            (block_center_pos_x, block_center_pos_y),
            (block_size_x, block_size_y),
        )
    }

    /// Returns the center positions of the elements in the first column
    /// Positions are relative to the first column block
    pub const fn get_stack_first_col_elems(
        &self,
        size_x: f32,
        size_y: f32,
    ) -> [(f32, f32); STACK_ROWS_USIZE] {
        let block_size = self.get_stack_header_col_block(size_x, size_y).1;
        let block_size_x = block_size.0;
        let block_size_y = block_size.1;

        let mut st_headers_pos: [(f32, f32); STACK_ROWS_USIZE] = [(0.0, 0.0); STACK_ROWS_USIZE];
        let st_headers_x = get_center(block_size_x);

        let mut row_id = 0;
        while row_id < STACK_ROWS {
            st_headers_pos[row_id as usize] = (
                st_headers_x,
                block_size_y * get_part_center(row_id + 1, STACK_ROWS),
            );
            row_id += 1
        }

        st_headers_pos
    }

    /// Returns the size and center positions of the stack rows, excluding the first column and its spacing
    pub const fn get_stack_rows_blocks(
        &self,
        size_x: f32,
        size_y: f32,
    ) -> ((f32, f32), [(f32, f32); STACK_ROWS_USIZE]) {
        let mut st_row_pos: [(f32, f32); STACK_ROWS_USIZE] = [(0.0, 0.0); STACK_ROWS_USIZE];

        let col_size = get_split_size(size_x, STACK_DRAW_COLS);
        let st_row_size_x = col_size * (STACK_COLS_HEADERLESS as f32);
        let st_row_size_y = get_split_size(size_y * self.stack_data, STACK_ROWS);
        let row_block_center_x = (col_size * 2.0) + get_center(st_row_size_x);

        let mut row_id = 0;
        while row_id < STACK_ROWS {
            st_row_pos[row_id as usize] = (
                row_block_center_x,
                size_y * self.get_sum_ratio_center(StackLayoutParts::StackDataRow(row_id + 1)),
            );
            row_id += 1;
        }

        ((st_row_size_x, st_row_size_y), st_row_pos)
    }

    /// Returns the size and center positions of each element in a row without a header.
    /// Positions are relative to the row
    pub const fn get_stack_rows_elems(
        &self,
        block_size_x: f32,
        block_size_y: f32,
    ) -> ((f32, f32), [(f32, f32); STACK_COLS_USIZE]) {
        let mut st_values_pos: [(f32, f32); STACK_COLS_USIZE] = [(0.0, 0.0); STACK_COLS_USIZE];

        let elem_size_x = get_split_size(block_size_x, STACK_COLS_HEADERLESS);
        let elem_size_y = block_size_y;

        let mut col_id = 0;
        let mut true_col_id = 0;

        while col_id < STACK_COLS_HEADERLESS {
            if col_id % (STACK_COLS_GROUP_SIZE + 1) == STACK_COLS_GROUP_SIZE {
                col_id += 1;
                continue;
            }

            st_values_pos[true_col_id] = (
                block_size_x * get_part_center(col_id + 1, STACK_COLS_HEADERLESS),
                get_center(elem_size_y),
            );

            true_col_id += 1;
            col_id += 1;
        }

        ((elem_size_x, elem_size_y), st_values_pos)
    }
}

/// Represents the vertical proportions of the different sections of the instructions log block
pub struct InstructionsLogLayout {
    header: f32,
    further_instructions: f32,
    splitter: f32,
    completed_instructions: f32,
}

#[derive(PartialEq)]
enum InstructionsLogLayoutParts {
    Header,
    FurherInstructions,
    Splitter,
    CompletedInstructions,
}

impl InstructionsLogLayout {
    /// Creates a instructions log layout with specified vertical proportions
    pub const fn new(
        header: f32,
        further_instructions: f32,
        splitter: f32,
        completed_instructions: f32,
    ) -> InstructionsLogLayout {
        assert!(header >= 0.0);
        assert!(further_instructions >= 0.0);
        assert!(splitter >= 0.0);
        assert!(completed_instructions >= 0.0);
        assert!(header + further_instructions + splitter + completed_instructions <= 1.0 + 0.005);

        InstructionsLogLayout {
            header,
            further_instructions,
            splitter,
            completed_instructions,
        }
    }

    /// Returns the normalized vertical position of the specified instructions log layout element
    const fn get_sum_ratio_center(&self, level: InstructionsLogLayoutParts) -> f32 {
        match level {
            InstructionsLogLayoutParts::Header => get_center(self.header),
            InstructionsLogLayoutParts::FurherInstructions => {
                self.header + get_center(self.further_instructions)
            }
            InstructionsLogLayoutParts::Splitter => {
                self.header + self.further_instructions + get_center(self.splitter)
            }
            InstructionsLogLayoutParts::CompletedInstructions => {
                self.header
                    + self.further_instructions
                    + self.splitter
                    + get_center(self.completed_instructions)
            }
        }
    }

    pub const fn get_header(&self, size_x: f32, size_y: f32) -> (f32, f32) {
        (
            get_center(size_x),
            size_y * self.get_sum_ratio_center(InstructionsLogLayoutParts::Header),
        )
    }
}
