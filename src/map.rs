use smallvec::SmallVec;

use crate::rng::Rng;

const ROW_WIDTH: usize = 7;
const NUM_FLOORS: usize = 15;
const SHOP_CHANCE: f32 = 0.05;
const QUESTION_CHANCE: f32 = 0.22;
const ELITE_CHANCE: f32 = 0.08;
const REST_CHANCE: f32 = 0.12;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ActMap {
    rooms: [[Room; ROW_WIDTH]; NUM_FLOORS],
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Forward,
    Right,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default, Copy)]
enum RoomType {
    QuestionMark,
    Shop,
    Treasure,
    Rest,
    Monster,
    Elite,
    #[default]
    Unassigned,
}

impl ActMap {
    pub fn standard(rng: &mut Rng) -> Self {
        let mut res = Self {
            rooms: [[Room::default(); ROW_WIDTH]; NUM_FLOORS],
        };
        res.create(rng);
        res
    }
    fn create(&mut self, rng: &mut Rng) {
        self.create_paths(6, rng);
        self.assign_fixed_rows();
        self.assign_remaining_random(rng);
    }

    fn assign_fixed_rows(&mut self) {
        for i in 0..ROW_WIDTH {
            self.rooms[0][i].room_type = RoomType::Monster;
        }
        for i in 0..ROW_WIDTH {
            self.rooms[8][i].room_type = RoomType::Treasure;
        }
        for i in 0..ROW_WIDTH {
            self.rooms[NUM_FLOORS - 1][i].room_type = RoomType::Rest;
        }
    }

    fn assign_remaining_random(&mut self, rng: &mut Rng) {
        let count = self.room_count_for_buckets() as f32;
        let mut bucket = Vec::new();
        for _ in 0..((count * SHOP_CHANCE).round_ties_even() as usize) {
            bucket.push(RoomType::Shop);
        }
        for _ in 0..((count * QUESTION_CHANCE).round_ties_even() as usize) {
            bucket.push(RoomType::QuestionMark);
        }
        for _ in 0..((count * ELITE_CHANCE).round_ties_even() as usize) {
            bucket.push(RoomType::Elite);
        }
        for _ in 0..((count * REST_CHANCE).round_ties_even() as usize) {
            bucket.push(RoomType::Rest);
        }
        for _ in bucket.len()..self.unassigned_count() {
            bucket.push(RoomType::Monster);
        }
        rng.shuffle(&mut bucket);
        //No need to assign the first or last floors, they are already assigned
        for i in 1..NUM_FLOORS - 1 {
            for j in 0..ROW_WIDTH {
                if self.rooms[i][j].reachable {
                    self.assign_room(i, j, &mut bucket);
                }
            }
        }
        //Any remaining unassigned rooms are assigned to Monster as a fallback.
        for i in 1..NUM_FLOORS - 1 {
            for j in 0..ROW_WIDTH {
                if self.rooms[i][j].reachable && self.rooms[i][j].room_type == RoomType::Unassigned
                {
                    self.rooms[i][j].room_type = RoomType::Monster;
                }
            }
        }
    }

    fn assign_room(&mut self, row: usize, x: usize, bucket: &mut Vec<RoomType>) {
        if self.rooms[row][x].room_type != RoomType::Unassigned {
            return;
        }
        let mut parent_types: SmallVec<[RoomType; 3]> = SmallVec::new();
        let mut siblings: [bool; ROW_WIDTH] = [false; ROW_WIDTH];
        let parent_row = &self.rooms[row - 1];
        fn check_for_siblings(parent_x: usize, parent: &Room, siblings: &mut [bool; ROW_WIDTH]) {
            if parent.has_left_child {
                siblings[parent_x - 1] = true;
            }
            if parent.has_front_child {
                siblings[parent_x] = true;
            }
            if parent.has_right_child {
                siblings[parent_x + 1] = true;
            }
        }
        if let Some(parent) = parent_row.get(x - 1)
            && parent.has_right_child
        {
            parent_types.push(parent.room_type);
            check_for_siblings(x - 1, parent, &mut siblings);
        }
        if let Some(parent) = parent_row.get(x)
            && parent.has_front_child
        {
            parent_types.push(parent.room_type);
            check_for_siblings(x, parent, &mut siblings);
        }
        if let Some(parent) = parent_row.get(x + 1)
            && parent.has_left_child
        {
            parent_types.push(parent.room_type);
            check_for_siblings(x + 1, parent, &mut siblings);
        }
        for i in 0..bucket.len() {
            let t = bucket[i];
            if row < 6 && (t == RoomType::Elite || t == RoomType::Rest) {
                continue;
            }
            if row == NUM_FLOORS - 2 && t == RoomType::Rest {
                continue;
            }
            if (t == RoomType::Elite || t == RoomType::Rest || t == RoomType::Shop)
                && ((&parent_types).into_iter().any(|parent_t| *parent_t == t))
            {
                continue;
            }
            for j in 0..ROW_WIDTH {
                if siblings[j] && (t == self.rooms[row][j].room_type) {
                    continue;
                }
            }
            self.rooms[row][x].room_type = t;
            //This is inefficient, but it is needed to match the STS code.
            //I will consider improving this later.
            bucket.remove(i);
        }
    }

    //The code of STS counts the first row and treasure row for the purpose
    //of determining the number of events and elites even though they
    //are assigned fixed rooms.
    fn room_count_for_buckets(&self) -> usize {
        let mut count = 0;
        for i in 0..NUM_FLOORS - 1 {
            for j in 0..=ROW_WIDTH {
                if self.rooms[i][j].reachable {
                    count += 1;
                }
            }
        }
        count
    }

    fn unassigned_count(&self) -> usize {
        let mut count = 0;
        for i in 0..NUM_FLOORS - 1 {
            for j in 0..=ROW_WIDTH {
                if self.rooms[i][j].reachable && self.rooms[i][j].room_type == RoomType::Unassigned
                {
                    count += 1;
                }
            }
        }
        count
    }

    fn create_paths(&mut self, count: usize, rng: &mut Rng) {
        let starting_floor = rng.sample(ROW_WIDTH);
        self.create_path(starting_floor, rng);
        for i in 1..count {
            if i == 1 {
                //The first floor can't be chosen as the second one.
                let mut sample = rng.sample(ROW_WIDTH - 1);
                if sample >= starting_floor {
                    sample += 1;
                }
                self.create_path(sample, rng);
            } else {
                self.create_path(rng.sample(ROW_WIDTH), rng);
            }
        }
    }

    fn create_path(&mut self, starting_x: usize, rng: &mut Rng) {
        let mut x = starting_x;
        for i in 0..NUM_FLOORS - 1 {
            self.rooms[i][x].reachable = true;
            let mut direction = if x == 0 {
                [Direction::Forward, Direction::Right][rng.sample(2)]
            } else if x == ROW_WIDTH - 1 {
                [Direction::Left, Direction::Forward][rng.sample(2)]
            } else {
                [Direction::Left, Direction::Forward, Direction::Right][rng.sample(3)]
            };
            //Don't let paths cross.
            if direction == Direction::Left && self.rooms[i][x - 1].has_right_child {
                direction = Direction::Forward;
            }
            if direction == Direction::Right && self.rooms[i][x + 1].has_left_child {
                direction = Direction::Forward;
            }
            match direction {
                Direction::Left => {
                    self.rooms[i][x].has_left_child = true;
                    x = x - 1;
                }
                Direction::Forward => {
                    self.rooms[i][x].has_front_child = true;
                }
                Direction::Right => {
                    self.rooms[i][x].has_right_child = true;
                    x = x + 1;
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default, Copy)]
struct Room {
    has_left_child: bool,
    has_front_child: bool,
    has_right_child: bool,
    room_type: RoomType,
    reachable: bool,
}
