use std::collections::HashSet;
use std::cmp::min;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Clone)]
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

    pub fn step(&mut self, dir: &Move){
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
    ///
    /// let mut player = Player::new(concat!(
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
    ///     player.available_moves(&surface, &true),
    ///     Vec::from([Move::Down, Move::Right])
    /// );
    /// player.step(&Move::Down);
    /// assert_eq!(
    ///     player.available_moves(&surface, &true),
    ///     Vec::from([Move::Right, Move::Up, Move::Down])
    /// );
    /// player.step(&Move::Right);
    /// assert_eq!(
    ///     player.available_moves(&surface, &true),
    ///     Vec::from([Move::Down, Move::Right, Move::Up, Move::Left])
    /// );
    /// ```
    pub fn available_moves(&self, surface: &Surface, up: &bool) -> Vec<Move>{
        let mut moves = Vec::new();
        let this_height  = surface.heights[self.position.y][self.position.x];

        let height_check = |next_height: char| match up{
            true  => this_height as u8 + 1 >= next_height as u8,
            false => this_height as u8 - 1 <= next_height as u8
        };

        if self.position.x < surface.width() - 1{
            let right_height = surface.heights[self.position.y][self.position.x+1];
            if height_check(right_height){
                moves.push((Move::Right, right_height));
            }
        }
        if self.position.x > 0{
            let left_height  = surface.heights[self.position.y][self.position.x-1];
            if height_check(left_height){
                moves.push((Move::Left, left_height));
            }
        }
        if self.position.y < surface.height() - 1{
            let down_height  = surface.heights[self.position.y+1][self.position.x];
            if height_check(down_height){
                moves.push((Move::Down, down_height));
            }
        }
        if self.position.y > 0{
            let up_height    = surface.heights[self.position.y-1][self.position.x];
            if height_check(up_height){
                moves.push((Move::Up, up_height));
            }
        }

        moves.sort_unstable_by_key(|(_m,h)| *h as u8);
        return moves.iter().rev().map(|(m,_h)| *m).collect();
    }

    // Returns the length of the shortest path to the top of surface
    fn shortest_path(&self, surface: &Surface, max_depth: &usize, distmap: &mut Vec<Vec<usize>>, end_condition: &fn(&Position, &Surface) -> bool, up: &bool) -> usize{
        let mut new_max = *max_depth;
        let mut paths = Vec::new();

        if end_condition(&self.position, surface){
            return self.previous.len()
        }else if self.previous.contains(&self.position){
            return usize::MAX;
        }else if self.previous.len() >= new_max - 1{
            return usize::MAX;
        }else if distmap[self.position.y][self.position.x] <= self.previous.len(){
            return usize::MAX;
        }else{
            distmap[self.position.y][self.position.x] = self.previous.len();

            for m in self.available_moves(surface, &up).iter(){
                let mut new_player = (*self).clone();
                new_player.step(m);
                paths.push(new_player.shortest_path(surface, &new_max, distmap, &end_condition, up));
                new_max = min(*paths.last().unwrap(), new_max);
            }

            return *paths.iter().min().unwrap();
        }
    }

    pub fn find_shortest_path_up(&self, surface: &Surface) -> usize{
        fn at_top(position: &Position, surface: &Surface) -> bool {*position == surface.best_signal}

        self.shortest_path(
            surface,
            &usize::MAX,
            &mut (0..surface.height())
                .map(|_| (0..surface.width())
                    .map(|_| usize::MAX)
                    .collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            &(at_top as fn(&Position, &Surface)->bool),
            &true
        )
    }

    pub fn find_shortest_path_down(&self, surface: &Surface) -> usize{
        fn at_bottom(position: &Position, surface: &Surface) -> bool{surface.heights[position.y][position.x] == 'a'}
        self.shortest_path(
            surface,
            &usize::MAX,
            &mut (0..surface.height())
                .map(|_| (0..surface.width())
                    .map(|_| usize::MAX)
                    .collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            &(at_bottom as fn(&Position, &Surface)->bool),
            &false
        )
    }
}

/// Finds the shortest path to the top and returns its length
/// # Examples
/// ```
/// use advent_of_code_2022_12::shortest_path_up;
///
/// assert_eq!(
///     31,
///     shortest_path_up(concat!(
///         "Sabqponm\n",
///         "abcryxxl\n",
///         "accszExk\n",
///         "acctuvwj\n",
///         "abdefghi"
/// )));
/// ```
pub fn shortest_path_up(input: &str) -> usize{
    let player = Player::new(input);
    let surface = Surface::new(input);

    return player.find_shortest_path_up(&surface);
}

/// Finds the shortest path to the bottom and returns its length
/// # Examples
/// ```
/// use advent_of_code_2022_12::shortest_path_down;
///
/// assert_eq!(
///     29,
///     shortest_path_down(concat!(
///         "Sabqponm\n",
///         "abcryxxl\n",
///         "accszExk\n",
///         "acctuvwj\n",
///         "abdefghi"
/// )));
/// ```
pub fn shortest_path_down(input: &str) -> usize{
    let surface = Surface::new(input);
    let player = Player{
        position : surface.best_signal,
        previous : HashSet::new()
    };

    return player.find_shortest_path_down(&surface);
}
