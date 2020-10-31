use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A two-dimensional vector.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Debug, Copy, Default, Hash)]
pub struct Vector2<T> {
    /// The x-component of the vector.
    pub x: T,

    /// The y-component of the vector.
    pub y: T,
}

impl<T> Vector2<T> {
    /// Returns a new [`Vector2`] with the given components.
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> From<(T, T)> for Vector2<T> {
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

macro_rules! impl_ops {
    ( $_trait:ident, $_func:ident, $( $_type:ty ),+ ) => {
        impl<T: $_trait + Copy> $_trait<T> for Vector2<T> {
            type Output = Vector2<T::Output>;

            fn $_func(self, rhs: T) -> Vector2<T::Output> {
                Vector2 {
                    x: $_trait::$_func(self.x, rhs),
                    y: $_trait::$_func(self.y, rhs)
                }
            }
        }

        $(
            impl $_trait<Vector2<$_type>> for $_type {
                type Output = Vector2<$_type>;

                fn $_func(self, rhs: Vector2<$_type>) -> Vector2<$_type> {
                    Vector2 {
                        x: $_trait::$_func(self, rhs.x),
                        y: $_trait::$_func(self, rhs.y)
                    }
                }
            }
        )+
    }
}

impl_ops!(Add, add, i32, u32, f32);
impl_ops!(Sub, sub, i32, u32, f32);
impl_ops!(Mul, mul, i32, u32, f32);
impl_ops!(Div, div, i32, u32, f32);

impl<T: Add> Add for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn add(self, rhs: Vector2<T>) -> Vector2<T::Output> {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: AddAssign> AddAssign for Vector2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub> Sub for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn sub(self, rhs: Vector2<T>) -> Vector2<T::Output> {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: SubAssign> SubAssign for Vector2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Mul> Mul for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn mul(self, rhs: Vector2<T>) -> Vector2<T::Output> {
        Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vector2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Div> Div for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn div(self, rhs: Vector2<T>) -> Vector2<T::Output> {
        Vector2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T: DivAssign + Copy> DivAssign<T> for Vector2<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T: Neg<Output = T>> Neg for Vector2<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

/// A [`Vector2`] with [`i32`] components.
pub type Vector2i = Vector2<i32>;

impl Vector2i {
    /// A vector with components (0, 0).
    pub const ZERO: Self = Self { x: 0, y: 0 };

    /// A vector with components (1, 0).
    pub const UNIT_X: Self = Self { x: 1, y: 0 };

    /// A vector with components (0, 1).
    pub const UNIT_Y: Self = Self { x: 0, y: 1 };

    /// A vector with components (1, 1).
    pub const UNIT: Self = Self { x: 1, y: 1 };
}

/// A [`Vector2`] with [`u32`] components.
pub type Vector2u = Vector2<u32>;

impl Vector2u {
    /// A vector with components (0, 0).
    pub const ZERO: Self = Self { x: 0u32, y: 0u32 };

    /// A vector with components (1, 0).
    pub const UNIT_X: Self = Self { x: 1u32, y: 0u32 };

    /// A vector with components (0, 1).
    pub const UNIT_Y: Self = Self { x: 0u32, y: 1u32 };

    /// A vector with components (1, 1).
    pub const UNIT: Self = Self { x: 1u32, y: 1u32 };
}

/// A [`Vector2`] with [`f32`] components.
pub type Vector2f = Vector2<f32>;

impl Vector2f {
    /// A vector with components (0, 0).
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    /// A vector with components (1, 0).
    pub const UNIT_X: Self = Self { x: 1.0, y: 0.0 };

    /// A vector with components (0, 1).
    pub const UNIT_Y: Self = Self { x: 0.0, y: 1.0 };

    /// A vector with components (1, 1).
    pub const UNIT: Self = Self { x: 1.0, y: 1.0 };

    pub fn normalize(self) -> Self {
        let value = 1.0 / f32::sqrt((self.x * self.x) + (self.y * self.y));
        Self {
            x: self.x * value,
            y: self.y * value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_use_the_addition_operator() {
        assert_eq!(
            Vector2i::new(1, 2) + Vector2i::new(3, 4),
            Vector2i::new(4, 6)
        );
    }
}
