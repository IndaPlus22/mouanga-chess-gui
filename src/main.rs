/**
 * Chess GUI.
 * Author: Anders Mouanga <mouanga@kth.se>
 * Based on Chess GUI Template from Isak Larsson <isaklar@kth.se>.
 */

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::collections::HashMap;

use chess_template::{Colour, Game, PieceType};

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, MouseButton, MouseCursorEvent, PressEvent, Key};

/// A chess board is 8x8 tiles.
const GRID_SIZE: i16 = 8;
/// Sutible size of each tile.
const GRID_CELL_SIZE: (i16, i16) = (90, 90);

/// Size of the application window.
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE as f32 * GRID_CELL_SIZE.1 as f32,
);

// GUI Color representations
const BLACK: [f32; 4] = [228.0 / 255.0, 196.0 / 255.0, 108.0 / 255.0, 1.0];
const WHITE: [f32; 4] = [188.0 / 255.0, 140.0 / 255.0, 76.0 / 255.0, 1.0];

pub struct App {
    gl: GlGraphics,                                 // OpenGL drawing backend.
    mouse_pos: [f64; 2],                            // Current mouse postition
    sprites: HashMap<(Colour, PieceType), Texture>, // For easy access to the apropriate PNGs
    board: [[Option<(Colour, PieceType)>; 8]; 8], // Or whatever way you prefer to represent the board (hint: might not be neccesary)
    pub game: Game, // Save piece positions, which tiles has been clicked, current colour, etc...
}

impl App {
    fn new(opengl: OpenGL) -> App {
        let royal_rank = |colour| {
            [
                Some((colour, PieceType::Rook)),
                Some((colour, PieceType::Knight)),
                Some((colour, PieceType::Knight)), // Should be bishop but this part is not even used
                Some((colour, PieceType::Queen)),
                Some((colour, PieceType::King)),
                Some((colour, PieceType::Bishop)),
                Some((colour, PieceType::Knight)),
                Some((colour, PieceType::Rook)),
            ]
        };
        let pawn_rank = |colour| [Some((colour, PieceType::Pawn)); 8];
        let empty_rank = || [None; 8];

        App {
            gl: GlGraphics::new(opengl),
            mouse_pos: [0., 0.],
            board: [
                royal_rank(Colour::Black),
                pawn_rank(Colour::Black),
                empty_rank(),
                empty_rank(),
                empty_rank(),
                empty_rank(),
                pawn_rank(Colour::White),
                royal_rank(Colour::White),
            ],
            game: chess_template::Game::new(),
            sprites: Self::load_sprites(),
        }
    }
    fn render(&mut self, args: &RenderArgs, glyphs: &mut GlyphCache) {
        use graphics::*; // Now we don't have to use this everytime :D

        let square = rectangle::square(0.0, 0.0, GRID_CELL_SIZE.0 as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // // Clear the screen.
            // clear(GREEN, gl);
            // draw grid
            for row in 0..8 {
                for col in 0..8 {
                    // draw tile
                    rectangle(
                        match col % 2 {
                            0 => {
                                if row % 2 == 0 {
                                    BLACK
                                } else {
                                    WHITE
                                }
                            }
                            _ => {
                                if row % 2 == 0 {
                                    WHITE
                                } else {
                                    BLACK
                                }
                            }
                        },
                        square,
                        c.transform.trans(
                            (col * GRID_CELL_SIZE.0) as f64,
                            (row * GRID_CELL_SIZE.0) as f64,
                        ),
                        gl,
                    );

                    // draw piece
                    if let Some(piece) = self.game.get_board()[63 - chess_template::Position::new(row.try_into().unwrap(), col.try_into().unwrap()).unwrap().idx] /* This ilne inspired by the library author's implementation of Position struct in his own main.rs file */ {
                        let img = Image::new().rect(square);
					//	println!("Drawing{:?} at square {}", piece, 63 - chess_template::Position::new(row.try_into().unwrap(), col.try_into().unwrap()).unwrap().idx);
                        img.draw(
                            self.sprites.get(&(piece.colour, piece.piece_type)).unwrap(),
                            &c.draw_state,
                            c.transform.trans(
                                (col * GRID_CELL_SIZE.0) as f64,
                                (row * GRID_CELL_SIZE.0) as f64,
                            ),
                            gl,
                        )
					
                    }
                }
				
            }

            // Draw text
            // We do some calculations to center the text
            // Is not exactly in the middle, try to fix it if you want to!
            let state_text = format!(""/*"{:?} | {:?}", self.game.get_game_state(), self.game.get_active_colour()*/);
            let text_size: (f32, f32) = ((24 * state_text.len()) as f32, 24f32);
            let text_postition = c.transform.trans(
                ((SCREEN_SIZE.0 - text_size.0) / 2f32) as f64,
                ((SCREEN_SIZE.1 - text_size.1) / 2f32) as f64,
            );
            text::Text::new(32)
                .draw(&state_text, glyphs, &c.draw_state, text_postition, gl)
                .unwrap();
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Currently empty, but maybe you can find a fun use for it!
    }

    #[rustfmt::skip]
    /// Loads chess piese images into vector.
    fn load_sprites() -> HashMap<(Colour, PieceType), Texture> {

        [
            ((Colour::Black, PieceType::King), "resources/black_king.png".to_string()),
            ((Colour::Black, PieceType::Queen), "resources/black_queen.png".to_string()),
            ((Colour::Black, PieceType::Rook), "resources/black_rook.png".to_string()),
            ((Colour::Black, PieceType::Pawn), "resources/black_pawn.png".to_string()),
            ((Colour::Black, PieceType::Bishop), "resources/black_bishop.png".to_string()),
            ((Colour::Black, PieceType::Knight), "resources/black_knight.png".to_string()),
            ((Colour::White, PieceType::King), "resources/white_king.png".to_string()),
            ((Colour::White, PieceType::Queen), "resources/white_queen.png".to_string()),
            ((Colour::White, PieceType::Rook), "resources/white_rook.png".to_string()),
            ((Colour::White, PieceType::Pawn), "resources/white_pawn.png".to_string()),
            ((Colour::White, PieceType::Bishop), "resources/white_bishop.png".to_string()),
            ((Colour::White, PieceType::Knight), "resources/white_knight.png".to_string())
        ]
            .iter()
            .map(|(piece, path)| {
                (*piece, Texture::from_path(path, &TextureSettings::new()).unwrap())
            })
            .collect::<HashMap<(Colour, PieceType), Texture>>()
    }
}

fn coords_to_square(x: f64, y: f64) -> String {
    let new_x: i16 = x as i16;
    let new_y: i16 = y as i16;
    let mut square_result = "".to_string();
    if new_x / (GRID_CELL_SIZE.0 as i16) == 7 {
        square_result.push('a')
    } else if new_x / (GRID_CELL_SIZE.0) as i16 == 6 {
        square_result.push('b')
    } else if new_x / (GRID_CELL_SIZE.0) as i16 == 5 {
        square_result.push('c')
    } else if new_x / (GRID_CELL_SIZE.0) as i16 == 4 {
        square_result.push('d')
    } else if new_x / (GRID_CELL_SIZE.0) as i16 == 3 {
        square_result.push('e')
    } else if new_x / (GRID_CELL_SIZE.0) as i16 == 2 {
        square_result.push('f')
    } else if new_x / (GRID_CELL_SIZE.0) as i16 == 1 {
        square_result.push('g')
    } else if new_x / (GRID_CELL_SIZE.0) as i16 == 0 {
        square_result.push('h')
    }
    square_result.push(match 7 - new_y / GRID_CELL_SIZE.0 {
        0 => '1',
        1 => '2',
        2 => '3',
        3 => '4',
        4 => '5',
        5 => '6',
        6 => '7',
        7 | 8 => '8',
        _ => panic!("How is your y position equal to {y}?!"),
    });
    return square_result;

    }

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    let mut move_in_progress = false;
    let mut game = chess_template::Game::new();

    // Create a Glutin window.
    let mut window: Window =
        WindowSettings::new("Chess", [SCREEN_SIZE.0 as f64, SCREEN_SIZE.1 as f64])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

    // Initialize our app state
    let mut app = App::new(opengl);

    // Initialize font
    let mut glyphs = GlyphCache::new(
        "resources/AbyssinicaSIL-Regular.ttf",
        (),
        TextureSettings::new(),
    )
    .unwrap();

    let mut events = Events::new(EventSettings::new());
    // Our "game loop". Will run until we exit the window
    let mut mouse_x: f64 = 0.0;
    let mut mouse_y: f64 = 0.0;
    let mut start_square: String = "a0".to_string();


    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, &mut glyphs);
        }
        if let Some(args) = e.update_args() {
            app.update(&args);
        }
        if let Some(pos) = e.mouse_cursor_args() { // The following code is most definitely horribly inefficient but this is easier for me to read :)
            mouse_x = pos[0];
            mouse_y = pos[1];

        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            // Moves are split into 2 parts:
            // The "pick-up-the-piece" part, only doable when move_in_progress is false.
            // The "place-down-the-piece" part, only doable when move_in_progress is true.
          ///////  println!("Mouse coords are: ({}, {})", mouse_x, mouse_y);
          ///////  println!("You clicked on the square: {}", coords_to_square(mouse_x, mouse_y));
		  ///////  println!("Move in progress is equal to: {move_in_progress}");
          //  println!("The board is equal to: {:?}", game.get_board());
			// println!("The piece at the clicked position is: {:?}", chess_template::Position::parse_str(&coords_to_square(mouse_x, mouse_y)));

            if move_in_progress == false  {
            start_square = coords_to_square(mouse_x, mouse_y);
            println!("Moving from {}...", start_square);     
            move_in_progress = true;
            }
            else {
                match app.game.make_move(&start_square, &coords_to_square(mouse_x, mouse_y)) {
                    Ok(_) => println!("OK: {}-{}", &start_square, &coords_to_square(mouse_x,mouse_y)),
                    Err(message) => println!("Error: \"{}\" at attempted move `{}-{}`", message, &start_square, &coords_to_square(mouse_x, mouse_y)),
                    _ => println!("Oops!"),
                }
                move_in_progress = false;
            }
        }

        if let Some(Button::Mouse(MouseButton::Right)) = e.press_args() {
            app = App::new(opengl);
            game = chess_template::Game::new();
        }
		
		if let Some(Button::Keyboard(Key::Left)) = e.press_args() {
			if game.get_game_state() == chess_template::GameState::WaitingOnPromotionChoice { 
				game.set_promotion("queen".to_string());
			}
		}
		
		if let Some(Button::Keyboard(Key::Up)) = e.press_args() {
			if game.get_game_state() == chess_template::GameState::WaitingOnPromotionChoice { 
				game.set_promotion("rook".to_string());
			}
		}
		
		if let Some(Button::Keyboard(Key::Right)) = e.press_args() {
			if game.get_game_state() == chess_template::GameState::WaitingOnPromotionChoice { 
				game.set_promotion("bishop".to_string());
			}
		}
		
		if let Some(Button::Keyboard(Key::Down)) = e.press_args() {
			if game.get_game_state() == chess_template::GameState::WaitingOnPromotionChoice { 
				game.set_promotion("knight".to_string());
			}
		}
    }
}
