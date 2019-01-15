
extern crate rand;

#[derive(Debug)]
struct Stone {
    typ: String,
    typ_num: u8,
    obj: Vec<bool>,
    dim_x: u8,
    dim_y: u8,
    score: u8,
}

impl Clone for Stone {
    fn clone(&self) -> Stone {
        Stone {
            typ: self.typ.clone(),
            typ_num: self.typ_num,
            obj: self.obj.clone(),
            dim_x: self.dim_x,
            dim_y: self.dim_y,
            score: self.score,
        }
    }    
}

impl Stone {
    fn new(typ: String, typ_num: u8, dim_x: u8, dim_y: u8, score: u8, s: &str) -> Stone {
        let mut st: Stone = Stone {
            typ,
            typ_num,
            dim_x,
            dim_y,
            obj: Vec::new(),
            score,
        };
        for c in s.chars() {
            st.obj.push(c != '.');
        }

        st
    }

    fn _to_string(&self) -> String {
        let mut s = String::new();
        let o = &(self.obj);
        let mut nl = String::new();
        for y in 0..self.dim_y {
            s = s + nl.as_str();
            for x in 0..self.dim_x {
                if *o.get((x + y * self.dim_x) as usize).unwrap() {
                    s = s + "x";
                } else {
                    s = s + " ";
                }
            }
            nl = "\n".to_string();
        }
        s
    }

    fn rotate(&self) -> Stone {
        let mut st = Stone {
            typ: self.typ.clone(),
            typ_num: self.typ_num,
            dim_x: self.dim_y,
            dim_y: self.dim_x,
            obj: self.obj.clone(),
            score: self.score,
        };

        let mut i: usize = 0;
        for x in 0..st.dim_x {
            for y in (0..st.dim_y).rev() {
                let b = self.obj[i].clone();
                i += 1;
                st.obj[(x + y * st.dim_x) as usize] = b;
                //println!("{}->{:?},{:?}", i, x, y);
            }
        }

        st
    }
}

struct Board {
    dim_x: u8,
    dim_y: u8,
    cells: Vec<u8>,
}

impl Clone for Board {
    fn clone(&self) -> Board {
        Board {
            dim_x: self.dim_x,
            dim_y: self.dim_y,
            cells: self.cells.clone(),
        }
    }
}

impl Board {
    ///
    /// create empty board
    ///
    fn new(dim_x: u8, dim_y: u8) -> Board {
        let mut b = Board {
            dim_x,
            dim_y,
            cells: Vec::new(),
        };
        for _i in 0..(dim_x as usize * dim_y as usize) {
            b.cells.push(0);
        }
        b
    }

    ///
    /// check if there is a collision
    ///

    fn valid(&self, xp: u8, yp: u8, stone: &Stone) -> bool {

        let mut i = 0;

        for y in yp as usize..(yp + stone.dim_y) as usize {
            for x in xp as usize..(xp + stone.dim_x) as usize {

                let p = x + y * self.dim_x as usize;
                if p >= self.cells.len() {
                    return false;
                }

                let cell_stone = stone.obj[i];
                i += 1;

                if !cell_stone {
                    continue;
                }

                let cell_board = self.cells[p];
                if cell_board != 0 {
                    return false;
                }
            }
        }

        true
    }

    ///
    /// put a stone finally on the board
    ///

    fn put(&mut self, xp: u8, yp: u8, stone: &Stone) {
        let xp = xp as usize;
        let yp = yp as usize;
        let mut i = 0;
        for y in yp..(yp + stone.dim_y as usize) {
            for x in xp..(xp + stone.dim_x as usize) {
                let cell_stone = stone.obj[i];
                i += 1;
                if cell_stone {
                    let p = x + y * self.dim_x as usize;
                    self.cells[p] = stone.typ_num;
                }
            }
        }
    }

    ///
    /// eliminate
    ///

    fn eliminate(&mut self) -> bool {
        let mut done_something = false;

        let mut y = (self.dim_y - 1) as usize;

        loop {
            let mut can_eliminate = true;
            let mut cell = 0;
            for x in 0..self.dim_x as usize {
                cell = self.cells[x + y * self.dim_x as usize];
                can_eliminate &= cell != 0;
                if !can_eliminate {
                    break;
                }
            }
            if can_eliminate && cell == 9 {
                self.remove_line(y);
                done_something = true;
                continue;
            }
            if can_eliminate && cell < 9 {
                for x in 0..self.dim_x as usize {
                    let p = x + y * self.dim_x as usize;
                    self.cells[p] = 9;
                }
                done_something = true;
            }
            if y == 0 {
                break;
            }
            y -= 1;
        }

        done_something
    }

    fn remove_line(&mut self, y: usize) {
        let p = y * self.dim_x as usize;
        for x in 0..self.dim_x as usize {
            self.cells.remove(p);
        }
        for x in 0..self.dim_x as usize {
            self.cells.insert(0,0);
        }
    }

    ///
    /// produces a string
    ///

    fn to_string(&self) -> String {
        let mut s = self.state_string();

        for i in (1..=self.dim_y).rev() {            
            let p = i * self.dim_x;
            s.insert_str(p as usize, "\n");
        }

        s.replace("0", ".")
    }

    ///
    /// get the state as string
    ///

    fn state_string(&self) -> String {
        let mut s = String::new();
        for y in 0..self.dim_y as usize {
            for x in 0..self.dim_x as usize {
                let p = x + y * (self.dim_x as usize);
                let num = self.cells[p];

                let nstr = format!("{}", num);
                s.push_str(nstr.as_str());
                
            }
        }
        s
    }
}

pub struct Game {
    board: Board,
    stone: usize,
    xp: u8,
    yp: u8,
    rot: u8,
    next_stone: usize,
    pub score: u64,
    stone_no: usize,
    stones: Vec<Stone>,
}


impl Game {
    fn rand_stone(no: usize, rand: f64 ) -> usize {
        /*let y = rand::random::<f64>();*/
        (rand * no as f64) as usize
    }

    fn init() ->  Vec<Stone> {

        let i_stone = Stone::new("i".to_string(), 1, 4, 1, 40, "xxxx");
        let j_stone = Stone::new("j".to_string(), 2, 3, 2, 30, "xxx..x");
        let l_stone = Stone::new("l".to_string(), 3, 3, 2, 30, "xxxx..");
        let o_stone = Stone::new("o".to_string(), 4, 2, 2, 10, "xxxx");
        let s_stone = Stone::new("s".to_string(), 5, 3, 2, 20, ".xxxx.");
        let t_stone = Stone::new("t".to_string(), 6, 3, 2, 30, "xxx.x.");
        let z_stone = Stone::new("z".to_string(), 7, 3, 2, 20, "xx..xx");

        vec! [
            i_stone, j_stone, l_stone, o_stone, s_stone, t_stone, z_stone,
        ]

    }

    pub fn new( r1: f64, r2: f64 ) -> Game {

        let stns = Self::init();
        let stone_no = stns.len();

        let mut stones = Vec::new();
        for i in 0..stone_no {
            let st = &stns[i];
            stones.push(st.clone());
            for _ in 0..3 {
                // rotate always the last one
                let mut strot = stones[stones.len() - 1].rotate();
                stones.push(strot);
            }
        }

        let dim_x = 10;
        let dim_y = 24;

        Game {
            board: Board::new(dim_x, dim_y),
            stone: Self::rand_stone(stone_no, r1 ),
            next_stone: Self::rand_stone(stone_no, r2 ),
            rot: 0,
            score: 0,
            stone_no,
            stones,
            xp: Self::init_x(dim_x),
            yp: 0,
        }
    }

    fn init_x(x: u8) -> u8 {
        (x / 2)
    }

    ///
    /// step_down indicates if stone should move one down
    ///
    /// rotate <> 0 -> rotates to left
    ///
    /// returns game over as bool
    ///

    pub fn next_move(&mut self, step_down: bool, _move_x: i8, rotate: u8, rand: f64 ) -> bool {
        
        self.eliminate();

        let total_rotate = (self.rot + rotate) % 4;

        let current = self.stone * 4 + total_rotate as usize;

        let mut xp = self.xp as i8 + _move_x;
        if xp < 0 {
            xp = 0;
        }
        let mut xp = xp as u8;

        if xp + self.stones[current].dim_x > self.board.dim_x {
            xp = self.board.dim_x - self.stones[current].dim_x;
        }
        let yp = self.yp + if step_down { 1 } else { 0 };

        if self.board.valid(xp as u8, yp as u8, &self.stones[current]) {
            self.rot = total_rotate as u8;
            self.xp = xp;
            self.yp = yp;

            return false;
        }

        // put stone at this position
        self.board.put(self.xp, self.yp, &self.stones[ self.stone * 4 + self.rot as usize ]);

        // add new score
        self.score = self.score + self.stones[current].score as u64;

        // set next stone and create new one
        self.stone = self.next_stone;
        self.next_stone = Self::rand_stone(self.stone_no, rand );
        self.rot = 0;

        self.xp = Self::init_x(self.board.dim_x);
        self.yp = 0;

        // if next move not possile then game over
        !self.board
            .valid(self.xp as u8, self.yp as u8, &self.stones[self.stone * 4])
    }

    fn eliminate(&mut self) -> bool {
        let eli = self.board.eliminate();

        if eli {
            self.score += 100;
        }

        eli
    }

    fn current_board(&self) -> Board {
        let mut board = self.board.clone();
        board.put(
            self.xp,
            self.yp,
            &self.stones[self.stone * 4 + self.rot as usize],
        );
        board
    }

    pub fn board_state(&self) -> String {
        let board = self.current_board();
        board.state_string()
    }

    pub fn next_stone(&self) -> String {
        let z : &'static str = "0123456789";
        let mut s = String::from("");
        let stn = &self.stones[self.next_stone * 4 ];
        for i in 0..stn.obj.len() as usize {
            let cell = stn.obj[i];
            s.push( if cell { z.as_bytes()[stn.typ_num as usize] as char } else { '0' } );
        }
        format!("{}{}{}", stn.dim_x, stn.dim_y, s )
    }

}


