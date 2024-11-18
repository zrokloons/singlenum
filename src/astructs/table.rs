use crate::astructs::abox::ABox;
use crate::astructs::column::Column;
use crate::astructs::line::Line;
use crate::astructs::square::Square;
use crate::utils::helpers;
use anyhow::anyhow;
use anyhow::Result as AnyhowResult;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub enum SetKind {
    NORMAL,
    GUESS,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SnapShots {
    snapshots: Vec<SnapShot>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SnapShot {
    square: Vec<Square>,
    line: Vec<Line>,
    column: Vec<Column>,
    abox: Vec<ABox>,
    value: usize,
    square_id: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Table {
    abox: Vec<ABox>,
    line: Vec<Line>,
    column: Vec<Column>,
    squares: Vec<Square>,
    snapshots: Option<SnapShots>,
    max_attempts: i32,
    iteration: i32,
    signatures: Vec<u64>,
    signatures_duplicates: usize,
    snapshot_rollbacks: usize,
    test: Vec<Vec<usize>>,
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

            match index {
                8 | 17 | 26 | 35 | 44 | 53 | 62 | 71 | 80 => l.push(Line {
                    _id: index / 9,
                    _taken: Vec::new(),
                    _remaining: Vec::new(),
                    _0: s[index - 8].id,
                    _1: s[index - 7].id,
                    _2: s[index - 6].id,
                    _3: s[index - 5].id,
                    _4: s[index - 4].id,
                    _5: s[index - 3].id,
                    _6: s[index - 2].id,
                    _7: s[index - 1].id,
                    _8: s[index].id,
                }),
                _ => (),
            }

            if let 72..=80 = index {
                c.push(Column {
                    _id: index % 9,
                    _taken: Vec::new(),
                    _remaining: Vec::new(),
                    _0: s[index - (9 * 8)].id,
                    _1: s[index - (9 * 7)].id,
                    _2: s[index - (9 * 6)].id,
                    _3: s[index - (9 * 5)].id,
                    _4: s[index - (9 * 4)].id,
                    _5: s[index - (9 * 3)].id,
                    _6: s[index - (9 * 2)].id,
                    _7: s[index - 9].id,
                    _8: s[index].id,
                })
            }

            match index {
                20 | 23 | 26 | 47 | 50 | 53 | 74 | 77 | 80 => a.push(ABox {
                    _id: a_id,
                    _taken: Vec::new(),
                    _remaining: Vec::new(),
                    _0: s[index - 9 - 9 - 2].id,
                    _1: s[index - 9 - 9 - 1].id,
                    _2: s[index - 9 - 9].id,
                    _3: s[index - 9 - 2].id,
                    _4: s[index - 9 - 1].id,
                    _5: s[index - 9].id,
                    _6: s[index - 2].id,
                    _7: s[index - 1].id,
                    _8: s[index].id,
                }),
                _ => (),
            }
        }

        Table {
            abox: a,
            line: l,
            column: c,
            squares: s,
            snapshots: None,
            max_attempts,
            iteration: 0,
            signatures: Vec::new(),
            test: Vec::new(),
            snapshot_rollbacks: 0,
            signatures_duplicates: 0,
        }
    }

    fn progress(&mut self) -> AnyhowResult<()> {
        self.iteration += 1;
        log::debug!("[iteration] {}", self.iteration);

        self.hasher()?;

        let mut tmp: Vec<usize> = Vec::new();

        let mut progress: i32 = 0;
        for square in &self.squares {
            if square.value != 0 {
                progress += 1;
            } else {
                tmp.push(square.id);
            }
        }

        /*
         * Puzzle finished
         */
        if progress == 81 {
            self.draw();
            if self.validate()? {
                println!(
                    "Success !! Puzzle finished in {} iterations",
                    self.iteration
                );
                std::process::exit(0);
            }
            println!(
                "Failure !! Unable to finish in {} iterations",
                self.iteration
            );
            std::process::exit(1);
        }

        /*
         * Max attempts reached
         */
        if self.iteration == self.max_attempts {
            println!(
                "{progress}/81 - Failed to solve puzzle {} Iterations",
                self.iteration
            );
            self.draw();

            for square in self.squares.iter() {
                if square.value == 0 {
                    log::debug!(
                        "ID: {:?}, potentials: {:?}, history: {:?}",
                        square.id,
                        square.potentials,
                        square.history
                    );
                }
            }

            std::process::exit(1);
        }
        Ok(())
    }

    /*
     * Hash the table
     *
     *
     */
    fn hasher(&mut self) -> AnyhowResult<()> {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        let a = s.finish();

        if self.signatures.contains(&a) {
            self.signatures_duplicates += 1;
        } else {
            self.signatures.push(a);
        }

        if self.signatures_duplicates > self.snapshot_rollbacks {
            panic!(
                "Signature duplicates: {:?}, Snapshot rollbacks: {:?}!",
                self.signatures_duplicates, self.snapshot_rollbacks
            );
        }

        Ok(())
    }

    /*
     * Solve
     */
    pub fn solve(&mut self) -> AnyhowResult<&mut Self> {
        self.draw();

        loop {
            self.progress()?;

            // Update line, column, box, and finally squares. Then run Engine to set squares
            self.update()?;
            if self.engine()? {
                continue;
            }

            // Guess, this is based on hard potentials, not a random guess!
            self.guess()?;
        }
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
        match kind {
            SetKind::NORMAL => self.squares[square_id].set_value(value),
            SetKind::GUESS => self.squares[square_id].set_value_guess(value),
        }

        // Update Line, Column and ABox
        self.line[self.squares[square_id].line_id].set_taken(value);
        self.column[self.squares[square_id].column_id].set_taken(value);
        self.abox[self.squares[square_id].abox_id].set_taken(value);

        Ok(self)
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
    fn snapshot_rollback(&mut self) -> AnyhowResult<&mut Self> {
        if let Some(snapshots) = self.snapshots.as_ref() {
            if snapshots.snapshots.is_empty() {
                return Ok(self);
            }
        }

        log::debug!("[snapshot] Roll back to latest snapshot!");
        let snapshot = self.snapshots.as_mut().unwrap().snapshots.pop().unwrap();

        self.squares = snapshot.square;
        self.line = snapshot.line;
        self.column = snapshot.column;
        self.abox = snapshot.abox;

        self.snapshot_rollbacks += 1;
        //self.draw();

        // We need to update the square used in snapshot to include the value
        // used in their history.
        let square = self.get_square_mut(snapshot.square_id)?;
        square.history.push(snapshot.value);
        Ok(self)
    }

    /*
     * Guess
     *
     * This means that we take one square that have few hard potentials
     * and set it to one of them, then we see how it goes ;)
     */
    fn guess(&mut self) -> AnyhowResult<&mut Self> {
        let mut taken_snapshot = SnapShot {
            square: self.squares.clone(),
            line: self.line.clone(),
            column: self.column.clone(),
            abox: self.abox.clone(),
            value: 99,
            square_id: 99,
        };
        let mut update: Option<(usize, usize)> = None;

        'outer: for square in self.squares.iter() {
            /*
             * Take Snapshot of everything
             *
             * If guess on square was successful we keep this
             * snapshot in our history.
             *
             */
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

            taken_snapshot.square_id = square_id;
            taken_snapshot.value = value;

            match &mut self.snapshots {
                Some(snapshots) => {
                    snapshots.snapshots.push(taken_snapshot);
                }
                None => {
                    self.snapshots = Some(SnapShots {
                        snapshots: [taken_snapshot].to_vec(),
                    })
                }
            }

            log::debug!("[snapshot] Taken");
        }

        Ok(self)
    }

    /*
     * The engine will run multiple different routines to conclude
     * if a square can set a value.
     *
     * As soon an update has been done we need to break to make sure
     * that we get an update on our data.
     */
    fn engine(&mut self) -> AnyhowResult<bool> {
        let mut updated: bool;

        updated = self._engine_line_one_left()?;
        if updated {
            let valid: bool = self.validate()?;
            if !valid {
                self.snapshot_rollback()?;
            }
            return Ok(true);
        }

        updated = self._engine_column_one_left()?;
        if updated {
            let valid: bool = self.validate()?;
            if !valid {
                self.snapshot_rollback()?;
            }
            return Ok(true);
        }

        updated = self._engine_box_one_left()?;
        if updated {
            let valid: bool = self.validate()?;
            if !valid {
                self.snapshot_rollback()?;
            }
            return Ok(true);
        }

        updated = self._engine_only_one_possible()?;
        if updated {
            let valid: bool = self.validate()?;
            if !valid {
                self.snapshot_rollback()?;
            }
            return Ok(true);
        }

        updated = self._engine_box()?;
        if updated {
            let valid: bool = self.validate()?;
            if !valid {
                self.snapshot_rollback()?;
            }
            return Ok(true);
        }

        updated = self._engine_box_remove_potentials()?;
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
    fn _engine_box_remove_potentials(&mut self) -> AnyhowResult<bool> {
        let mut identified: Vec<(Vec<usize>, Vec<usize>)> = Vec::new();

        for abox in &self.abox {
            let mut tmp: HashMap<usize, Vec<usize>> = HashMap::new();
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

        log::debug!("[engine] _engine_box_remove_potentials -> false");
        Ok(false)
    }

    /*
     * Set value if only 1 square left in box
     *
     */
    fn _engine_box_one_left(&mut self) -> AnyhowResult<bool> {
        let mut update: Option<(Vec<usize>, usize)> = None;

        for abox in self.abox.iter_mut() {
            if abox._remaining.len() == 1 {
                update = Some((
                    [
                        abox._0, abox._1, abox._2, abox._3, abox._4, abox._5, abox._6, abox._7,
                        abox._8,
                    ]
                    .to_vec(),
                    abox._remaining.pop().unwrap(),
                ));
                break;
            }
        }

        if let Some(update) = update {
            let (square_ids, value) = update;
            for square_id in square_ids {
                if self.squares[square_id].value == 0 {
                    self.set_square(square_id, value, SetKind::NORMAL)?;
                    log::debug!("[engine] _engine_box_one_left -> true");
                    return Ok(true);
                }
            }
        }

        log::debug!("[engine] _engine_box_one_left -> false");
        Ok(false)
    }

    fn _engine_box(&mut self) -> AnyhowResult<bool> {
        /*
         * REFACTOR
         *
         * Check what potentials exists for other squares in box
         * If one potential is unique for this square it must be
         * set to value
         */
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
            log::debug!("[engine] _engine_box -> true");
            return Ok(true);
        }

        log::debug!("[engine] _engine_box -> false");
        Ok(false)
    }

    /*
     * Set value if only one left on the line
     *
     *
     */
    fn _engine_line_one_left(&mut self) -> AnyhowResult<bool> {
        let mut update: Option<(usize, usize, usize)> = None; //start, end, value, usize)> = None;

        for line in self.line.iter_mut() {
            if line._remaining.len() == 1 {
                update = Some((line._0, line._8, line._remaining.pop().unwrap()));
                break;
            }
        }

        if let Some(update) = update {
            let (start, end, value) = update;
            for square_id in start..=end {
                if self.squares[square_id].value == 0 {
                    self.set_square(square_id, value, SetKind::NORMAL)?;
                    log::debug!("[engine] _engine_line_one_left -> true");
                    return Ok(true);
                }
            }
        }

        log::debug!("[engine] _engine_line_one_left -> false");
        Ok(false)
    }

    /*
     * Set value if only one left on column
     *
     *
     */
    fn _engine_column_one_left(&mut self) -> AnyhowResult<bool> {
        let mut update: Option<(Vec<usize>, usize)> = None; //start, end, value, usize)> = None;

        for column in self.column.iter_mut() {
            if column._remaining.len() == 1 {
                update = Some((
                    [
                        column._id,
                        column._id + 9,
                        column._id + 2 * 9,
                        column._id + 3 * 9,
                        column._id + 4 * 9,
                        column._id + 5 * 9,
                        column._id + 6 * 9,
                        column._id + 7 * 9,
                        column._id + 8 * 9,
                    ]
                    .to_vec(),
                    column._remaining.pop().unwrap(),
                ));
                break;
            }
        }

        if let Some(update) = update {
            let (square_ids, value) = update;
            for square_id in square_ids {
                if self.squares[square_id].value == 0 {
                    self.set_square(square_id, value, SetKind::NORMAL)?;
                    log::debug!("[engine] _engine_column_one_left -> true");
                    return Ok(true);
                }
            }
        }

        log::debug!("[engine] _engine_column_one_left -> false");
        Ok(false)
    }

    /*
     * Set value if only one potential value exist for square
     *
     */
    fn _engine_only_one_possible(&mut self) -> AnyhowResult<bool> {
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
            log::debug!("[engine] _engine_only_one_possible -> true");
            return Ok(true);
        }

        log::debug!("[engine] _engine_only_one_possible -> false");
        Ok(false)
    }

    fn __engine_only_one_possible(&mut self) -> AnyhowResult<bool> {
        for square in &mut self.squares {
            if match square.get_potentials() {
                Some(potentials) => potentials.len() == 1,
                None => false,
            } {
                let (id, value) = (
                    square.id,
                    square.get_potentials().unwrap().clone().pop().unwrap(),
                );
                self.set_square(id, value, SetKind::NORMAL)?;
                return Ok(true);
            }
        }

        Ok(false)
    }

    /*
     * Get reference to ABox given id
     */
    fn get_abox(&self, _id: usize) -> AnyhowResult<&ABox> {
        for abox in self.abox.iter() {
            if abox._id == _id {
                return Ok(abox);
            }
        }
        Err(anyhow!("Unable find abox with id: {_id}"))
    }

    /*
     * Get reference to square given id
     */
    fn get_square(&self, _id: usize) -> AnyhowResult<&Square> {
        for square in &self.squares {
            if square.id == _id {
                return Ok(square);
            }
        }
        Err(anyhow!("No square with id: {_id} found"))
    }

    /*
     * Get reference to square given id
     */
    fn get_square_mut(&mut self, _id: usize) -> AnyhowResult<&mut Square> {
        for square in &mut self.squares {
            if square.id == _id {
                return Ok(square);
            }
        }
        Err(anyhow!("No square with id: {_id} found"))
    }

    /*
     * Update square Line Column and ABox
     *
     * This update inspect all squares and update corrspoinding
     * values on each struct.
     *
     */
    fn update(&mut self) -> AnyhowResult<&mut Self> {
        self._update_line()?;
        self._update_column()?;
        self._update_abox()?;
        self._update_square_potentials()?;
        Ok(self)
    }

    fn _update_square_potentials(&mut self) -> AnyhowResult<&mut Self> {
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

    fn _update_abox(&mut self) -> AnyhowResult<&mut Self> {
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

    fn _update_line(&mut self) -> AnyhowResult<&mut Self> {
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

    fn _update_column(&mut self) -> AnyhowResult<&mut Self> {
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
     * Draw the Table
     */
    pub fn draw(&mut self) -> &mut Self {
        print!("╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗");
        let mut c = 0;
        for i in 0..81_usize {
            match i {
                27 | 54 => {
                    println!();
                    print!("╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣");
                }
                9 | 18 | 36 | 45 | 63 | 72 => {
                    println!();
                    print!("╟───┼───┼───╫───┼───┼───╫───┼───┼───╢");
                }
                _ => (),
            }
            if i % 9 == 0 {
                println!();
                print!("║"); // beginning
                c = 0;
            };

            let value = self.squares[i].value;
            match value {
                0 => print!("   "),
                _ => print!(" {value} "),
            }
            if c == 2 || c == 5 || c == 8 {
                print!("║");
            } else {
                print!("│");
            }
            c += 1;
        }
        println!();
        println!("╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝");
        self
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
        Ok(self._validate_line()? && self._validate_column()?)
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
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
     * ╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗
     * ║ 8 │ 5 │ 9 ║ 6 │ 1 │ 2 ║ 4 │ 3 │ 7 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║ 7 │ 2 │ 3 ║ 8 │ 5 │ 4 ║ 1 │ 6 │ 9 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║   │ 6 │ 4 ║ 3 │ 7 │ 9 ║ 5 │ 2 │ 8 ║
     * ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
     * ...
     */
    #[test]
    fn test_01_engine_box_one_left() {
        let configuration: Vec<usize> = [
            8, 5, 9, 6, 1, 2, 4, 3, 7, 7, 2, 3, 8, 5, 4, 1, 6, 9, 0, 6, 4, 3, 7, 9, 5, 2, 8, 9, 8,
            6, 1, 4, 7, 3, 5, 2, 3, 7, 5, 2, 6, 8, 9, 1, 4, 2, 4, 1, 5, 9, 3, 7, 8, 6, 4, 3, 2, 9,
            8, 1, 6, 7, 5, 6, 1, 7, 4, 2, 5, 8, 9, 3, 5, 9, 8, 7, 3, 6, 2, 4, 1,
        ]
        .to_vec();
        let mut table = Table::new(configuration, 1);
        table._update_abox().unwrap();

        let result = table._engine_box_one_left().unwrap();
        assert_eq!(true, result);
        assert_eq!(table.squares[18].value, 1_usize);
    }

    /*
     * ╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗
     * ║ 8 │ 5 │ 9 ║ 6 │ 1 │ 2 ║ 4 │ 3 │ 7 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║ 7 │ 2 │ 3 ║ 8 │ 5 │ 4 ║ 1 │ 6 │ 9 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║   │ 6 │ 4 ║ 3 │ 7 │ 9 ║ 5 │ 2 │ 8 ║
     * ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
     * ...
     *
     */
    #[test]
    fn test_02_engine_box_one_left() {
        let configuration: Vec<usize> = [
            8, 5, 9, 6, 1, 2, 4, 3, 7, 7, 2, 3, 8, 5, 4, 1, 6, 9, 1, 6, 4, 3, 7, 9, 5, 2, 8, 9, 8,
            6, 1, 4, 7, 3, 5, 2, 3, 7, 5, 2, 6, 8, 9, 1, 4, 2, 4, 1, 5, 9, 3, 7, 8, 6, 4, 3, 2, 9,
            8, 1, 6, 7, 5, 6, 1, 7, 4, 2, 5, 8, 9, 3, 5, 9, 8, 7, 3, 6, 2, 4, 1,
        ]
        .to_vec();
        let mut table = Table::new(configuration, 1);
        table._update_abox().unwrap();

        let result = table._engine_box_one_left().unwrap();
        assert_eq!(false, result);
    }

    /*
     * ╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗
     * ║   │ 5 │ 9 ║ 6 │ 1 │ 2 ║ 4 │ 3 │ 7 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║   │   │ 3 ║ 8 │ 5 │ 4 ║ 1 │ 6 │ 9 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║ 1 │ 6 │ 4 ║ 3 │ 7 │ 9 ║ 5 │ 2 │ 8 ║
     * ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
     * ...
     *
     */
    #[test]
    fn test_01_engine_line_one_left() {
        let configuration: Vec<usize> = [
            0, 5, 9, 6, 1, 2, 4, 3, 7, 0, 0, 3, 8, 5, 4, 1, 6, 9, 1, 6, 4, 3, 7, 9, 5, 2, 8, 9, 8,
            6, 1, 4, 7, 3, 5, 2, 3, 7, 5, 2, 6, 8, 9, 1, 4, 2, 4, 1, 5, 9, 3, 7, 8, 6, 4, 3, 2, 9,
            8, 1, 6, 7, 5, 6, 1, 7, 4, 2, 5, 8, 9, 3, 5, 9, 8, 7, 3, 6, 2, 4, 1,
        ]
        .to_vec();
        let mut table = Table::new(configuration, 1);
        table._update_line().unwrap();

        let result = table._engine_line_one_left().unwrap();
        assert_eq!(true, result);
        assert_eq!(table.squares[0].value, 8_usize);
    }

    /*
     * ╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗
     * ║   │   │ 9 ║ 6 │ 1 │ 2 ║ 4 │ 3 │ 7 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║ 7 │ 2 │ 3 ║ 8 │ 5 │ 4 ║ 1 │ 6 │ 9 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║ 1 │ 6 │ 4 ║ 3 │ 7 │ 9 ║ 5 │ 2 │ 8 ║
     * ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
     * ...
     *
     */
    #[test]
    fn test_02_engine_line_one_left() {
        let configuration: Vec<usize> = [
            0, 0, 9, 6, 1, 2, 4, 3, 7, 7, 2, 3, 8, 5, 4, 1, 6, 9, 1, 6, 4, 3, 7, 9, 5, 2, 8, 9, 8,
            6, 1, 4, 7, 3, 5, 2, 3, 7, 5, 2, 6, 8, 9, 1, 4, 2, 4, 1, 5, 9, 3, 7, 8, 6, 4, 3, 2, 9,
            8, 1, 6, 7, 5, 6, 1, 7, 4, 2, 5, 8, 9, 3, 5, 9, 8, 7, 3, 6, 2, 4, 1,
        ]
        .to_vec();
        let mut table = Table::new(configuration, 1);
        table._update_line().unwrap();

        let result = table._engine_line_one_left().unwrap();
        assert_eq!(false, result);
    }

    /*
     * ╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗
     * ║ 8 │ 5 │ 9 ║ 6 │ 1 │ 2 ║ 4 │ 3 │ 7 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║ 7 │ 2 │ 3 ║ 8 │ 5 │ 4 ║ 1 │ 6 │ 9 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║   │ 6 │ 4 ║ 3 │ 7 │ 9 ║ 5 │ 2 │ 8 ║
     * ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
     * ║ 9 │ 8 │ 6 ║ 1 │ 4 │ 7 ║ 3 │ 5 │ 2 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║ 3 │ 7 │ 5 ║ 2 │ 6 │ 8 ║ 9 │ 1 │ 4 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║ 2 │ 4 │ 1 ║ 5 │ 9 │ 3 ║ 7 │ 8 │ 6 ║
     * ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
     * ║ 4 │ 3 │ 2 ║ 9 │ 8 │ 1 ║ 6 │ 7 │ 5 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║ 6 │ 1 │ 7 ║ 4 │ 2 │ 5 ║ 8 │ 9 │ 3 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║ 5 │ 9 │ 8 ║ 7 │ 3 │ 6 ║ 2 │ 4 │ 1 ║
     * ╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝
     *
     */
    #[test]
    fn test_01_engine_column_one_left() {
        let configuration: Vec<usize> = [
            8, 5, 9, 6, 1, 2, 4, 3, 7, 7, 2, 3, 8, 5, 4, 1, 6, 9, 0, 6, 4, 3, 7, 9, 5, 2, 8, 9, 8,
            6, 1, 4, 7, 3, 5, 2, 3, 7, 5, 2, 6, 8, 9, 1, 4, 2, 4, 1, 5, 9, 3, 7, 8, 6, 4, 3, 2, 9,
            8, 1, 6, 7, 5, 6, 1, 7, 4, 2, 5, 8, 9, 3, 5, 9, 8, 7, 3, 6, 2, 4, 1,
        ]
        .to_vec();
        let mut table = Table::new(configuration, 1);
        table._update_column().unwrap();

        let result = table._engine_column_one_left().unwrap();
        assert_eq!(true, result);
        assert_eq!(table.squares[18].value, 1_usize);
    }

    /*
     * ╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗
     * ║   │ 5 │ 9 ║ 6 │ 1 │ 2 ║ 4 │ 3 │ 7 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║   │ 2 │ 3 ║ 8 │ 5 │ 4 ║ 1 │ 6 │ 9 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║   │ 6 │ 4 ║ 3 │ 7 │ 9 ║ 5 │ 2 │ 8 ║
     * ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
     * ║ 9 │ 8 │ 6 ║ 1 │ 4 │ 7 ║ 3 │ 5 │ 2 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║ 3 │ 7 │ 5 ║ 2 │ 6 │ 8 ║ 9 │ 1 │ 4 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║ 2 │ 4 │ 1 ║ 5 │ 9 │ 3 ║ 7 │ 8 │ 6 ║
     * ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
     * ║ 4 │ 3 │ 2 ║ 9 │ 8 │ 1 ║ 6 │ 7 │ 5 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║ 6 │ 1 │ 7 ║ 4 │ 2 │ 5 ║ 8 │ 9 │ 3 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║ 5 │ 9 │ 8 ║ 7 │ 3 │ 6 ║ 2 │ 4 │ 1 ║
     * ╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝
     *
     */
    #[test]
    fn test_02_engine_column_one_left() {
        let configuration: Vec<usize> = [
            0, 5, 9, 6, 1, 2, 4, 3, 7, 0, 2, 3, 8, 5, 4, 1, 6, 9, 0, 6, 4, 3, 7, 9, 5, 2, 8, 9, 8,
            6, 1, 4, 7, 3, 5, 2, 3, 7, 5, 2, 6, 8, 9, 1, 4, 2, 4, 1, 5, 9, 3, 7, 8, 6, 4, 3, 2, 9,
            8, 1, 6, 7, 5, 6, 1, 7, 4, 2, 5, 8, 9, 3, 5, 9, 8, 7, 3, 6, 2, 4, 1,
        ]
        .to_vec();
        let mut table = Table::new(configuration, 1);
        table._update_column().unwrap();

        let result = table._engine_column_one_left().unwrap();
        assert_eq!(false, result);
    }

    /*
     * ╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗
     * ║ 8 │ 5 │ 9 ║ 6 │ 1 │ 2 ║ 4 │ 3 │ 7 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║ 7 │ 2 │ 3 ║ 8 │ 5 │ 4 ║ 1 │ 6 │ 9 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║ 1 │ 6 │ 4 ║ 3 │ 7 │ 9 ║ 5 │ 2 │ 8 ║
     * ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
     * ║ 9 │ 8 │ 6 ║ 1 │ 4 │ 7 ║ 3 │ 5 │ 2 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║ 3 │ 7 │ 5 ║ 2 │   │ 8 ║ 9 │ 1 │ 4 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║ 2 │ 4 │ 1 ║ 5 │ 9 │ 3 ║ 7 │ 8 │ 6 ║
     * ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
     * ║ 4 │ 3 │ 2 ║ 9 │ 8 │ 1 ║ 6 │ 7 │ 5 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║ 6 │ 1 │ 7 ║ 4 │ 2 │ 5 ║ 8 │ 9 │ 3 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║ 5 │ 9 │ 8 ║ 7 │ 3 │ 6 ║ 2 │ 4 │ 1 ║
     * ╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝
     *
     */
    #[test]
    fn test_01_engine_only_one_possible() {
        let configuration: Vec<usize> = [
            8, 5, 9, 6, 1, 2, 4, 3, 7, 7, 2, 3, 8, 5, 4, 1, 6, 9, 1, 6, 4, 3, 7, 9, 5, 2, 8, 9, 8,
            6, 1, 4, 7, 3, 5, 2, 3, 7, 5, 2, 0, 8, 9, 1, 4, 2, 4, 1, 5, 9, 3, 7, 8, 6, 4, 3, 2, 9,
            8, 1, 6, 7, 5, 6, 1, 7, 4, 2, 5, 8, 9, 3, 5, 9, 8, 7, 3, 6, 2, 4, 1,
        ]
        .to_vec();
        let mut table = Table::new(configuration, 1);
        table.squares[40].potentials = [6].to_vec();

        let result = table._engine_only_one_possible().unwrap();
        assert_eq!(true, result);
        assert_eq!(table.squares[40].value, 6_usize);
    }

    /*
     * ╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗
     * ║ 8 │ 5 │ 9 ║ 6 │ 1 │ 2 ║ 4 │ 3 │ 7 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║ 7 │ 2 │ 3 ║ 8 │ 5 │ 4 ║ 1 │ 6 │ 9 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║ 1 │ 6 │ 4 ║ 3 │ 7 │ 9 ║ 5 │ 2 │ 8 ║
     * ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
     * ║ 9 │ 8 │ 6 ║ 1 │ 4 │ 7 ║ 3 │ 5 │ 2 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║ 3 │ 7 │ 5 ║   │   │ 8 ║ 9 │ 1 │ 4 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║ 2 │ 4 │ 1 ║ 5 │ 9 │ 3 ║ 7 │ 8 │ 6 ║
     * ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
     * ║ 4 │ 3 │ 2 ║ 9 │ 8 │ 1 ║ 6 │ 7 │ 5 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║ 6 │ 1 │ 7 ║ 4 │ 2 │ 5 ║ 8 │ 9 │ 3 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║ 5 │ 9 │ 8 ║ 7 │ 3 │ 6 ║ 2 │ 4 │ 1 ║
     * ╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝
     *
     */
    #[test]
    fn test_02_engine_only_one_possible() {
        let configuration: Vec<usize> = [
            8, 5, 9, 6, 1, 2, 4, 3, 7, 7, 2, 3, 8, 5, 4, 1, 6, 9, 1, 6, 4, 3, 7, 9, 5, 2, 8, 9, 8,
            6, 1, 4, 7, 3, 5, 2, 3, 7, 5, 0, 0, 8, 9, 1, 4, 2, 4, 1, 5, 9, 3, 7, 8, 6, 4, 3, 2, 9,
            8, 1, 6, 7, 5, 6, 1, 7, 4, 2, 5, 8, 9, 3, 5, 9, 8, 7, 3, 6, 2, 4, 1,
        ]
        .to_vec();
        let mut table = Table::new(configuration, 1);
        table.squares[39].potentials = [6, 2].to_vec();
        table.squares[40].potentials = [6, 2].to_vec();

        let result = table._engine_only_one_possible().unwrap();
        assert_eq!(false, result);
    }

    /*
     * ╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗
     * ║ 3 │   │   ║   │   │ 1 ║   │   │   ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║   │ 7 │ 1 ║ 9 │ 6 │   ║   │ 2 │ 4 ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║   │   │   ║ 5 │   │   ║   │   │ 1 ║
     * ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
     * ║   │ 2 │   ║ 8 │ 4 │   ║ 7 │   │   ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║   │   │   ║ 6 │   │ 9 ║   │   │   ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║   │   │ 5 ║   │ 1 │ 2 ║   │ 9 │   ║
     * ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
     * ║ 9 │   │   ║   │   │ 6 ║   │   │   ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║ 2 │ 6 │   ║   │ 9 │ 7 ║ 1 │ 5 │   ║
     * ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
     * ║   │   │   ║ 1 │   │   ║   │   │ 2 ║
     * ╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝
     *
     *  ID: 27 potentials: [6, 1]
     *  ID: 28 potentials: []
     *  ID: 29 potentials: [3, 6, 9]
     *  ID: 36 potentials: [8, 1, 4, 7]
     *  ID: 37 potentials: [1, 3, 8, 4]
     *  ID: 38 potentials: [3, 4, 7, 8]
     *  ID: 45 potentials: [8, 6, 4, 7]
     *  ID: 46 potentials: [8, 3, 4]
     *  ID: 47 potentials: []
     *
     *  In this case it's only square 29 that have a potential for number 9
     *
     */
    #[test]
    fn test_01__engine_box() {
        let configuration: Vec<usize> = [
            3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 7, 1, 9, 6, 0, 0, 2, 4, 0, 0, 0, 5, 0, 0, 0, 0, 1, 0, 2,
            0, 8, 4, 0, 7, 0, 0, 0, 0, 0, 6, 0, 9, 0, 0, 0, 0, 0, 5, 0, 1, 2, 0, 9, 0, 9, 0, 0, 0,
            0, 6, 0, 0, 0, 2, 6, 0, 0, 9, 7, 1, 5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 2,
        ]
        .to_vec();
        let mut table = Table::new(configuration, 1);

        table.update().unwrap();
        let result = table._engine_box().unwrap();
        assert_eq!(true, result);
        assert_eq!(table.squares[29].value, 9_usize);
    }
    // _engine_box_remove_potentials
}
