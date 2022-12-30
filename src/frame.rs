use crate::{NUM_COLS, NUM_ROWS};
//Alias for a two dimensional vector of string slices
pub type Frame = Vec<Vec<& 'static str>>;

//Creates a new frame
pub fn new_frame() -> Frame {
    let mut cols = Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS{
        let mut col = Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_ROWS  {
            //Creates a blank string slice
            col.push(" ");
        }
        cols.push(col);
    } 
    cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}