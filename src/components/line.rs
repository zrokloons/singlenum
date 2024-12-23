use crate::utils::helpers::remove_element;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Line {
    pub _id: usize,
    pub _taken: Vec<usize>,
    pub _remaining: Vec<usize>,
    pub _0: usize,
    pub _1: usize,
    pub _2: usize,
    pub _3: usize,
    pub _4: usize,
    pub _5: usize,
    pub _6: usize,
    pub _7: usize,
    pub _8: usize,
    pub _squares: Vec<usize>,
}

impl Line {
    pub fn new(id: usize, square_ids: Vec<usize>) -> Line {
        Line {
            _id: id,
            _taken: Vec::new(),
            _remaining: Vec::new(),
            _0: square_ids[0],
            _1: square_ids[1],
            _2: square_ids[2],
            _3: square_ids[3],
            _4: square_ids[4],
            _5: square_ids[5],
            _6: square_ids[6],
            _7: square_ids[7],
            _8: square_ids[8],
            _squares: square_ids,
        }
    }

    pub fn get_square_ids(&self) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        for l in self {
            result.push(l);
        }
        result
    }

    pub fn set_taken(&mut self, value: usize) {
        self._taken.push(value);

        // Remove value from _remaining
        remove_element(value, &mut self._remaining);
    }
}

impl<'a> IntoIterator for &'a Line {
    type Item = usize;
    type IntoIter = LineIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LineIntoIterator {
            line: self,
            index: 0,
        }
    }
}

impl<'a> IntoIterator for &'a mut Line {
    type Item = usize;
    type IntoIter = LineIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LineIntoIterator {
            line: self,
            index: 0,
        }
    }
}

pub struct LineIntoIterator<'a> {
    line: &'a Line,
    index: usize,
}

impl<'a> Iterator for LineIntoIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let result = match self.index {
            0 => self.line._0,
            1 => self.line._1,
            2 => self.line._2,
            3 => self.line._3,
            4 => self.line._4,
            5 => self.line._5,
            6 => self.line._6,
            7 => self.line._7,
            8 => self.line._8,
            _ => return None,
        };

        self.index += 1;
        Some(result)
    }
}
