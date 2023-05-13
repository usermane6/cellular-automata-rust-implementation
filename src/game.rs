use rand::prelude::*;

use crate::display::PixelTexture;

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    DEAD,
    ALIVE,
}

pub struct Filter {
    l: Vec<i32>
}

impl Filter {
    pub fn new(range: i32, size: (usize, usize)) -> Filter {
        let mut l: Vec<i32> = vec![];
        let w = size.0 as i32;

        for i in -range..range {
            for j in -range..range {
                let (x, y) = (i, j);
                l.push((w * y) + x)
            }
        }
        
        Filter { l }
    }

}

pub struct State<'a> {
    tiles: Vec<Tile>,
    size: (usize, usize),
    filter: &'a Filter
}

impl State<'_> {
    pub fn random<'a>(size: (usize, usize), filter: &'a Filter) -> State<'a> {
        let mut rng = thread_rng();
        let mut tiles: Vec<Tile> = Vec::new();

        for _ in 0..size.0 * size.1 {
            let val = rng.gen_range(0..=1);
            tiles.push(
                match val {
                    0 => {Tile::DEAD},
                    1 => {Tile::ALIVE},
                    _ => {Tile::DEAD}
                }
            );
        }

        // println!("------ random len: {}", tiles.len());

        State { tiles, size: size, filter }
    }

    pub fn from_previous(previous_state: State) -> State {
        
        // println!("------ new iteration created | previous len: {}", previous_state.tiles.len());

        let (mut x, mut y): (usize, usize) = (0, 0);
        //cap size of new tile list
        let mut tiles: Vec<Tile> = Vec::with_capacity(previous_state.tiles.len());

        for tile in &previous_state.tiles {
            let neighbors = previous_state.get_neighbor_sum(x, y, 1);

            if tile == &Tile::DEAD && neighbors == 3 { tiles.push(Tile::ALIVE) }
            else if neighbors < 2 || neighbors > 3 { tiles.push(Tile::DEAD) }
            else {tiles.push(*tile)}

            // manage position
            x += 1;
            if x >= previous_state.size.0 { x = 0; y += 1; }
        }

        State{ tiles, size: previous_state.size, filter: previous_state.filter }
    }

    fn get_neighbor_sum(&self, x: usize, y: usize, range: i32) -> usize {
        let mut total: usize = 0;

        // println!("------ call to get neighbor");

        for i in -range..=range {
            for j in -range..=range {
                if j == i && j == 0 {continue;}

                total += self.tile_val(
                        wrap(x as i32 + i, self.size.0 as i32) as usize,
                        wrap(y as i32 + j, self.size.1 as i32) as usize
                    );
            }
        }

        total
    }

    fn tile_val(&self, x: usize, y: usize) -> usize {
        let id = self.id_from_pos(x, y);
        
        if let Tile::ALIVE = self.tiles[id] { 1 }
        else { 0 }
    }
 
    fn id_from_pos(&self, x: usize, y: usize) -> usize {
        (( self.size.0) * y) + x
    }

    fn pos_from_id (&self, id: usize)-> (usize, usize) {
        (
            id % self.size.0,
            id / self.size.0
        )
    }


    pub fn push_state(&self, pixel_texture: &mut PixelTexture<'_>) {
        let mut pixels: Vec<u8> = Vec::new(); 

        for tile in &self.tiles {
            match tile {
                Tile::ALIVE => {
                    pixels.append( &mut vec![255_u8; 4] );
                },
                Tile::DEAD => {
                    pixels.append( &mut vec![0_u8; 4] );
                }
            }
        }

        let pixels_slice: &[u8] = &pixels[..];

        pixel_texture.update_texture(pixels_slice)
    }
}

fn wrap(a: i32, len: i32) -> i32 {
    (len + a) % len
}