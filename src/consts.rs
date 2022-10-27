/// 物理帧的时间间隔
pub const TIME_STEP: f64 = 10.0 / 60.0;
/// 单元格的尺寸
pub const CELL_SIZE: f32 = 10.0;
/// 单元格的边框厚度
pub const BORDER: f32 = CELL_SIZE / 10.;
/// 默认的元胞数量（行列）: 100 x 100
pub const DEFAULT_WORLD_SIZE: WorldSize = WorldSize::new(100, 100);
/// Max evaluate times
pub const MAX_EVALUATE_TIMES: usize = 20;

pub struct WorldSize {
    row: i32,
    col: i32,
}

impl WorldSize {
    pub const fn new(col: i32, row: i32) -> Self {
        Self { col, row }
    }

    pub fn row(&self) -> i32 {
        self.row
    }

    pub fn col(&self) -> i32 {
        self.col
    }
}

impl From<(i32, i32)> for WorldSize {
    fn from((row, col): (i32, i32)) -> Self {
        Self { row, col }
    }
}
