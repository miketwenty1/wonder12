#[derive(Debug)]
pub enum EdgeType {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Clone)]
pub struct EdgeData {
    pub tile: i32,
    pub pixel: f32,
}

#[derive(Clone)]
pub enum TileResource {
    Wheat,
    // Brick,
    // Sheep,
    // Wood,
    // Stone,
    // Desert,
    // Water,
    // Grass,
    // Unknown,
}

#[derive(Debug)]
pub struct SpawnDiffData {
    pub xstart: i32,
    pub xend: i32,
    pub ystart: i32,
    pub yend: i32,
}
