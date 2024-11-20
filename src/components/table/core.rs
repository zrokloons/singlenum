use crate::components::abox::ABox;
use crate::components::column::Column;
use crate::components::line::Line;
use crate::components::square::Square;
use crate::enums::{Container, Progress, SetKind};
use crate::utils::helpers;
use anyhow::anyhow;
use anyhow::Result as AnyhowResult;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapShot {
    square: Vec<Square>,
    line: Vec<Line>,
    column: Vec<Column>,
    abox: Vec<ABox>,
    value: usize,
    square_id: usize,
}

impl Hash for SnapShot {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.square.hash(state);
        self.line.hash(state);
        self.column.hash(state);
        self.abox.hash(state);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Table {
    abox: Vec<ABox>,
    line: Vec<Line>,
    column: Vec<Column>,
    pub squares: Vec<Square>,
    snapshots: Vec<SnapShot>,
    max_attempts: i32,
    iteration: i32,
    snapshots_taken: usize,
    snapshot_rollbacks: usize,
}

impl Hash for Table {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.squares.hash(state);
    }
}

impl Table {
    pub fn new(configuration: Vec<usize>, max_attempts: i32) -> Table {
        let mut a: Vec<ABox> = Vec::new();
        let mut a_id: usize;
        let mut l: Vec<Line> = Vec::new();
        let mut c: Vec<Column> = Vec::new();
        let mut s: Vec<Square> = Vec::new();

        for (index, value) in configuration.iter().enumerate() {
            a_id = match index {
                0 | 1 | 2 | 9 | 10 | 11 | 18 | 19 | 20 => 0,
                3 | 4 | 5 | 12 | 13 | 14 | 21 | 22 | 23 => 1,
                6 | 7 | 8 | 15 | 16 | 17 | 24 | 25 | 26 => 2,
                27 | 28 | 29 | 36 | 37 | 38 | 45 | 46 | 47 => 3,
                30 | 31 | 32 | 39 | 40 | 41 | 48 | 49 | 50 => 4,
                33 | 34 | 35 | 42 | 43 | 44 | 51 | 52 | 53 => 5,
                54 | 55 | 56 | 63 | 64 | 65 | 72 | 73 | 74 => 6,
                57 | 58 | 59 | 66 | 67 | 68 | 75 | 76 | 77 => 7,
                60 | 61 | 62 | 69 | 70 | 71 | 78 | 79 | 80 => 8,
                _ => panic!(
                    "Number of squares in configuration input is: {} > 81",
                    configuration.len()
                ),
            };

            let t = Square {
                id: index,
                value: *value,
                potentials: Vec::new(),
                line_potentials: Vec::new(),
                column_potentials: Vec::new(),
                box_potentials: Vec::new(),
                abox_id: a_id,
                line_id: index / 9,
                column_id: index % 9,
                history: Vec::new(),
            };

            s.push(t);
            let numbers: Vec<usize> = (0..=8).collect();

            match index {
                8 | 17 | 26 | 35 | 44 | 53 | 62 | 71 | 80 => l.push(Line::new(
                    index / 9,
                    numbers.iter().map(|x| index - x).collect(),
                )),
                _ => (),
            }

            if let 72..=80 = index {
                c.push(Column::new(
                    index % 9,
                    numbers.iter().map(|x| index - (9 * x)).collect(),
                ));
            }

            match index {
                20 | 23 | 26 | 47 | 50 | 53 | 74 | 77 | 80 => a.push(ABox::new(
                    a_id,
                    [
                        index - 9 - 9 - 2,
                        index - 9 - 9 - 1,
                        index - 9 - 9,
                        index - 9 - 2,
                        index - 9 - 1,
                        index - 9,
                        index - 2,
                        index - 1,
                        index,
                    ]
                    .to_vec(),
                )),
                _ => (),
            }
        }

        Table {
            abox: a,
            line: l,
            column: c,
            squares: s,
            snapshots: Vec::new(),
            snapshots_taken: 0,
            max_attempts,
            iteration: 0,
            snapshot_rollbacks: 0,
        }
    }

    /*
     * Puzzle finished
     */
    pub fn complete(&mut self) -> Progress {
        self.iteration += 1;
        let progress: &usize = &self.squares.iter().filter(|x| x.value != 0).count();
        let msg = format!(
            "[iterations: {}, snapshots: {}, rollbacks: {}]",
            self.iteration, self.snapshots_taken, self.snapshot_rollbacks
        );

        if *progress == 81 {
            Progress::Solved(msg)
        } else if self.iteration == self.max_attempts {
            Progress::LimitReached(msg)
        } else {
            Progress::InProgress(self.iteration)
        }
    }

    /*
     * Restore to last snapshot.
     *
     * The earlier guess we made was not successful. Therefore we need to revert
     * back to the last snapshot (taken just before last guess was made).
     *
     * After the rollback we need to update the square with the history of the
     * guess that did not lead anywhere. This shall prevent us from going that
     * route again.
     */
    pub fn snapshot_rollback(&mut self) -> AnyhowResult<()> {
        if self.snapshots.is_empty() {
            panic!("ERROR, there is no snapshot!");
        }

        let snapshot = self.snapshots.pop().unwrap();
        log::debug!("[snapshot] Roll back to snapshot");

        self.squares = snapshot.square;
        self.line = snapshot.line;
        self.column = snapshot.column;
        self.abox = snapshot.abox;

        self.snapshot_rollbacks += 1;

        // We need to update the square used in snapshot to include the value
        // used in their history.
        let square = self.get_square_mut(snapshot.square_id)?;
        square.history.push(snapshot.value);
        Ok(())
    }

    /*
     * Qualified Guess
     *
     * This means that we take one square that have few hard potentials
     * and set it to one of them, then we see how it goes ;)
     */
    pub fn qualified_guess(&mut self) -> AnyhowResult<bool> {
        let mut snapshot = self.prepare_snapshot();
        let mut update: Option<(usize, usize)> = None;

        'outer: for square in self.squares.iter() {
            if !square.potentials.is_empty() {
                for potential in &square.potentials {
                    if square.history.contains(potential) {
                        continue;
                    }
                    update = Some((square.id, *potential));
                    break 'outer;
                }
            }
        }

        if let Some(update) = update {
            let (square_id, value) = update;
            self.set_square(square_id, value, SetKind::GUESS)?;

            snapshot.square_id = square_id;
            snapshot.value = value;

            self.snapshot_take(snapshot);
            log::debug!("[guess] Qualified Guess -> true");
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn snapshot_take(&mut self, snapshot: SnapShot) {
        self.snapshots.push(snapshot);
        self.snapshots_taken += 1;
        log::debug!("[snapshot] Snapshot taken");
    }

    pub fn incompetent_guess(&mut self) -> AnyhowResult<bool> {
        let mut snapshot = self.prepare_snapshot();
        let mut update: Option<(usize, usize)> = None;

        'outer: for square in self.squares.iter() {
            let potentials = helpers::multi_intersections(vec![
                square.box_potentials.clone(),
                square.line_potentials.clone(),
                square.column_potentials.clone(),
            ]);

            if !potentials.is_empty() {
                for potential in potentials.iter() {
                    if square.history.contains(potential) {
                        continue;
                    }
                    update = Some((square.id, *potential));
                    break 'outer;
                }
            }
        }

        if let Some(update) = update {
            let (square_id, value) = update;
            self.set_square(square_id, value, SetKind::GUESS)?;

            snapshot.square_id = square_id;
            snapshot.value = value;

            self.snapshot_take(snapshot);
            log::debug!("[guess] Incompetent Guess -> true");
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /*
     * The engine will run multiple different routines to conclude
     * if a square can set a value.
     *
     * As soon an update has been done we need to break to make sure
     * that we get an update on our data.
     */
    pub fn engine(&mut self) -> AnyhowResult<bool> {
        let mut updated: bool;

        updated = self.engine_line_one_left()?;
        if updated {
            let valid: bool = self.validate()?;
            if !valid {
                self.snapshot_rollback()?;
            }
            return Ok(true);
        }

        updated = self.engine_column_one_left()?;
        if updated {
            let valid: bool = self.validate()?;
            if !valid {
                self.snapshot_rollback()?;
            }
            return Ok(true);
        }

        updated = self.engine_box_one_left()?;
        if updated {
            let valid: bool = self.validate()?;
            if !valid {
                self.snapshot_rollback()?;
            }
            return Ok(true);
        }

        updated = self.engine_only_one_possible()?;
        if updated {
            let valid: bool = self.validate()?;
            if !valid {
                self.snapshot_rollback()?;
            }
            return Ok(true);
        }

        updated = self.engine_box()?;
        if updated {
            let valid: bool = self.validate()?;
            if !valid {
                self.snapshot_rollback()?;
            }
            return Ok(true);
        }

        Ok(false)
    }

    /*
     * Update square Line Column and ABox
     *
     * This update inspect all squares and update corrspoinding
     * values on each struct.
     *
     */
    pub fn update(&mut self) -> AnyhowResult<&mut Self> {
        self.update_line()?;
        self.update_column()?;
        self.update_abox()?;
        self.update_square_potentials()?;
        self.update_box_remove_potentials()?;
        Ok(self)
    }

    /*
     * Set a square value
     *
     * Once the value have been set, update Line, Column and ABox
     *
     * The method support two kinds of set value, Normal and Guess. We need
     * to seperate them apart so that we can store the guessed value in the
     * history. When we do not guess it should not be needed.
     *
     */
    fn set_square(&mut self, square_id: usize, value: usize, kind: SetKind) -> AnyhowResult<&Self> {
        self.squares[square_id].set_value(value, kind);

        // Update Line, Column and ABox
        self.line[self.squares[square_id].line_id].set_taken(value);
        self.column[self.squares[square_id].column_id].set_taken(value);
        self.abox[self.squares[square_id].abox_id].set_taken(value);

        Ok(self)
    }

    /*
     * Prepare a snapshot
     *
     */
    fn prepare_snapshot(&mut self) -> SnapShot {
        SnapShot {
            square: self.squares.clone(),
            line: self.line.clone(),
            column: self.column.clone(),
            abox: self.abox.clone(),
            value: 99,
            square_id: 99,
        }
    }

    /*
     * REFACTOR
     *
     * Check what potentials exists for other squares in box
     * If one potential is unique for this square it must be
     * set to value
     */
    pub fn engine_box(&mut self) -> AnyhowResult<bool> {
        let mut update: Option<(usize, usize)> = None;
        'outer: for square in &self.squares {
            let mut friends_potentials: Vec<usize> = Vec::new();
            let squares_in_box = self.get_abox(square.abox_id)?.get_square_ids();

            // Ivestigate first
            for square_id in squares_in_box {
                if square_id == square.id {
                    continue;
                }
                let friend_square = self.get_square(square_id)?;
                if let Some(potentials) = friend_square.get_potentials() {
                    for potential in potentials {
                        if !friends_potentials.contains(potential) {
                            friends_potentials.push(*potential)
                        }
                    }
                };
            }

            if let Some(potentials) = square.get_potentials() {
                for potential in potentials {
                    if !friends_potentials.contains(potential) {
                        update = Some((square.id, *potential));
                        break 'outer;
                    }
                }
            }

            // Update
        }

        if update.is_some() {
            let (id, value) = update.unwrap();
            self.set_square(id, value, SetKind::NORMAL)?;
            log::debug!("[engine] engine_box -> true");
            return Ok(true);
        }

        log::debug!("[engine] engine_box -> false");
        Ok(false)
    }

    /*
     * Update square given on what line,column,box
     *
     */
    fn _update_one_from(
        &mut self,
        container: Container,
        id: usize,
        value: usize,
    ) -> AnyhowResult<()> {
        let mut set_square_id: Option<usize> = None;
        let mut a: Vec<usize> = Vec::new();

        match container {
            Container::ABOX => {
                for square_id in self.abox.get(id).unwrap()._squares.iter() {
                    a.push(*square_id);
                    if self.squares[*square_id].value == 0 {
                        set_square_id = Some(*square_id);
                        break;
                    }
                }
            }
            Container::LINE => {
                for square_id in self.line.get(id).unwrap()._squares.iter() {
                    if self.squares[*square_id].value == 0 {
                        set_square_id = Some(*square_id);
                        break;
                    }
                }
            }
            Container::COLUMN => {
                for square_id in self.column.get(id).unwrap()._squares.iter() {
                    if self.squares[*square_id].value == 0 {
                        set_square_id = Some(*square_id);
                        break;
                    }
                }
            }
        }

        self.set_square(set_square_id.unwrap(), value, SetKind::NORMAL)?;
        Ok(())
    }

    /*
     * Set value if only one left on the line
     *
     */
    pub fn engine_line_one_left(&mut self) -> AnyhowResult<bool> {
        let mut updates: Vec<(Container, usize, usize)> = Vec::new();
        for line in self.line.iter_mut() {
            if line._remaining.len() == 1 {
                updates.push((Container::LINE, line._id, line._remaining.pop().unwrap()));
                log::debug!("[engine] engine_line_one_left -> true");
                break;
            } else {
                log::debug!("[engine] engine_line_one_left -> false");
            }
        }

        if !updates.is_empty() {
            if updates.len() > 1 {
                panic!("LINE update > 1");
            }
            for update in updates {
                self._update_one_from(update.0, update.1, update.2)?;
            }
            return Ok(true);
        }

        Ok(false)
    }

    /*
     * Set value if only one left on column
     *
     */
    pub fn engine_column_one_left(&mut self) -> AnyhowResult<bool> {
        let mut updates: Vec<(Container, usize, usize)> = Vec::new();
        for column in self.column.iter_mut() {
            if column._remaining.len() == 1 {
                updates.push((
                    Container::COLUMN,
                    column._id,
                    column._remaining.pop().unwrap(),
                ));
                log::debug!("[engine] engine_column_one_left -> true");
                break;
            } else {
                log::debug!("[engine] engine_column_one_left -> false");
            }
        }

        if !updates.is_empty() {
            for update in updates {
                self._update_one_from(update.0, update.1, update.2)?;
            }
            return Ok(true);
        }

        Ok(false)
    }

    /*
     * Set value if only 1 square left in box
     *
     */
    pub fn engine_box_one_left(&mut self) -> AnyhowResult<bool> {
        let mut updates: Vec<(Container, usize, usize)> = Vec::new();
        for abox in self.abox.iter_mut() {
            if abox._remaining.len() == 1 {
                updates.push((Container::ABOX, abox._id, abox._remaining.pop().unwrap()));
                log::debug!("[engine] engine_box_one_left -> true");
                break;
            } else {
                log::debug!("[engine] engine_box_one_left -> false");
            }
        }

        if !updates.is_empty() {
            for update in updates {
                self._update_one_from(update.0, update.1, update.2)?;
            }
            return Ok(true);
        }

        Ok(false)
    }

    /*
     * Set value if only one potential value exist for square
     *
     */
    pub fn engine_only_one_possible(&mut self) -> AnyhowResult<bool> {
        let mut update: Option<(usize, usize)> = None;
        for square in &mut self.squares {
            if let Some(potentials) = square.get_potentials() {
                if potentials.len() == 1 {
                    update = Some((square.id, potentials.clone().pop().unwrap()));
                }
            }
        }

        if let Some(update) = update {
            let (square_id, value) = update;
            self.set_square(square_id, value, SetKind::NORMAL)?;
            log::debug!("[engine] engine_only_one_possible -> true");
            return Ok(true);
        }

        log::debug!("[engine] engine_only_one_possible -> false");
        Ok(false)
    }

    /*
     * Get reference to ABox given id
     *
     */
    fn get_abox(&self, _id: usize) -> AnyhowResult<&ABox> {
        match self.abox.iter().filter(|x| x._id == _id).last() {
            Some(abox) => Ok(abox),
            None => Err(anyhow!("Unable find abox with id: {_id}")),
        }
    }

    /*
     * Get reference to square given id
     *
     */
    fn get_square(&self, _id: usize) -> AnyhowResult<&Square> {
        match self.squares.iter().filter(|x| x.id == _id).last() {
            Some(square) => Ok(square),
            None => Err(anyhow!("No square with id: {_id} found")),
        }
    }

    /*
     * Get mut reference to square given id
     *
     */
    fn get_square_mut(&mut self, _id: usize) -> AnyhowResult<&mut Square> {
        match self.squares.iter_mut().filter(|x| x.id == _id).last() {
            Some(square) => Ok(square),
            None => Err(anyhow!("No square with id: {_id} found")),
        }
    }

    /*
     * Update line,column,box potentials and potential for all squares
     *
     */
    fn update_square_potentials(&mut self) -> AnyhowResult<&mut Self> {
        for square in &mut self.squares {
            let ln = self.line.get(square.line_id).unwrap()._taken.clone();
            square.update("line_potentials", helpers::inverse_vec(&ln));

            let cn = self.column.get(square.column_id).unwrap()._taken.clone();
            square.update("column_potentials", helpers::inverse_vec(&cn));

            let bn = self.abox.get(square.abox_id).unwrap()._taken.clone();
            square.update("box_potentials", helpers::inverse_vec(&bn));

            if square.value == 0 {
                square.set_potentials(helpers::multi_intersections(vec![
                    square.box_potentials.clone(),
                    square.line_potentials.clone(),
                    square.column_potentials.clone(),
                ]));
            }
        }

        Ok(self)
    }

    /*
     * Consider the case where two squares are the only ones in
     * the box that have the potential for x,y. In this case any
     * other potentials can be removed.
     *
     *      id: 27,
     *      potentials: [ 6, 9 ],
     *      id: 28,
     *      potentials: [ 6, 8, 9 ],
     *      id: 29,
     *      potentials: [ 6, 8, 9 ],
     *      id: 45,
     *      potentials: [ 1, 2, 6, 9 ],
     *      id: 47,
     *      potentials: [ 1, 2, 6, 8, 9 ],
     *
     * In this case only 45 and 47 can hold 1 and 2.
     *
     *
     */
    fn update_box_remove_potentials(&mut self) -> AnyhowResult<()> {
        let mut identified: Vec<(Vec<usize>, Vec<usize>)> = Vec::new();

        for abox in &self.abox {
            let mut tmp: HashMap<usize, Vec<usize>> = HashMap::new();
            //                    ^       ^
            //                    |       + Vec of square ids that have this potential
            //                    + potential
            //
            // So we build up a HashMap with the potentials and what squares that can have these

            for square_id in abox.get_square_ids() {
                let square = self.get_square(square_id)?;
                for potential in &square.potentials {
                    match tmp.get_mut(potential) {
                        Some(vec) => {
                            vec.push(square.id);
                        }
                        None => {
                            tmp.insert(*potential, vec![square.id]);
                        }
                    }
                }
            }

            let mut hash_vec: Vec<(&usize, &Vec<usize>)> = tmp.iter().collect();
            hash_vec.sort_by(helpers::compare);

            let mut _2pairs: Vec<(&usize, &Vec<usize>)> = Vec::new();
            let mut _3pairs: Vec<(&usize, &Vec<usize>)> = Vec::new();
            let mut _4pairs: Vec<(&usize, &Vec<usize>)> = Vec::new();

            for (n, v) in hash_vec.iter() {
                match v.len() {
                    2 => _2pairs.push((*n, v)),
                    3 => _3pairs.push((*n, v)),
                    4 => _4pairs.push((*n, v)),
                    _ => (),
                }
            }

            if _2pairs.len() == 2 {
                let (num1, vec1) = _2pairs[0];
                let (num2, vec2) = _2pairs[1];
                if vec1.iter().all(|item| vec2.contains(item)) {
                    identified.push(
                        ([num1.to_owned(), num2.to_owned()].to_vec(), vec1.to_owned()).to_owned(),
                    );
                }
            }
            if _3pairs.len() == 3 {
                let (num1, vec1) = _3pairs[0];
                let (num2, vec2) = _3pairs[1];
                let (num3, vec3) = _3pairs[2];

                if vec1
                    .iter()
                    .all(|item| vec2.contains(item) && vec3.contains(item))
                {
                    identified.push(
                        (
                            [num1.to_owned(), num2.to_owned(), num3.to_owned()].to_vec(),
                            vec1.to_owned(),
                        )
                            .to_owned(),
                    );
                }
            }
            if _4pairs.len() >= 4 {
                let (num1, vec1) = _4pairs[0];
                let (num2, vec2) = _4pairs[1];
                let (num3, vec3) = _4pairs[2];
                let (num4, vec4) = _4pairs[3];

                if vec1
                    .iter()
                    .all(|item| vec2.contains(item) && vec3.contains(item) && vec4.contains(item))
                {
                    identified.push(
                        (
                            [
                                num1.to_owned(),
                                num2.to_owned(),
                                num3.to_owned(),
                                num4.to_owned(),
                            ]
                            .to_vec(),
                            vec1.to_owned(),
                        )
                            .to_owned(),
                    );
                }
            }
        }

        for (number, squares) in identified.iter() {
            match squares.len() {
                2 => {
                    let s1 = self.get_square_mut(squares[0])?;
                    s1.set_potentials(number.clone());

                    let s2 = self.get_square_mut(squares[1])?;
                    s2.set_potentials(number.clone());
                }
                3 => {
                    let s1 = self.get_square_mut(squares[0])?;
                    s1.set_potentials(number.clone());

                    let s2 = self.get_square_mut(squares[1])?;
                    s2.set_potentials(number.clone());

                    let s3 = self.get_square_mut(squares[2])?;
                    s3.set_potentials(number.clone());
                }
                4 => {
                    let s1 = self.get_square_mut(squares[0])?;
                    s1.set_potentials(number.clone());

                    let s2 = self.get_square_mut(squares[1])?;
                    s2.set_potentials(number.clone());

                    let s3 = self.get_square_mut(squares[2])?;
                    s3.set_potentials(number.clone());

                    let s4 = self.get_square_mut(squares[3])?;
                    s4.set_potentials(number.clone());
                }
                _ => (),
            }
        }
        Ok(())
    }

    /*
     * Update remaining and taken for each box
     *
     */
    pub fn update_abox(&mut self) -> AnyhowResult<&mut Self> {
        let mut abox_taken: HashMap<usize, Vec<usize>> = HashMap::new();

        for abox in &self.abox {
            let mut current_abox: Vec<usize> = Vec::new();

            // Populate all taken values for current box
            for square_id in abox.get_square_ids() {
                let square = self.get_square(square_id)?;
                if square.value != 0 {
                    current_abox.push(square.value)
                }
            }

            abox_taken.insert(abox._id, current_abox);
        }

        // Update all ABox
        for abox in &mut self.abox {
            let t = abox_taken.remove(&abox._id).unwrap();
            abox._remaining = helpers::inverse_vec(&t);
            abox._taken = t;
        }

        Ok(self)
    }

    /*
     * Update remaining and taken for each line
     *
     */
    pub fn update_line(&mut self) -> AnyhowResult<&mut Self> {
        let mut line_taken: HashMap<usize, Vec<usize>> = HashMap::new();

        for line in &self.line {
            let mut current_line: Vec<usize> = Vec::new();

            // Populate all taken values for current line
            for square_id in line.get_square_ids() {
                let square = self.get_square(square_id)?;
                if square.value != 0 {
                    current_line.push(square.value)
                }
            }

            line_taken.insert(line._id, current_line);
        }

        // Update all Lines
        for line in &mut self.line {
            let t = line_taken.remove(&line._id).unwrap();
            line._remaining = helpers::inverse_vec(&t);
            line._taken = t;
        }

        Ok(self)
    }

    /*
     * Update remaining and taken for each line
     *
     */
    pub fn update_column(&mut self) -> AnyhowResult<&mut Self> {
        let mut column_taken: HashMap<usize, Vec<usize>> = HashMap::new();

        for column in &self.column {
            let mut current_column: Vec<usize> = Vec::new();

            // Populate all taken values for current line
            for square_id in column.get_square_ids() {
                let square = self.get_square(square_id)?;
                if square.value != 0 {
                    current_column.push(square.value)
                }
            }

            column_taken.insert(column._id, current_column);
        }

        // Update all Columns
        for column in &mut self.column {
            let t = column_taken.remove(&column._id).unwrap();
            column._remaining = helpers::inverse_vec(&t);
            column._taken = t;
        }

        Ok(self)
    }

    /*
     * Validate Table
     *
     * This is done by:
     *  - line verification
     *  - column verification
     *  - box verification
     *
     */
    fn validate(&mut self) -> AnyhowResult<bool> {
        Ok(self._validate_line()? && self._validate_column()? && self._validate_box()?)
    }

    /*
     * Line verification
     *
     */
    fn _validate_line(&self) -> AnyhowResult<bool> {
        for line in &self.line {
            let mut test: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
            for square_id in line.get_square_ids() {
                let square = self.get_square(square_id)?;
                if square.value == 0 {
                    continue;
                }
                if !helpers::remove_element(square.value, &mut test) {
                    log::debug!(
                        "[validation] failed on line: {:?}, duplicate value: {:?}",
                        line._id,
                        square.value
                    );
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    /*
     * Column verification
     *
     */
    fn _validate_column(&self) -> AnyhowResult<bool> {
        for column in &self.column {
            let mut test: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
            for square_id in column.get_square_ids() {
                let square = self.get_square(square_id)?;
                if square.value == 0 {
                    continue;
                }
                if !helpers::remove_element(square.value, &mut test) {
                    log::debug!(
                        "[validation] failed on column: {:?}, duplicate value: {:?}",
                        column._id,
                        square.value
                    );
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    /*
     * Box verification
     *
     */
    fn _validate_box(&self) -> AnyhowResult<bool> {
        for abox in &self.abox {
            let mut test: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
            for square_id in abox.get_square_ids() {
                let square = self.get_square(square_id)?;
                if square.value == 0 {
                    continue;
                }
                if !helpers::remove_element(square.value, &mut test) {
                    log::debug!(
                        "[validation] failed on box: {:?}, duplicate value: {:?}",
                        abox._id,
                        square.value
                    );
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }
}
