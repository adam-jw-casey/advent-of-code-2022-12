use std::collections::HashSet;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Move{
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position{
    pub x: usize,
    pub y: usize
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
            x: height_str.lines().filter(|l| l.contains('E')).next().unwrap().find('E').unwrap(),
            y: height_str.lines().enumerate().filter(|(_, l)| l.contains('E')).map(|(i,_)| i).next().unwrap()
        };

        Surface{
            best_signal,
            heights
        }
    }

    pub fn width(&self) -> usize{
        self.heights[0].len()
    }

    pub fn height(&self) -> usize{
        self.heights.len()
    }
}

#[derive(Debug, PartialEq)]
pub struct Player{
    pub position: Position,
    pub previous: HashSet<Position>
}

impl Player{
    /// Creates a player with at the position marked by 'S'
    /// # Examples
    /// ```
    /// use advent_of_code_2022_12::Player;
    /// use advent_of_code_2022_12::Position;
    /// use std::collections::HashSet;
    ///
    /// assert_eq!(
    ///     Player{
    ///         position: Position{
    ///             x: 0,
    ///             y: 0
    ///         },
    ///         previous: HashSet::new()
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
                x: map_str.lines().filter(|l| l.contains('S')).next().unwrap().find('S').unwrap(),
                y: map_str.lines().enumerate().filter(|(_, l)| l.contains('S')).map(|(i,_)| i).next().unwrap()
            },
            previous: HashSet::new()
        }
    }

    pub fn step(&mut self, dir: Move){
        self.previous.insert(self.position);

        match dir{
            Move::Up   =>  self.position.y -= 1,
            Move::Down =>  self.position.y += 1,
            Move::Left =>  self.position.x -= 1,
            Move::Right => self.position.x += 1
        };
    }

    /// Returns a list of moves available to the player at its position
    /// # Examples
    /// ```
    /// use advent_of_code_2022_12::Player;
    /// use advent_of_code_2022_12::Surface;
    /// use advent_of_code_2022_12::Move;
    /// use std::collections::HashSet;
    ///
    /// let player = Player::new(concat!(
    ///     "Sabqponm\n",
    ///     "abcryxxl\n",
    ///     "accszExk\n",
    ///     "acctuvwj\n",
    ///     "abdefghi"
    /// ));
    /// let surface = Surface::new(concat!(
    ///     "Sabqponm\n",
    ///     "abcryxxl\n",
    ///     "accszExk\n",
    ///     "acctuvwj\n",
    ///     "abdefghi"
    /// ));
    ///
    /// assert_eq!(
    ///     player.available_moves(&surface),
    ///     HashSet::from([Move::Right, Move::Down])
    /// );
    /// ```
    pub fn available_moves(&self, surface: &Surface) -> HashSet<Move>{
        let mut moves = HashSet::new();

        if self.position.x < surface.width() - 1
            && surface.heights[self.position.x][self.position.y] as u8 + 1 >= surface.heights[self.position.x+1][self.position.y] as u8{
                moves.insert(Move::Right);
        }
        if self.position.x > 0
            && surface.heights[self.position.x][self.position.y] as u8 + 1 >= surface.heights[self.position.x-1][self.position.y] as u8{
                moves.insert(Move::Left);
        }
        if self.position.y < surface.height() - 1
            && surface.heights[self.position.x][self.position.y] as u8 + 1 >= surface.heights[self.position.x][self.position.y+1] as u8{
                moves.insert(Move::Down);
        }
        if self.position.y > 0
            && surface.heights[self.position.x][self.position.y] as u8 + 1 >= surface.heights[self.position.x][self.position.y-1] as u8{
                moves.insert(Move::Up);
        }

        return moves;
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
