pub enum Move{
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, PartialEq)]
pub struct Position{
    pub x: u32,
    pub y: u32
}

#[derive(Debug, PartialEq)]
pub struct Surface{
    pub best_signal: Position,
    pub heights: Vec<Vec<char>>
}

impl Surface{
    /// Creates a surface with the heights and highest point in the passed string
    /// # Examples
    /// ```
    /// use advent_of_code_2022_12::Surface;
    /// use advent_of_code_2022_12::Position;
    ///
    /// assert_eq!(
    ///     Surface{
    ///         best_signal: Position{x: 5, y: 2},
    ///         heights: vec![
    ///             "aabqponm".chars().collect::<Vec<_>>(),
    ///             "abcryxxl".chars().collect::<Vec<_>>(),
    ///             "accszzxk".chars().collect::<Vec<_>>(),
    ///             "acctuvwj".chars().collect::<Vec<_>>(),
    ///             "abdefghi".chars().collect::<Vec<_>>()
    ///         ]
    ///     },
    ///     Surface::new(concat!(
    ///         "Sabqponm\n",
    ///         "abcryxxl\n",
    ///         "accszExk\n",
    ///         "acctuvwj\n",
    ///         "abdefghi"
    /// )));
    /// ```
    pub fn new(height_str: &str) -> Self{
        let heights = height_str
            .lines()
            .map(|line| {
                line
                    .chars()
                    .map(|character| {
                        if character.is_ascii_lowercase(){
                            character
                        }else if character == 'S'{
                            'a'
                        }else if character == 'E'{
                            'z'
                        }else{
                            panic!("Invalid character {character}");
                        }
                    })
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>();

        let best_signal = Position{
            x: height_str.lines().filter(|l| l.contains('E')).next().unwrap().find('E').unwrap() as u32,
            y: height_str.lines().enumerate().filter(|(_, l)| l.contains('E')).map(|(i,_)| i).next().unwrap() as u32
        };

        Surface{
            best_signal,
            heights
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Player{
    pub position: Position
}

impl Player{
    /// Creates a player with at the position marked by 'S'
    /// # Examples
    /// ```
    /// use advent_of_code_2022_12::Player;
    /// use advent_of_code_2022_12::Position;
    ///
    /// assert_eq!(
    ///     Player{
    ///         position: Position{
    ///             x: 0,
    ///             y: 0
    ///         }
    ///     },
    ///     Player::new(concat!(
    ///         "Sabqponm\n",
    ///         "abcryxxl\n",
    ///         "accszExk\n",
    ///         "acctuvwj\n",
    ///         "abdefghi"
    /// )));
    /// ```
    pub fn new(map_str: &str) -> Self{
        Player{
            position: Position{
                x: map_str.lines().filter(|l| l.contains('S')).next().unwrap().find('S').unwrap() as u32,
                y: map_str.lines().enumerate().filter(|(_, l)| l.contains('S')).map(|(i,_)| i).next().unwrap() as u32
            }
        }
    }

    pub fn step(&mut self, dir: Move){
        match dir{
            Move::Up   =>  self.position.y += 1,
            Move::Down =>  self.position.y -= 1,
            Move::Left =>  self.position.x -= 1,
            Move::Right => self.position.x += 1
        }
    }

    pub fn available_moves(&self, surface: &Surface) -> Vec<Move>{
        todo!();
    }
}

/// Finds the shortest path to the top and returns its length
/// # Examples
/// ```
/// use advent_of_code_2022_12::shortest_path;
///
/// assert_eq!(
///     31,
///     shortest_path(concat!(
///         "Sabqponm\n",
///         "abcryxxl\n",
///         "accszExk\n",
///         "acctuvwj\n",
///         "abdefghi"
/// )));
/// ```
pub fn shortest_path(input_commands: &str) -> u32{
    todo!();
}
