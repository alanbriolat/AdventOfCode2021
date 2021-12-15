use std::cmp;
use std::fmt;
use std::ops;
use std::str::FromStr;

pub trait Coord: num::Integer + num::CheckedSub + num::ToPrimitive + Copy {}
impl<T: num::Integer + num::CheckedSub + num::ToPrimitive + Copy> Coord for T {}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Vector<C: Coord, const N: usize>([C; N]);

impl<C: Coord + fmt::Debug, const N: usize> fmt::Debug for Vector<C, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vector({:?})", self.0)
    }
}

impl<C: Coord, const N: usize> Default for Vector<C, N> {
    fn default() -> Self {
        Vector([C::zero(); N])
    }
}

impl<C: Coord, const N: usize> From<[C; N]> for Vector<C, N> {
    fn from(raw: [C; N]) -> Self {
        Vector(raw)
    }
}

impl<C: Coord, const N: usize> TryFrom<&[C]> for Vector<C, N> {
    type Error = ();

    fn try_from(value: &[C]) -> Result<Self, Self::Error> {
        if value.len() != N {
            Err(())
        } else {
            let mut new = Self::default();
            new.0.copy_from_slice(value);
            Ok(new)
        }
    }
}

impl<C: Coord, const N: usize> AsRef<[C]> for Vector<C, N> {
    fn as_ref(&self) -> &[C] {
        self.0.as_ref()
    }
}

impl<C: Coord, const N: usize> ops::Index<usize> for Vector<C, N> {
    type Output = C;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<C: Coord, const N: usize> ops::IndexMut<usize> for Vector<C, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: AsRef<[C]>, C: Coord, const N: usize> ops::Add<T> for Vector<C, N> {
    type Output = Self;

    fn add(mut self, rhs: T) -> Self::Output {
        let rhs = rhs.as_ref();
        for i in 0..N {
            self[i] = self[i] + rhs[i];
        }
        self
    }
}

impl<T: AsRef<[C]>, C: Coord, const N: usize> ops::Sub<T> for Vector<C, N> {
    type Output = Self;

    fn sub(mut self, rhs: T) -> Self::Output {
        let rhs = rhs.as_ref();
        for i in 0..N {
            self[i] = self[i] - rhs[i];
        }
        self
    }
}

impl<T: AsRef<[C]>, C: Coord, const N: usize> ops::Mul<T> for Vector<C, N> {
    type Output = Self;

    fn mul(mut self, rhs: T) -> Self::Output {
        let rhs = rhs.as_ref();
        for i in 0..N {
            self[i] = self[i] * rhs[i];
        }
        self
    }
}

impl<C: Coord, const N: usize> num::CheckedSub for Vector<C, N> {
    fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        let mut new = self.clone();
        for i in 0..N {
            new[i] = self[i].checked_sub(&rhs[i])?;
        }
        Some(new)
    }
}

impl<C: Coord, const N: usize> Vector<C, N> {
    pub fn min(&self, rhs: &Self) -> Self {
        let mut new = self.clone();
        for i in 0..N {
            new[i] = cmp::min(new[i], rhs[i]);
        }
        new
    }

    pub fn max(&self, rhs: &Self) -> Self {
        let mut new = self.clone();
        for i in 0..N {
            new[i] = cmp::max(new[i], rhs[i]);
        }
        new
    }
}

impl<C: Coord + FromStr, const N: usize> FromStr for Vector<C, N> {
    type Err = C::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut new = Self::default();
        for (i, part) in s.split(',').enumerate() {
            new[i] = part.parse()?;
        }
        Ok(new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_basics() {
        type Vec = Vector<i64, 3>;
        assert_eq!(Vec::default(), Vec::from([0, 0, 0]));
        assert_eq!(
            Vec::from([1, 2, 3]) + Vec::from([4, 5, 6]),
            Vec::from([5, 7, 9])
        );
        assert_eq!(Vec::from([1, 2, 3]) + [4, 5, 6], Vec::from([5, 7, 9]));
        assert_eq!(Vec::from([1, 2, 3]) * [2, 3, 4], Vec::from([2, 6, 12]));
    }
}
