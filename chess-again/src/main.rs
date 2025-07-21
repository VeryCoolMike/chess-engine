use eframe::egui;
use egui::Color32;

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 640.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Chess (but better)",
        options, 
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<Chess>::default())
        }),
    )
}

enum Direction {
    North,
    NorthEast,
    West,
    SouthWest,
    South,
    SouthEast,
    East,
    NorthWest
}

#[derive(std::marker::Copy, Clone, PartialEq)]
enum Team {
    Black,
    White,
    None
}

#[derive(std::marker::Copy, Clone, PartialEq)]
enum PieceType {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn,
    None
}

#[derive(std::marker::Copy, Clone)]
struct Piece {
    piece_type: PieceType,
    team: Team,
    moved: bool,
}

fn create_piece(piece_type: PieceType, team: Team) -> Piece {
    Piece {
        piece_type: piece_type,
        team,
        moved: false
    }
}

fn create_board() -> [[Piece; 8]; 8] {
    let empty_piece = create_piece(PieceType::None, Team::None);
    
    let mut temp_board = [[empty_piece; 8]; 8];
    temp_board[0][0] = create_piece(PieceType::Rook, Team::White);
    temp_board[1][0] = create_piece(PieceType::Knight, Team::White);
    temp_board[2][0] = create_piece(PieceType::Bishop, Team::White);
    temp_board[3][0] = create_piece(PieceType::Queen, Team::White);
    temp_board[4][0] = create_piece(PieceType::King, Team::White);
    temp_board[5][0] = create_piece(PieceType::Bishop, Team::White);
    temp_board[6][0] = create_piece(PieceType::Knight, Team::White);
    temp_board[7][0] = create_piece(PieceType::Rook, Team::White);
    for x in 0..=7 {
        temp_board[x][1] = create_piece(PieceType::Pawn, Team::White);
    }

    temp_board[0][7] = create_piece(PieceType::Rook, Team::Black);
    temp_board[1][7] = create_piece(PieceType::Knight, Team::Black);
    temp_board[2][7] = create_piece(PieceType::Bishop, Team::Black);
    temp_board[3][7] = create_piece(PieceType::Queen, Team::Black);
    temp_board[4][7] = create_piece(PieceType::King, Team::Black);
    temp_board[5][7] = create_piece(PieceType::Bishop, Team::Black);
    temp_board[6][7] = create_piece(PieceType::Knight, Team::Black);
    temp_board[7][7] = create_piece(PieceType::Rook, Team::Black);
    for x in 0..=7 {
        temp_board[x][6] = create_piece(PieceType::Pawn, Team::Black);
    }

    temp_board
}

fn check_in_board(pos: [isize; 2]) -> bool {
    if pos[0] < 0 || pos[0] > 7 {
        return false;
    }

    if pos[1] < 0 || pos[1] > 7 {
        return false;
    }

    return true;
}

fn move_list(pos: [isize; 2], piece: Piece, board: &[[Piece; 8]; 8], direction: Direction, length: isize, can_capture: bool) -> Vec<[isize; 2]> {
    let mut temp_vector: Vec<[isize; 2]> = vec![];
    let mut offset: [isize; 2] = [0, 0];
    match direction {
        Direction::North => {
            offset = [0, 1];           
        },
        Direction::East => {
            offset = [1, 0];
        },
        Direction::South => {
            offset = [0, -1];
        },
        Direction::West => {
            offset = [-1, 0];
        },

        Direction::NorthEast => {
            offset = [1, 1];
        },
        Direction::SouthEast => {
            offset = [1, -1];
        },
        Direction::NorthWest => {
            offset = [-1, 1];
        },
        Direction::SouthWest => {
            offset = [-1, -1];
        }
    }

    for i in 1..=length {
        let new_pos = [pos[0] + offset[0] * i, pos[1] + offset[1] * i];
        if check_in_board(new_pos) {
            let new_team = board[new_pos[0] as usize][new_pos[1] as usize].team;

            if new_team == piece.team { // Same team
                break; 
            }
            else if new_team == Team::None { // No team
                temp_vector.push(new_pos);
            }
            else {
                if can_capture == true {
                    temp_vector.push(new_pos); // Other team
                }
                break; 
            }
        }
    }

    temp_vector
}

fn attack_list(pos: [isize; 2], piece: Piece, board: &[[Piece; 8]; 8], new_pos: Vec<[isize; 2]>, enemy_only: bool) -> Vec<[isize; 2]> {
    let mut temp_vector: Vec<[isize; 2]> = vec![];
    for element in new_pos {
        let new_element = [pos[0] + element[0], pos[1] + element[1]];
        if check_in_board(new_element) {
            let enemy_team = board[new_element[0] as usize][new_element[1] as usize].team;
            if enemy_only == false {
                if enemy_team != piece.team {
                    temp_vector.push(new_element);
                }
            } else {
                if enemy_team != piece.team && enemy_team != Team::None {
                    temp_vector.push(new_element);
                } 
            }
        }
    }

    temp_vector
}

fn final_move_list(piece: Piece, pos: [isize; 2], board: &[[Piece; 8]; 8], filter_check: bool) -> Vec<[isize; 2]> {
    let mut temp_vector: Vec<[isize; 2]> = vec![];

    match (piece.team, piece.piece_type) {
        (_, PieceType::Bishop) => {
            let north_east = move_list(pos, piece, board, Direction::NorthEast, 8, true);
            let south_east = move_list(pos, piece, board, Direction::SouthEast, 8, true);
            let south_west = move_list(pos, piece, board, Direction::SouthWest, 8, true);
            let north_west = move_list(pos, piece, board, Direction::NorthWest, 8, true);

            temp_vector.extend(&north_east);
            temp_vector.extend(&south_east);
            temp_vector.extend(&south_west);
            temp_vector.extend(&north_west);
        },
        (_, PieceType::Rook) => {
            let north = move_list(pos, piece, board, Direction::North, 8, true);
            let east = move_list(pos, piece, board, Direction::East, 8, true);
            let south = move_list(pos, piece, board, Direction::South, 8, true);
            let west = move_list(pos, piece, board, Direction::West, 8, true);

            temp_vector.extend(&north);
            temp_vector.extend(&east);
            temp_vector.extend(&south);
            temp_vector.extend(&west);
        },
        (_, PieceType::Queen) => {
            let north = move_list(pos, piece, board, Direction::North, 8, true);
            let east = move_list(pos, piece, board, Direction::East, 8, true);
            let south = move_list(pos, piece, board, Direction::South, 8, true);
            let west = move_list(pos, piece, board, Direction::West, 8, true);

            temp_vector.extend(&north);
            temp_vector.extend(&east);
            temp_vector.extend(&south);
            temp_vector.extend(&west);

            let north_east = move_list(pos, piece, board, Direction::NorthEast, 8, true);
            let south_east = move_list(pos, piece, board, Direction::SouthEast, 8, true);
            let south_west = move_list(pos, piece, board, Direction::SouthWest, 8, true);
            let north_west = move_list(pos, piece, board, Direction::NorthWest, 8, true);

            temp_vector.extend(&north_east);
            temp_vector.extend(&south_east);
            temp_vector.extend(&south_west);
            temp_vector.extend(&north_west);
        },
        (_, PieceType::King) => {
            let attack_vec = vec![
                [0, 1],
                [1, 1],
                [1, 0],
                [1, -1],
                [0, -1],
                [-1, -1],
                [-1, 0],
                [-1, 1]];
            
            let attacks = attack_list(pos, piece, board, attack_vec, false);
            temp_vector.extend(&attacks);
        },
        (_, PieceType::Knight) => {
            let attack_vec = vec![
                [1, 2],
                [2, 1],
                [2, -1],
                [1, -2],
                [-1, 2],
                [-2, 1],
                [-2, -1],
                [-1, -2]];

            let attacks = attack_list(pos, piece, board, attack_vec, false);
            temp_vector.extend(&attacks);
        },
        (Team::Black, PieceType::Pawn) => {
            let move_length = if piece.moved == true {
                1
            } else {
                2
            };

            let south = move_list(pos, piece, board, Direction::South, move_length, false);

            let attacks = attack_list(pos, piece, board, vec![[-1, -1], [1, -1]], true);

            temp_vector.extend(&south);
            temp_vector.extend(&attacks);
        },
        (Team::White, PieceType::Pawn) => {
            let move_length = if piece.moved == true {
                1
            } else {
                2
            };
            let north = move_list(pos, piece, board, Direction::North, move_length, false);
            let attacks = attack_list(pos, piece, board, vec![[-1, 1], [1, 1]], true);

            temp_vector.extend(&north);
            temp_vector.extend(&attacks);
        },
        (_, _) => {}
    }
    
    if filter_check == true {
        temp_vector.retain(|new_pos| {
            let mut temp_board = board.clone();

            temp_board[pos[0] as usize][pos[1] as usize].piece_type = PieceType::None;
            temp_board[pos[0] as usize][pos[1] as usize].team = Team::None;

            temp_board[new_pos[0] as usize][new_pos[1] as usize] = piece;

            !is_in_check(piece.team, &temp_board) 
        });
    }

    temp_vector
}

fn is_in_check(team: Team, board: &[[Piece; 8]; 8]) -> bool {
    for y in (0..=7).rev() {
        for x in 0..=7 {
            let piece = board[x as usize][y as usize];
            if piece.team != team && piece.piece_type != PieceType::None { // On other team
                let moves = final_move_list(board[x as usize][y as usize], [x, y], board, false);
                for available_moves in moves {
                    if board[available_moves[0] as usize][available_moves[1] as usize].piece_type == PieceType::King && board[available_moves[0] as usize][available_moves[1] as usize].team == team {
                        return true;
                    }
                }
            }
        }
    }

    return false;
}

fn is_stalemate(team: Team, board: &[[Piece; 8]; 8]) -> bool {
    let mut found_piece = false;
    for y in (0..=7).rev() {
        for x in 0..=7 {
            let piece = board[x as usize][y as usize];
            if piece.team == team && piece.piece_type != PieceType::None {
                found_piece = true;
                let moves = final_move_list(board[x as usize][y as usize], [x, y], board, true);
                if moves.is_empty() == false {
                    return false;
                }
            }
        }
    }

    if found_piece == false {
        return false;
    }

    return true;
}

fn calculate_advantage(board: &[[Piece; 8]; 8]) -> (i32, i32) {
    let mut white_team = 0;
    let mut black_team = 0;

    for y in (0..=7).rev() {
        for x in 0..=7 {
            let piece = board[x][y];
            if piece.team == Team::White {
                match piece.piece_type {
                    PieceType::Pawn => { white_team += 1; },
                    PieceType::Knight => { white_team += 3; },
                    PieceType::Bishop => { white_team += 3; },
                    PieceType::Rook => { white_team += 5; },
                    PieceType::Queen => { white_team += 9; },
                    _ => {}
                }
            } else {
                match piece.piece_type {
                    PieceType::Pawn => { black_team += 1; },
                    PieceType::Knight => { black_team += 3; },
                    PieceType::Bishop => { black_team += 3; },
                    PieceType::Rook => { black_team += 5; },
                    PieceType::Queen => { black_team += 9; },
                    _ => {}
                }
            }
        }
    }

    return (white_team, black_team);
}

fn castle(can_castle: bool, x: i32, y: i32, board: &mut [[Piece; 8]; 8], selected_piece_0: i32, selected_piece_1: i32, current_team: &mut Team, history: &mut Vec<[[Piece; 8]; 8]>) {
    let selected_piece = [selected_piece_0, selected_piece_1];
    if can_castle && [x,y] == [6,0] || [x,y] == [2,0] || [x,y] == [6,7] || [x,y] == [2,7] {
        let current_piece = board[selected_piece[0] as usize][selected_piece[1] as usize];
        if current_piece.piece_type == PieceType::King && current_piece.team == *current_team {
            if current_piece.team == Team::White {
                if [x, y] == [6, 0] {
                    if board[(selected_piece[0]+1) as usize][selected_piece[1] as usize].piece_type == PieceType::None                                                            && board[(selected_piece[0]+2) as usize][selected_piece[1] as usize].piece_type == PieceType::None {
                        history.insert(0, *board);
                        board[x as usize][y as usize] = board[selected_piece[0] as usize][selected_piece[1] as usize];
                        board[x as usize][y as usize].moved = true;
                        board[selected_piece[0] as usize][selected_piece[1] as usize].team = Team::None;
                        board[selected_piece[0] as usize][selected_piece[1] as usize].piece_type = PieceType::None;
                        board[7][0].team = Team::None;
                        board[7][0].piece_type = PieceType::None;
                        board[5][0].team = Team::White;
                        board[5][0].piece_type = PieceType::Rook;

                        switch_teams(current_team);
                    }
                } else if [x, y] == [2, 0] {
                    if board[(selected_piece[0]-1) as usize][selected_piece[1] as usize].piece_type == PieceType::None                                                            && board[(selected_piece[0]-2) as usize][selected_piece[1] as usize].piece_type == PieceType::None
                       && board[(selected_piece[0]-3) as usize][selected_piece[1] as usize].piece_type == PieceType::None {
                        history.insert(0, *board);
                        board[x as usize][y as usize] = board[selected_piece[0] as usize][selected_piece[1] as usize];
                        board[x as usize][y as usize].moved = true;
                        board[selected_piece[0] as usize][selected_piece[1] as usize].team = Team::None;
                        board[selected_piece[0] as usize][selected_piece[1] as usize].piece_type = PieceType::None;
                        board[0][0].team = Team::None;
                        board[0][0].piece_type = PieceType::None;
                        board[3][0].team = Team::White;
                        board[3][0].piece_type = PieceType::Rook;

                        switch_teams(current_team);
                    }
                }
            } else {
                if [x, y] == [6, 7] {
                    if board[(selected_piece[0]+1) as usize][selected_piece[1] as usize].piece_type == PieceType::None                                                            && board[(selected_piece[0]+2) as usize][selected_piece[1] as usize].piece_type == PieceType::None {
                        history.insert(0, *board);
                        board[x as usize][y as usize] = board[selected_piece[0] as usize][selected_piece[1] as usize];
                        board[x as usize][y as usize].moved = true;
                        board[selected_piece[0] as usize][selected_piece[1] as usize].team = Team::None;
                        board[selected_piece[0] as usize][selected_piece[1] as usize].piece_type = PieceType::None;
                        board[7][7].team = Team::None;
                        board[7][7].piece_type = PieceType::None;
                        board[5][7].team = Team::Black;
                        board[5][7].piece_type = PieceType::Rook;

                        switch_teams(current_team);
                    }
                } else if [x, y] == [2, 7] {
                    if board[(selected_piece[0]-1) as usize][selected_piece[1] as usize].piece_type == PieceType::None                                                            && board[(selected_piece[0]-2) as usize][selected_piece[1] as usize].piece_type == PieceType::None
                       && board[(selected_piece[0]-3) as usize][selected_piece[1] as usize].piece_type == PieceType::None {
                        history.insert(0, *board);
                        board[x as usize][y as usize] = board[selected_piece[0] as usize][selected_piece[1] as usize];
                        board[x as usize][y as usize].moved = true;
                        board[selected_piece[0] as usize][selected_piece[1] as usize].team = Team::None;
                        board[selected_piece[0] as usize][selected_piece[1] as usize].piece_type = PieceType::None;
                        board[0][7].team = Team::None;
                        board[0][7].piece_type = PieceType::None;
                        board[3][7].team = Team::Black;
                        board[3][7].piece_type = PieceType::Rook;

                        switch_teams(current_team);
                    }
                }
            }
        }
    }
}

fn en_passant(x: i32, y: i32, board: &mut [[Piece; 8]; 8], selected_piece_0: i32, selected_piece_1: i32, current_team: &mut Team, history: &mut Vec<[[Piece; 8]; 8]>) {
    let selected_piece = [selected_piece_0, selected_piece_1];
    if board[selected_piece[0] as usize][selected_piece[1] as usize].piece_type == PieceType::Pawn {
        if y == 2 { // White en passant
            if board[x as usize][(y+1) as usize].piece_type == PieceType::Pawn &&
            board[x as usize][(y+1) as usize].team == Team::White {
                if let Some(i) = history.first() {
                    if i[x as usize][(y-1) as usize].piece_type == PieceType::Pawn &&
                    i[x as usize][(y-1) as usize].team == Team::White &&
                    i[x as usize][(y-1) as usize].moved == false {
                        history.insert(0, *board);
                        board[x as usize][y as usize] = board[selected_piece[0] as usize][selected_piece[1] as usize];
                        board[x as usize][(y+1) as usize].piece_type = PieceType::None;
                        board[x as usize][(y+1) as usize].team = Team::None;
                        board[selected_piece[0] as usize][selected_piece[1] as usize].piece_type = PieceType::None;
                        board[selected_piece[0] as usize][selected_piece[1] as usize].team = Team::None;
                        
                        switch_teams(current_team);
                    }
                }
            }
        } else if y == 5 { // Black en passant
            if board[x as usize][(y-1) as usize].piece_type == PieceType::Pawn &&
            board[x as usize][(y-1) as usize].team == Team::Black {
                if let Some(i) = history.first() {
                    if i[x as usize][(y+1) as usize].piece_type == PieceType::Pawn &&
                    i[x as usize][(y+1) as usize].team == Team::Black &&
                    i[x as usize][(y+1) as usize].moved == false {
                        history.insert(0, *board);
                        board[x as usize][y as usize] = board[selected_piece[0] as usize][selected_piece[1] as usize];
                        board[x as usize][(y-1) as usize].piece_type = PieceType::None;
                        board[x as usize][(y-1) as usize].team = Team::None;
                        board[selected_piece[0] as usize][selected_piece[1] as usize].piece_type = PieceType::None;
                        board[selected_piece[0] as usize][selected_piece[1] as usize].team = Team::None;

                        switch_teams(current_team);
                    }
                }
            }
        }
    }
}

fn switch_teams(team: &mut Team) {
    *team = match *team {
        Team::Black => Team::White,
        Team::White => Team::Black,
        _ => Team::White
    }
}

struct Chess {
    board: [[Piece; 8]; 8],
    history: Vec<[[Piece; 8]; 8]>,
    valid_moves: Vec<[isize; 2]>,
    can_castle: bool,
    can_en_passant: bool,
    current_team: Team,
    piece_selected: bool,
    selected_piece: [isize; 2],
    advantage: (i32, i32),
    selecting_promotion: bool
}

impl Default for Chess {
    fn default() -> Self {
        Self {
            board: create_board(),
            history: vec![],
            valid_moves: vec![],
            can_castle: true,
            can_en_passant: true,
            current_team: Team::White,
            piece_selected: false,
            selected_piece: [0, 0],
            advantage: (0, 0),
            selecting_promotion: false
        }
    }
}

impl eframe::App for Chess {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            match self.current_team {
                Team::Black => { ui.label(egui::RichText::new("Current Team: Black").color(egui::Color32::from_rgb(255, 255, 255)).size(30.0)); }
                Team::White => { ui.label(egui::RichText::new("Current Team: White").color(egui::Color32::from_rgb(255, 255, 255)).size(30.0)); }
                _ => {
                    self.current_team = Team::White;
                }
            }

            if self.advantage.1 > self.advantage.0 {
                ui.label(egui::RichText::new(format!("+{}", self.advantage.1 - self.advantage.0)).color(egui::Color32::from_rgb(255, 255, 255)).size(25.0));
            }

            egui::Grid::new("grid")
                .min_col_width(64.0)
                .min_row_height(64.0)
                .show(ui, |ui| {
                for y in (0..=7).rev() {
                    for x in 0..=7 {
                        let mut bg_colour = if [x, y] == self.selected_piece && self.piece_selected == true {
                            Color32::from_rgb(0, 0, 255)
                        } else {
                            if (x+y) % 2 == 0 {
                                Color32::from_rgb(117, 149, 85)
                            } else {
                                Color32::from_rgb(237, 237, 209)
                            }
                        };
                        if self.valid_moves.contains(&[x, y]) && self.piece_selected == true {
                            bg_colour = Color32::from_rgb(255, 0, 0);
                        }
                        let frame = egui::Frame::new()
                            .fill(bg_colour)
                            .inner_margin(egui::Margin::ZERO);
                        
                        frame.show(ui, |ui| {
                            let piece = self.board[x as usize][y as usize];
                            let image = match (piece.team, piece.piece_type) {
                                (Team::Black, PieceType::Rook) => egui::include_image!("../assets/black_rook.png"),
                                (Team::Black, PieceType::Knight) => egui::include_image!("../assets/black_knight.png"),
                                (Team::Black, PieceType::Bishop) => egui::include_image!("../assets/black_bishop.png"),
                                (Team::Black, PieceType::Queen) => egui::include_image!("../assets/black_queen.png"),
                                (Team::Black, PieceType::King) => egui::include_image!("../assets/black_king.png"),
                                (Team::Black, PieceType::Pawn) => egui::include_image!("../assets/black_pawn.png"),
                                (Team::White, PieceType::Rook) => egui::include_image!("../assets/white_rook.png"),
                                (Team::White, PieceType::Knight) => egui::include_image!("../assets/white_knight.png"),
                                (Team::White, PieceType::Bishop) => egui::include_image!("../assets/white_bishop.png"),
                                (Team::White, PieceType::Queen) => egui::include_image!("../assets/white_queen.png"),
                                (Team::White, PieceType::King) => egui::include_image!("../assets/white_king.png"),
                                (Team::White, PieceType::Pawn) => egui::include_image!("../assets/white_pawn.png"),
                                _ => egui::include_image!("../assets/blank.png"),
                            };

                            let response = ui.add(
                                egui::ImageButton::new(image)
                                    .frame(false)
                            );

                            if response.clicked() {
                                if self.piece_selected == false {
                                    if self.board[x as usize][y as usize].team == self.current_team { 
                                        self.selected_piece = [x as isize, y as isize];
                                        self.piece_selected = true;
                                        self.valid_moves = final_move_list(self.board[x as usize][y as usize], [x, y], &self.board, true);
                                        // Check for castling
                                        
                                        self.can_castle = false;
                                        self.can_en_passant = false;
                                        
                                        if self.board[x as usize][y as usize].piece_type == PieceType::King && self.board[x as usize][y as usize].moved == false {
                                            // King able to castle
                                            if self.board[(x+3) as usize][y as usize].piece_type == PieceType::Rook && self.board[(x+3) as usize][y as usize].moved == false {
                                                // Rook able to castle
                                                self.can_castle = true;
                                            }
                                        }

                                        if self.board[x as usize][y as usize].piece_type == PieceType::Pawn && self.board[x as usize][y as usize].moved == true {
                                            if y == 3 {
                                                self.can_en_passant = true;
                                            }
                                        }
                                    }
                                } else {
                                    if self.valid_moves.contains(&[x, y]) {
                                        self.history.insert(0, self.board);
                                        self.board[x as usize][y as usize] = self.board[self.selected_piece[0] as usize][self.selected_piece[1] as usize];
                                        self.board[x as usize][y as usize].moved = true;
                                        self.board[self.selected_piece[0] as usize][self.selected_piece[1] as usize].team = Team::None;
                                        self.board[self.selected_piece[0] as usize][self.selected_piece[1] as usize].piece_type = PieceType::None;

                                        self.piece_selected = false;

                                        self.advantage = calculate_advantage(&self.board);

                                        switch_teams(&mut self.current_team);
                                        
                                        for y2 in (0..=7).rev() {
                                            for x2 in 0..=7 {
                                                let current_piece = self.board[x2 as usize][y2 as usize];
                                                if current_piece.piece_type == PieceType::King && current_piece.team == self.current_team {
                                                    let check_list = final_move_list(current_piece, [x2, y2], &self.board, true);
                                                    let check = is_in_check(self.current_team, &self.board);

                                                    if check_list.is_empty() && check == true {
                                                        println!("GAME OVER!!!");
                                                        match self.current_team {
                                                            Team::White => { println!("BLACK WINS!"); },
                                                            Team::Black => { println!("WHITE WINS!"); },
                                                            _ => { println!("ERROR!"); }
                                                        }
                                                        std::process::exit(0);
                                                    }
                                                }
                                            }
                                        }

                                        if is_stalemate(self.current_team, &self.board) == true {
                                            println!("GAME OVER!! STALEMATE!!");
                                            std::process::exit(1);
                                        }

                                    } else {
                                        castle(self.can_castle,
                                               x as i32,
                                               y as i32,
                                               &mut self.board,
                                               self.selected_piece[0] as i32,
                                               self.selected_piece[1] as i32,
                                               &mut self.current_team,
                                               &mut self.history);
                                        
                                        en_passant(x as i32,
                                                   y as i32,
                                                   &mut self.board,
                                                   self.selected_piece[0] as i32,
                                                   self.selected_piece[1] as i32,
                                                   &mut self.current_team,
                                                   &mut self.history);
                                        
                                        self.piece_selected = false;
                                    }
                                }
                            }
                        });
                    }
                    ui.end_row();
                }
            });

            if self.advantage.0 > self.advantage.1 {
                ui.label(egui::RichText::new(format!("+{}", self.advantage.0 - self.advantage.1)).color(egui::Color32::from_rgb(255, 255, 255)).size(25.0));
            }

            for (i, board) in self.history.clone().into_iter().enumerate() {
                ui.label("\n"); // Seriously egui, seriously?
                egui::Grid::new(format!("grid {}", i))
                    .min_col_width(32.0)
                    .min_row_height(32.0)
                    .show(ui, |ui| {
                    for y in (0..=7).rev() {
                        for x in 0..=7 {
                            let mut bg_colour = if (x+y) % 2 == 0 {
                                Color32::from_rgb(117, 149, 85)
                            } else {
                                Color32::from_rgb(237, 237, 209)
                            };

                            let frame = egui::Frame::new()
                                .fill(bg_colour)
                                .inner_margin(egui::Margin::ZERO);
                        
                            frame.show(ui, |ui| {
                                let piece = board[x as usize][y as usize];
                                let image = match (piece.team, piece.piece_type) {
                                    (Team::Black, PieceType::Rook) => egui::include_image!("../assets/black_rook.png"),
                                    (Team::Black, PieceType::Knight) => egui::include_image!("../assets/black_knight.png"),
                                    (Team::Black, PieceType::Bishop) => egui::include_image!("../assets/black_bishop.png"),
                                    (Team::Black, PieceType::Queen) => egui::include_image!("../assets/black_queen.png"),
                                    (Team::Black, PieceType::King) => egui::include_image!("../assets/black_king.png"),
                                    (Team::Black, PieceType::Pawn) => egui::include_image!("../assets/black_pawn.png"),
                                    (Team::White, PieceType::Rook) => egui::include_image!("../assets/white_rook.png"),
                                    (Team::White, PieceType::Knight) => egui::include_image!("../assets/white_knight.png"),
                                    (Team::White, PieceType::Bishop) => egui::include_image!("../assets/white_bishop.png"),
                                    (Team::White, PieceType::Queen) => egui::include_image!("../assets/white_queen.png"),
                                    (Team::White, PieceType::King) => egui::include_image!("../assets/white_king.png"),
                                    (Team::White, PieceType::Pawn) => egui::include_image!("../assets/white_pawn.png"),
                                    _ => egui::include_image!("../assets/blank.png"),
                                };

                                let response = ui.add(
                                    egui::ImageButton::new(image)
                                        .frame(false)
                                );
                            });
                        }
                        ui.end_row();
                    }
                });
            }
        });
        });
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_in_check_simple() {
        // Setup a simple board where black king is attacked by white rook
        let empty_piece = create_piece(PieceType::None, Team::None);
        let mut board = [[empty_piece; 8]; 8];

        board[4][4] = create_piece(PieceType::King, Team::Black);
        board[4][7] = create_piece(PieceType::Rook, Team::White);

        assert!(is_in_check(Team::Black, &board));
        assert!(!is_in_check(Team::White, &board));
    }

    #[test]
    fn test_is_stalemate_true() {
        // Setup a known stalemate position (black to move, no moves, not in check)
        let empty_piece = create_piece(PieceType::None, Team::None);
        let mut board = [[empty_piece; 8]; 8];

        board[7][7] = create_piece(PieceType::King, Team::Black);
        board[5][6] = create_piece(PieceType::King, Team::White);
        board[6][5] = create_piece(PieceType::Queen, Team::White);

        assert!(is_stalemate(Team::Black, &board));
        assert!(!is_stalemate(Team::White, &board));
    }

    #[test]
    fn test_is_in_check() {
        let empty_piece = create_piece(PieceType::None, Team::None);
        let mut board = [[empty_piece; 8]; 8];

        board[4][4] = create_piece(PieceType::King, Team::White);
        board[5][5] = create_piece(PieceType::Pawn, Team::Black);

        assert!(is_in_check(Team::White, &board));
    }

    #[test]
    fn test_is_stalemate_false_with_legal_move() {
        let empty_piece = create_piece(PieceType::None, Team::None);
        let mut board = [[empty_piece; 8]; 8];

        board[0][0] = create_piece(PieceType::King, Team::Black);
        board[1][0] = create_piece(PieceType::Pawn, Team::Black); // Can move

        board[7][7] = create_piece(PieceType::King, Team::White);

        assert!(!is_stalemate(Team::Black, &board));
    }

    #[test]
    fn test_checkmate_scenario() {
        let empty_piece = create_piece(PieceType::None, Team::None);
        let mut board = [[empty_piece; 8]; 8];

        // Black king cornered and attacked (simulate a checkmate position)
        board[7][7] = create_piece(PieceType::King, Team::Black);
        board[6][7] = create_piece(PieceType::Rook, Team::White);
        board[7][6] = create_piece(PieceType::Rook, Team::White);

        assert!(is_in_check(Team::Black, &board));
    }

    #[test]
    fn test_empty_board() {
        let empty_piece = create_piece(PieceType::None, Team::None);
        let board = [[empty_piece; 8]; 8];

        assert!(!is_in_check(Team::White, &board));
        assert!(!is_in_check(Team::Black, &board));
        assert!(!is_stalemate(Team::White, &board));
        assert!(!is_stalemate(Team::Black, &board));
    }

    #[test]
    fn test_is_in_check_diagonal() {
        let empty_piece = create_piece(PieceType::None, Team::None);
        let mut board = [[empty_piece; 8]; 8];

        board[4][4] = create_piece(PieceType::King, Team::White);
        board[1][1] = create_piece(PieceType::Bishop, Team::Black);

        assert!(is_in_check(Team::White, &board));
    }
}
