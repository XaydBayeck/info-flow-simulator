/// 物理帧的时间间隔
pub const TIME_STEP: f64 = 1.0 / 60.0;
/// 单元格的尺寸
pub const CELL_SIZE: f32 = 10.0;
/// 单元格的边框厚度
pub const BORDER: f32 = CELL_SIZE / 10.;
/// 默认的元胞数量（行列）: 100 x 100
pub const DEFAULT_WORLD_SIZE: WorldSize = WorldSize::new();

pub struct WorldSize {
    row: usize,
    col: usize,
}

impl WorldSize {
    pub const fn new() -> Self {
        Self { col: 100, row: 100 }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }
}

impl From<(usize, usize)> for WorldSize {
    fn from((row, col): (usize, usize)) -> Self {
        Self { row, col }
    }
}
