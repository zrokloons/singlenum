use crate::utils::helpers::remove_element;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ABox {
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
}

impl ABox {
    pub fn get_square_ids(&self) -> Vec<usize> {
        vec![
            self._0, self._1, self._2, self._3, self._4, self._5, self._6, self._7, self._8,
        ]
    }

    pub fn set_taken(&mut self, value: usize) {
        self._taken.push(value);

        // Remove value from _remaining
        remove_element(value, &mut self._remaining);
    }
}

impl<'a> IntoIterator for &'a ABox {
    type Item = usize;
    type IntoIter = ABoxIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ABoxIntoIterator {
            abox: self,
            index: 0,
        }
    }
}

impl<'a> IntoIterator for &'a mut ABox {
    type Item = usize;
    type IntoIter = ABoxIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ABoxIntoIterator {
            abox: self,
            index: 0,
        }
    }
}

pub struct ABoxIntoIterator<'a> {
    abox: &'a ABox,
    index: usize,
}

impl<'a> Iterator for ABoxIntoIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let result = match self.index {
            0 => self.abox._0,
            1 => self.abox._1,
            2 => self.abox._2,
            3 => self.abox._3,
            4 => self.abox._4,
            5 => self.abox._5,
            6 => self.abox._6,
            7 => self.abox._7,
            8 => self.abox._8,
            _ => return None,
        };

        self.index += 1;
        Some(result)
    }
}
