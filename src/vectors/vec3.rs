use std::io;
use std::io::{Read, Write};
use std::ops::{Add, Mul, Sub};
use num::{Num, One, ToPrimitive, Zero};
use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};

use pixel_renderer_model_helper::binary_serializable::BinarySerializable;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }
}

impl<T: Default> Default for Vec3<T> {
    fn default() -> Self {
        Vec3::new(T::default(), T::default(), T::default())
    }
}

impl<T: Zero> Zero for Vec3<T> {
    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
    }
    
    fn zero() -> Self {
        Vec3::new(T::zero(), T::zero(), T::zero())
    }
}

impl<T: One> One for Vec3<T> {
    fn one() -> Self {
        Vec3::new(T::one(), T::one(), T::one())
    }
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

// NOT DOT OR CROSS PRODUCT
impl<T: Mul<Output = T>> Mul for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl<T: Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Copy> Vec3<T> {
    pub fn dot(&self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn length(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl<T: Num + ToPrimitive + Copy> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl BinarySerializable for Vec3<f32> {
    fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
        let x = reader.read_f32::<NativeEndian>()?;
        let y = reader.read_f32::<NativeEndian>()?;
        let z = reader.read_f32::<NativeEndian>()?;
        Ok(Vec3 { x, y, z })
    }

    fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_f32::<NativeEndian>(self.x)?;
        writer.write_f32::<NativeEndian>(self.y)?;
        writer.write_f32::<NativeEndian>(self.z)
    }
}

impl BinarySerializable for Vec3<u32> {
    fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
        let x = reader.read_u32::<NativeEndian>()?;
        let y = reader.read_u32::<NativeEndian>()?;
        let z = reader.read_u32::<NativeEndian>()?;
        Ok(Vec3 { x, y, z })
    }

    fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u32::<NativeEndian>(self.x)?;
        writer.write_u32::<NativeEndian>(self.y)?;
        writer.write_u32::<NativeEndian>(self.z)
    }
}