use std::io;

fn main(){
    println!("Тыкни enter, чтобы начать...");
    io::stdin().read_line(&mut String::new()).expect("Failed to read line");

    let _ = crate::game_field::start_game();
}

mod game_field {
    use std::{io::{self, Stdout, Write}, time::Duration};

    use crossterm::{ExecutableCommand, cursor::{Hide, MoveTo, Show}, event::{ Event, KeyCode, poll, read}, execute, style::Print, terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode}};

    const SIZE: i8 = 32+2;
    use std::{collections::LinkedList};
    pub struct Snake {
        pub body: LinkedList<[i8; 2]>,
        pub direction: char,
    }
    impl Snake {
        pub fn new() -> Snake {
            Snake { body: (LinkedList::new()), direction: ('r') }
        }
        pub fn check_direction_available(&self, d: char) -> bool {
            let head_coords = self.get_head_coords();
            let new_coords = make_new_coords(head_coords, d);
            if !self.body.contains(&new_coords) {
                return true;
            }
            return false
        }
        pub fn set_direction(&mut self, d: char) {
            if self.check_direction_available(d) {
                self.direction = d;
            }
        }
        pub fn add_head(&mut self) -> () {
            let _ = &self.body.push_front([1, 1]);
        }
        pub fn add_tail(&mut self) -> () {
            let mut tail = self.body.back();
            
            let mut new_tail;
            match tail {
                Some(coords) => {
                    new_tail = [coords[0], coords[1]];
                },
                None => panic!(),
            }
            let before_tail = tail.iter_mut().next_back();
            match before_tail {
                Some(node) => {
                    let diff = [new_tail[0] - node[0], new_tail[1] - new_tail[1]];
                    new_tail = [new_tail[0] + diff[0], new_tail[1] + diff[1]];
                }
                None => panic!()
            }
            let _ = &self.body.push_back(new_tail);
        }
        pub fn make_step(&mut self) -> bool {
            let head = self.body.front();
            let mut next_coords: [i8; 2];
            match head {
                Some(node) => {
                    next_coords = make_new_coords(*node, self.direction);
                }
                None => {panic!()}
            }            
            if self.body.contains(&next_coords) {
                return false;
            }
            for part in self.body.iter_mut() {
                let curr_coords = *part;
                *part = next_coords;
                next_coords = curr_coords;
            }
            return true
        }
        pub fn get_snake_len(&self) -> usize {
            return self.body.len();
        }
        pub fn check_snake(&self, i: i8, j: i8) -> bool{
            let temp = [i, j];
            let temp_bool = self.body.contains(&temp);
            return temp_bool;
        }
        pub fn get_head_coords(&self) -> [i8; 2] {
            let coords;
            let head = self.body.front();
            match head {
                Some(node) => {
                    coords = *node;
                }
                None => panic!()
            }
            return coords;
        }
    }

    pub fn check_eating_food(all_food: &mut FoodObj, snake: &Snake) -> bool {
        if all_food.contains(snake.get_head_coords()) {
            return true;
        } 
        else {return false}
    }
    pub fn make_new_coords(mut coords: [i8; 2], dir: char) -> [i8; 2]{
        match dir {
            'r' => {
                coords = [coords[0], coords[1] + 1];
                if coords[1] > 32 {
                    coords[1] = 1
                }
            },
            'l' => {
                coords = [coords[0], coords[1] - 1];
                if coords[1] < 1 {
                    coords[1] = 32
                }
            },
            't' => {
                coords = [coords[0] - 1, coords[1]];
                if coords[0] < 1 {
                    coords[0] = 32
                }
            },
            'd' => {
                coords = [coords[0] + 1, coords[1]];
                if coords[0] > 32 {
                    coords[0] = 1
                }

            },
            _ => println!("Invalid Direction")
        }
        return coords
    }
    pub struct FoodObj {
        pub food_coords: Vec<[i8; 2]>

    }
    
    impl FoodObj {
        pub fn new() -> FoodObj {
            FoodObj { food_coords: (Vec::new())}
        }
        pub fn add_food(&mut self) {
            let coord_row = rand::random_range(1..33);
            let coord_col = rand::random_range(1..33);
            self.food_coords.push([coord_row, coord_col]);
        }
        pub fn contains(&self, coords: [i8; 2]) -> bool {
            if self.food_coords.contains(&coords) {
                return true
            }
            return false 
        }
        pub fn remove(&mut self, cds: [i8; 2]){
            self.food_coords.retain(|&coords| coords != cds);
        }

    }

    pub fn check_border(i: i8, j: i8) -> i8{
        if i == 0 || i == SIZE-1 {
            return 1;
        } else if j == 0 || j == SIZE-1 {
            return 2;
        } else {return 0}
    }

    pub fn draw(snake: &Snake, stdout: &mut Stdout, all_food: &mut FoodObj) -> Result<(), Box<dyn std::error::Error>> {
        let _ = write!(stdout, "\rSnake length: {:?}\n", snake.get_snake_len());
        for i in 0..SIZE  {
            let mut line: String = "".to_string();
            for j in 0..SIZE {
                let res = check_border(i, j);
                if res == 0{
                    if snake.check_snake(i, j) {
                        let snake_head = snake.body.front();
                        match snake_head {
                            Some(node) => {
                                if *node == [i, j] {
                                    match snake.direction {
                                        't' => line += "[^]",
                                        'l' => line += "[<]",
                                        'd' => line += "[v]",
                                        'r' => line += "[>]",
                                        _ => panic!()
                                    }
                                }
                                else {
                                    line += "[◍]"
                                }
                            }
                            None => panic!()
                        }
                    } else {
                        if all_food.contains([i, j]) {
                            line += "<*>"
                        } else {
                            line += "   ";
                        }
                            
                        }
                } else {
                    match res {
                        1 => {
                            line += "---"
                        }
                        2 => {
                            line += "| |"
                        }
                        _ => {panic!()}
                    }
                }
                
            }
            execute!(
                stdout,
                Print("\r\n".to_string() + &line.clone())
            )?;
            let _ = stdout.flush();
        }
        return Ok(());
        
    }
    pub fn start_game() -> std::io::Result<()> {
        let mut stdout = io::stdout();
        let mut snake = Snake::new();
        let mut all_food: FoodObj = FoodObj::new();
        enable_raw_mode()?;
        stdout.execute(Clear(ClearType::All))?;
        stdout.execute(Hide)?;
        snake.add_head();
        snake.add_tail();
        loop {
            
            if poll(Duration::from_millis(400))? {
                match read()? {
                    Event::Key(event) => {
                        match event.code {
                            KeyCode::Char('w') | KeyCode::Char('ц') => {
                                snake.set_direction('t');
                            }
                            KeyCode::Char('a') | KeyCode::Char('ф') => {
                                snake.set_direction('l');
                            }
                            KeyCode::Char('s') | KeyCode::Char('ы') => {
                                snake.set_direction('d');
                            }
                            KeyCode::Char('d') | KeyCode::Char('в') => {
                                snake.set_direction('r');
                            }
                            KeyCode::Char('e') | KeyCode::Char('у') => {
                                snake.add_tail();
                            }
                            KeyCode::Esc => {
                                break;
                            }
                            _ => {
                                continue;
                            }
                        }
                    }
                    Event::FocusGained => todo!(),
                    Event::FocusLost => todo!(),
                    Event::Mouse(_) => todo!(),
                    Event::Paste(_) => todo!(),
                    Event::Resize(_, _) => todo!(),
                }
            }
            let food_spawn = rand::random_range(0..9);
            if food_spawn == 1 {
                all_food.add_food();
            }


            if !snake.make_step() {
                break
            }
            if check_eating_food(&mut all_food, &mut snake) {
                snake.add_tail();
                all_food.remove(snake.get_head_coords());
            }   
            stdout.execute(MoveTo(0, 0))?;
            let _ = draw(&snake, &mut stdout, &mut all_food);
            
        }
        stdout.execute(Clear(ClearType::All))?;
        stdout.execute(Show)?;
        disable_raw_mode()?;
        Ok(())
    }
}