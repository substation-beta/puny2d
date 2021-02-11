// Imports
use super::types::{Coordinate,Degree};
use crate::math::FloatExt;
use std::ops::{Add,AddAssign,Sub,Mul,Neg};


// Generic point with math
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct GenericPoint<T> {
    pub x: T,
    pub y: T
}
impl<T: Add<Output = T>> Add for GenericPoint<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}
impl<T: AddAssign> AddAssign for GenericPoint<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl<T: Sub<Output = T>> Sub for GenericPoint<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}
impl<T: Mul<Output = T> + Copy> Mul<T> for GenericPoint<T> {
    type Output = Self;
    fn mul(self, factor: T) -> Self::Output {
        Self {
            x: self.x * factor,
            y: self.y * factor
        }
    }
}
impl<T: Neg<Output = T>> Neg for GenericPoint<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y
        }
    }
}

// Point conversions
macro_rules! impl_from {
    ($FROM:ty,$TO:ty) => {
        impl From<GenericPoint<$FROM>> for GenericPoint<$TO> {
            fn from(point: GenericPoint<$FROM>) -> Self {
                Self {
                    x: point.x as $TO,
                    y: point.y as $TO
                }
            }
        }
    }
}
impl_from!(Degree,Coordinate);
impl_from!(Coordinate,Degree);
impl_from!(Coordinate,u16);

// Point as path segment
pub type Point = GenericPoint<Coordinate>;

// Default point (possible to reference)
pub static ORIGIN_POINT: Point = Point {x: 0.0, y: 0.0};

// Point methods
#[allow(clippy::len_without_is_empty)]
impl Point {
    pub fn len(self) -> Coordinate {
        self.x.hypot(self.y)
    }
    pub fn round(self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round()
        }
    }
    pub fn round_half_down(self) -> Self {
        Self {
            x: self.x.round_half_down(),
            y: self.y.round_half_down()
        }
    }
    pub fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y)
        }
    }
    pub fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y)
        }
    }
}

// Point collections
pub trait PointMinMaxCollector<'origin>: Iterator<Item=&'origin Point> {
    fn min_max(self) -> Option<(Point,Point)>;
}
impl <'origin, I: Iterator<Item=&'origin Point>> PointMinMaxCollector<'origin> for I {
    fn min_max(self) -> Option<(Point,Point)> {
        self.fold(None, |mut min_max_points, point|
            if let Some((min_point, max_point)) = min_max_points.as_mut() {
                if point.x < min_point.x {min_point.x = point.x;}
                if point.y < min_point.y {min_point.y = point.y;}
                if point.x > max_point.x {max_point.x = point.x;}
                if point.y > max_point.y {max_point.y = point.y;}
                min_max_points
            } else {
                Some((
                    *point,
                    *point
                ))
            }
        )
    }
}