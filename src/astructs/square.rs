use log;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Square {
    pub id: usize,
    pub value: usize,
    pub potentials: Vec<usize>,
    pub line_potentials: Vec<usize>,
    pub column_potentials: Vec<usize>,
    pub box_potentials: Vec<usize>,
    pub abox_id: usize,
    pub line_id: usize,
    pub column_id: usize,
    pub history: Vec<usize>,
    pub age: i32,
}

impl Hash for Square {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.age.hash(state);
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id: {:?}, value: {:?}, potentials: {:?}",
            self.id, self.value, self.potentials
        )
    }
}

impl Square {
    pub fn set_value(&mut self, value: usize, age: i32) {
        log::debug!(
            "[set_value] ID: {:?}, value: {:?}, | {:?} {:?} potentials {:?}",
            self.id,
            value,
            self.line_id,
            self.column_id,
            self.potentials,
        );
        self.value = value;
        self.age = age;
        self.potentials.clear();
        self.line_potentials.clear();
        self.column_potentials.clear();
        self.box_potentials.clear();
    }

    pub fn set_value_guess(&mut self, value: usize, age: i32) {
        log::debug!(
            "[set_value] ID: {:?}, value: {:?}, | {:?} {:?} potentials {:?}",
            self.id,
            value,
            self.line_id,
            self.column_id,
            self.potentials,
        );
        self.value = value;
        self.age = age;
        self.potentials.clear();
        self.line_potentials.clear();
        self.column_potentials.clear();
        self.box_potentials.clear();
        self.history.push(value);
    }

    /*
     * Set potentials.
     */
    pub fn set_potentials(&mut self, value: Vec<usize>) {
        self.potentials = value;
        //self.line_potentials.clear();
        //self.column_potentials.clear();
        //self.box_potentials.clear();
    }

    /*
     * Update Line, Column, and ABox potentials
     *
     */
    pub fn update(&mut self, target: &str, value: Vec<usize>) {
        match target {
            "line_potentials" => self.line_potentials = value,
            "column_potentials" => self.column_potentials = value,
            "box_potentials" => self.box_potentials = value,
            _ => panic!("Unsupported target: {target:?}"),
        }
    }

    /*
     * Get potentials
     *
     */
    pub fn get_potentials(&self) -> Option<&Vec<usize>> {
        if !self.potentials.is_empty() {
            Some(&self.potentials)
        } else {
            None
        }
    }
}
