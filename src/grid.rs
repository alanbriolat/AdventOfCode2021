use std::ops;

use crate::vector::Vector;

pub type Point<const N: usize> = Vector<i64, N>;

#[derive(Clone, Copy, Debug)]
struct Extent<const N: usize>(Vector<i64, N>);

impl<const N: usize> TryFrom<Point<N>> for Extent<N> {
    type Error = ();

    fn try_from(value: Point<N>) -> Result<Self, Self::Error> {
        if value.as_ref().iter().all(|&v| v > 0) {
            Ok(Extent(value))
        } else {
            Err(())
        }
    }
}

impl<const N: usize> Extent<N> {
    fn volume(&self) -> usize {
        self.0.as_ref().iter().product::<i64>() as usize
    }

    fn contains(&self, point: Point<N>) -> bool {
        (0..N).all(|i| point[i] >= 0 && point[i] < self.0[i])
    }

    fn row_major_index(&self, point: Point<N>) -> Option<usize> {
        if self.contains(point) {
            let size = self.0.as_ref();
            let index: i64 = (0..N)
                .map(|i| point[i] * size[..i].iter().product::<i64>())
                .sum();
            Some(index as usize)
        } else {
            None
        }
    }

    fn iter_points(self) -> impl Iterator<Item = Point<N>> {
        std::iter::successors(Some(Point::default()), move |prev| {
            let mut next = prev.clone();
            // Increment "innermost" index
            next[0] += 1;
            // Perform overflow of all but the "outermost" index
            for i in 0..(N - 1) {
                if next[i] == self.0[i] {
                    next[i] = 0;
                    next[i + 1] += 1;
                } else {
                    break;
                }
            }
            // If overflowed the "outermost" index, then there are no more points
            if next[N - 1] == self.0[N - 1] {
                None
            } else {
                Some(next)
            }
        })
    }
}

pub struct Grid<T, const N: usize> {
    extent: Extent<N>,
    offset: Point<N>,
    data: Vec<T>,
}

impl<T, const N: usize> Grid<T, N> {
    pub fn new<P: Into<Point<N>>>(size: P) -> Self {
        let extent: Extent<N> = Extent::try_from(size.into()).unwrap();
        Grid {
            extent,
            offset: Vector::default(),
            data: Vec::with_capacity(extent.volume()),
        }
    }

    pub fn with_offset<P: Into<Point<N>>>(mut self, offset: P) -> Self {
        self.offset = offset.into();
        self
    }

    pub fn with_data<I: IntoIterator<Item = T>>(mut self, data: I) -> Self {
        self.data.clear();
        self.data.extend(data.into_iter());
        self
    }

    pub fn contains<P: Into<Point<N>>>(&self, point: P) -> bool {
        self.extent.contains(point.into() - self.offset)
    }

    fn row_major_index(&self, point: Point<N>) -> Option<usize> {
        self.extent.row_major_index(point - self.offset)
    }

    pub fn iter_points(&self) -> impl Iterator<Item = Point<N>> {
        let offset = self.offset;
        self.extent.iter_points().map(move |point| point + offset)
    }

    pub fn get<P: Into<Point<N>>>(&self, point: P) -> Option<&T> {
        self.row_major_index(point.into())
            .and_then(|i| self.data.get(i))
    }

    pub fn get_mut<P: Into<Point<N>>>(&mut self, point: P) -> Option<&mut T> {
        self.row_major_index(point.into())
            .and_then(|i| self.data.get_mut(i))
    }
}

impl<T, P: Into<Point<N>>, const N: usize> ops::Index<P> for Grid<T, N> {
    type Output = T;

    fn index(&self, index: P) -> &Self::Output {
        &self.data[self.extent.row_major_index(index.into()).unwrap()]
    }
}

impl<T, P: Into<Point<N>>, const N: usize> ops::IndexMut<P> for Grid<T, N> {
    fn index_mut(&mut self, index: P) -> &mut Self::Output {
        &mut self.data[self.extent.row_major_index(index.into()).unwrap()]
    }
}

pub const GRID_DIRECTIONS_4: [[i64; 2]; 4] = [[-1, 0], [0, -1], [0, 1], [1, 0]];

pub const GRID_DIRECTIONS_8: [[i64; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];

impl<T> Grid<T, 2> {
    pub fn iter_adjacent_4_points<P: Into<Point<2>>>(
        &self,
        point: P,
    ) -> impl Iterator<Item = Point<2>> {
        let point: Point<2> = point.into();
        let extent = self.extent;
        let offset = self.offset;
        GRID_DIRECTIONS_4
            .iter()
            .map(move |direction| point + direction)
            .filter(move |p| extent.contains(*p - offset))
    }

    pub fn iter_adjacent_8_points<P: Into<Point<2>>>(
        &self,
        point: P,
    ) -> impl Iterator<Item = Point<2>> {
        let point: Point<2> = point.into();
        let extent = self.extent;
        let offset = self.offset;
        GRID_DIRECTIONS_8
            .iter()
            .map(move |direction| point + direction)
            .filter(move |p| extent.contains(*p - offset))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extent_2d() {
        let extent: Extent<2> = Extent::try_from(Point::from([5, 8])).unwrap();

        assert_eq!(extent.volume(), 40);

        assert!(extent.contains(Point::from([0, 0])));
        assert!(!extent.contains(Point::from([-1, 0])));
        assert!(!extent.contains(Point::from([0, -1])));
        assert!(extent.contains(Point::from([4, 7])));
        assert!(!extent.contains(Point::from([5, 7])));
        assert!(!extent.contains(Point::from([4, 8])));

        assert_eq!(extent.row_major_index(Point::from([3, 5])), Some(28));
        assert_eq!(extent.row_major_index(Point::from([5, 5])), None);
        assert_eq!(extent.row_major_index(Point::from([3, 8])), None);
        assert_eq!(extent.row_major_index(Point::from([-1, -1])), None);
        let points: Vec<_> = extent.iter_points().collect();
        assert_eq!(points.len(), extent.volume());
        assert_eq!(
            points[0..7],
            [
                Point::from([0, 0]),
                Point::from([1, 0]),
                Point::from([2, 0]),
                Point::from([3, 0]),
                Point::from([4, 0]),
                Point::from([0, 1]),
                Point::from([1, 1]),
            ]
        );
        assert_eq!(
            points[33..40],
            [
                Point::from([3, 6]),
                Point::from([4, 6]),
                Point::from([0, 7]),
                Point::from([1, 7]),
                Point::from([2, 7]),
                Point::from([3, 7]),
                Point::from([4, 7]),
            ]
        );
    }
}
