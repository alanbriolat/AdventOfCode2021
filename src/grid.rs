use num::CheckedSub;
use std::ops;

use crate::vector::Vector;

pub trait GridOps: ops::IndexMut<Self::Point> {
    // type Data;
    type Point;

    fn valid_point(&self, point: &Self::Point) -> bool;
    fn iter_points(&self) -> Box<dyn Iterator<Item = Self::Point>>;
    // fn iter_adjacent_4(&self, point: &Self::Point) -> Box<dyn Iterator<Item = Self::Point> + '_>;
    fn iter_adjacent_8(&self, point: &Self::Point) -> Box<dyn Iterator<Item = Self::Point>>;
    // fn get(&self, point: &Self::Point) -> Option<&Self::Data>;
    // fn get_mut(&mut self, point: &Self::Point) -> Option<&mut Self::Data>;
}

pub struct DynamicGrid2D<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> DynamicGrid2D<T> {
    pub fn new<I: IntoIterator<Item = T>>(width: usize, height: usize, data: I) -> Self {
        DynamicGrid2D {
            width,
            height,
            data: data.into_iter().take(width * height).collect(),
        }
    }

    fn index_from_point(&self, point: &Vector<usize, 2>) -> Option<usize> {
        if self.valid_point(point) {
            Some(self.width * point[1] + point[0])
        } else {
            None
        }
    }
}

impl<T> GridOps for DynamicGrid2D<T> {
    type Point = Vector<usize, 2>;

    fn valid_point(&self, point: &Self::Point) -> bool {
        (0..self.width).contains(&point[0]) && (0..self.height).contains(&point[1])
    }

    fn iter_points(&self) -> Box<dyn Iterator<Item = Self::Point>> {
        let (width, height) = (self.width, self.height);
        Box::new((0..height).flat_map(move |y| (0..width).map(move |x| [x, y].into())))
    }

    // fn iter_adjacent_4(&self, point: &Self::Point) -> Box<dyn Iterator<Item=Self::Point> + '_> {
    //     todo!()
    // }

    fn iter_adjacent_8(&self, point: &Self::Point) -> Box<dyn Iterator<Item = Self::Point>> {
        const VERTICAL: [usize; 2] = [0, 1];
        const HORIZONTAL: [usize; 2] = [1, 0];
        let up: Option<Self::Point> = point.checked_sub(&VERTICAL.into());
        let down: Option<Self::Point> = Some(*point + &VERTICAL).filter(|p| self.valid_point(p));
        let left: Option<Self::Point> = point.checked_sub(&HORIZONTAL.into());
        let right: Option<Self::Point> = Some(*point + &HORIZONTAL).filter(|p| self.valid_point(p));
        let candidates = [
            up.and_then(|p| p.checked_sub(&HORIZONTAL.into())),
            up,
            up.and_then(|p| Some(p + &HORIZONTAL).filter(|p| self.valid_point(p))),
            left,
            right,
            down.and_then(|p| p.checked_sub(&HORIZONTAL.into())),
            down,
            down.and_then(|p| Some(p + &HORIZONTAL).filter(|p| self.valid_point(p))),
        ];
        Box::new(candidates.into_iter().flatten())
    }
}

impl<T> ops::Index<Vector<usize, 2>> for DynamicGrid2D<T> {
    type Output = T;

    fn index(&self, index: Vector<usize, 2>) -> &Self::Output {
        &self.data[self.index_from_point(&index).unwrap()]
    }
}

impl<T> ops::IndexMut<Vector<usize, 2>> for DynamicGrid2D<T> {
    fn index_mut(&mut self, index: Vector<usize, 2>) -> &mut Self::Output {
        let index = self.index_from_point(&index).unwrap();
        &mut self.data[index]
    }
}
