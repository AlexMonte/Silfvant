use bevy::prelude::*;
use bevy::prelude::{FromReflect, Reflect};
use std::mem::transmute;

pub(crate) trait Quantize<T> {
    fn quantize(&self, value: T) -> Self;
    fn dequantize(&self) -> T;
    //fn from_target(&self, current: T, target: T) -> Self;
}
pub(crate) const X_MASK: i64 = 0b11111111111111111111;
pub(crate) const Y_MASK: i64 = 0b1111111111111111111100000000000000000000;
pub(crate) const Z_MASK: i64 = 0b111111111111111111110000000000000000000000000000000000000000;

pub(crate) const X_MASK2: i64 = 0b111111111111111111111111111111;
pub(crate) const Y_MASK2: i64 = 0b111111111111111111111111111111000000000000000000000000000000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Reflect)]
pub(crate) struct QVec3(pub i64);

impl Quantize<Vec3> for QVec3 {
    fn quantize(&self, value: Vec3) -> QVec3 {
        let x: f32 = (value.x).clamp(-0.000488, 1.0);
        let y: f32 = (value.y).clamp(-0.000488, 1.0);
        let z: f32 = (value.z).clamp(-0.000488, 1.0);

        let x_i64: i64 = (unsafe { transmute::<f32, u32>(x) } as i64) & X_MASK;
        let y_i64: i64 = ((unsafe { transmute::<f32, u32>(y) } as i64) << 20) & Y_MASK;
        let z_i64: i64 = ((unsafe { transmute::<f32, u32>(z) } as i64) << 40) & Z_MASK;

        let combined_i64 = x_i64 | y_i64 | z_i64;
        Self(combined_i64)
    }

    fn dequantize(&self) -> Vec3 {
        let x = (self.0 & X_MASK) as i32;
        let y = ((self.0 & Y_MASK) >> 20) as i32;
        let z = ((self.0 & Z_MASK) >> 40) as i32;

        let x_f32 = unsafe { transmute::<i32, f32>(x) };
        let y_f32 = unsafe { transmute::<i32, f32>(y) };
        let z_f32 = unsafe { transmute::<i32, f32>(z) };

        Vec3::new(x_f32, y_f32, z_f32)
    }
}
impl FromReflect for QVec3 {
    fn from_reflect(value: &dyn Reflect) -> Option<Self> {
        // Implement the FromReflect trait for StateValue

        unimplemented!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Reflect)]
pub(crate) struct QIVec3(pub i64);

impl Quantize<IVec3> for QIVec3 {
    fn quantize(&self, current: IVec3) -> QIVec3 {
        let x: i64 = ((current.x).clamp(-524288, 524287) as i64) & X_MASK;
        let y: i64 = (((current.y).clamp(-524288, 524287) as i64) << 20) & Y_MASK;
        let z: i64 = (((current.z).clamp(-524288, 524287) as i64) << 40) & Z_MASK;

        Self(x | y | z)
    }
    fn dequantize(&self) -> IVec3 {
        let x = (self.0 & X_MASK) as i32;
        let y = ((self.0 & Y_MASK) >> 20) as i32;
        let z = ((self.0 & Z_MASK) >> 40) as i32;

        IVec3::new(x, y, z)
    }
}

impl FromReflect for QIVec3 {
    fn from_reflect(value: &dyn Reflect) -> Option<Self> {
        // Implement the FromReflect trait for StateValue

        unimplemented!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Reflect)]
pub(crate) struct QIVec2(pub i64);

impl Quantize<IVec2> for QIVec2 {
    fn quantize(&self, value: IVec2) -> QIVec2 {
        let x: i64 = ((value.x).clamp(-32767, 32767) as i64) & X_MASK2;
        let y: i64 = (((value.y).clamp(-32767, 32767) as i64) << 11) & Y_MASK2;
        Self(x | y)
    }
    fn dequantize(&self) -> IVec2 {
        let x = (self.0 & X_MASK2) as i32;
        let y = ((self.0 & Y_MASK2) >> 16) as i32;

        IVec2::new(x, y)
    }
}

impl FromReflect for QIVec2 {
    fn from_reflect(value: &dyn Reflect) -> Option<Self> {
        // Implement the FromReflect trait for StateValue

        unimplemented!()
    }
}
