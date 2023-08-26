use bevy::prelude::*;

pub const CHUNK_SIZE: i32 = 16;
pub const CHUNK_HEIGHT: i32 = 256;
pub const CHUNK_VOLUME: i32 = CHUNK_SIZE * CHUNK_SIZE * CHUNK_HEIGHT;

pub const GRID_TILE_SIZE: f32 = 4.0;

pub struct GridBlock {
    //0 Forces
    //1 Effects
    pub data: Vec<u8>,
}
#[derive(Component)]
pub struct Chunk {
    pub gridblocks: Vec<GridBlock>,
}
impl Chunk {
    pub fn new() -> Self {
        Self {
            gridblocks: vec![
                GridBlock {
                    data: vec![0; 3 as usize]
                };
                CHUNK_SIZE as usize * CHUNK_SIZE as usize * CHUNK_HEIGHT as usize
            ],
        }
    }
}

pub struct Coordinates {
    pub position: IVec3,
    pub precise: Vec3,
}

impl Coordinates {
    #[inline]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            position: IVec3::new(x, y, z),
            precise: Vec3::new(0.5, 0.0, 0.5),
        }
    }

    pub fn new_from_transform(transform: &Transform) -> Self {
        let translation = transform.translation();
        Self {
            position: IVec3::new(
                translation.x as i32,
                translation.y as i32,
                translation.z as i32,
            ),
            precise: Vec3::new(
                translation.x - translation.x.floor().normalize_or_zero(),
                translation.y - translation.y.floor().normalize_or_zero(),
                translation.z - translation.z.floor().normalize_or_zero(),
            ),
        }
    }

    pub fn transform(&self) -> Transform {
        Transform::from_translation(self.position + self.precise)
    }
}
