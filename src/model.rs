#[derive(Debug)]
pub enum ColType {
    Ladder(usize), // usize will determine where the ladder will take the player
    Snake(usize),  // usize will determine where the snake will take the player
}

#[derive(Debug)]
pub struct Col {
    pub col_id: usize,
    pub col_type: Option<ColType>,
}

#[derive(Debug)]
pub struct Board {
    pub cols: Vec<Col>,
}

impl Board {
    pub fn new() -> Board {
        let mut vc = Vec::new();
        for i in 1..=100 {
            vc.insert(
                0,
                Col {
                    col_id: i,
                    col_type: None,
                },
            )
        }
        Board { cols: vc }
    }

    pub fn get_col_data(&self, col_id: usize) {
        println!("{:#?}", self.cols[col_id]);
    }

    pub fn set_col_prop(&mut self, col_id: usize, ct: ColType) {
        // self.cols[col_id - 1].col_type = Some(ct);
        println!(
            "The col {} is pointing to {}",
            col_id,
            match &ct {
                ColType::Snake(v) => v,
                ColType::Ladder(v) => v,
            }
        );
        self.cols[col_id].col_type = Some(ct);
    }
}

#[derive(Debug)]
pub struct Player {
    id: usize,
    name: String,
}

impl Player {
    pub fn new(i: usize, n: String) -> Self {
        Player { id: i, name: n }
    }
    pub fn set_name(&mut self, n: &'static str) {
        self.name = n.to_string();
    }
}
