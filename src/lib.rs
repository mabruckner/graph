//! A basic library for outputting graphical data to the terminal.
//!
//! If matplotlib is easy, then this library is dead simple. It tries to squeeze a lot of visual
//! fidelity out of the terminal, but it wont ever be as exact as an actual graphical system. If
//! you have ever wanted a quick-and-easy debugging solution that doesn't involve massive
//! boilerplate and external viewing programs then this is the library for you.

extern crate starfield_render;
use starfield_render as sf;
use std::mem;

static hblocks: [char; 9] = [' ','▏','▎','▍','▌','▋','▊','▉','█'];
static vblocks: [char; 9] = [' ','▁','▂','▃','▄','▅','▆','▇','█'];

/// A trait for objects that can be represented as a grid of characters.
pub trait GridPrint {
    /// Gets the dimensions as (width, height). Calls to `get_cell` should not exceed these bounds.
    fn get_size(&self) -> (usize, usize);

    /// Gets the character residing in cell (x,y). Calls to `get_cell` should respect the bounds set
    /// by `get_size`. This library assumes that y index increases with decreasing vertical position.
    /// (This is contrary to starfield)
    fn get_cell(&self, x:usize, y:usize) -> sf::ColorChar;

    /// Print the grid to standard out.
    fn print(&self) {
        let (width,height) = self.get_size();
        for i in 0..height {
            println!("{}", sf::make_colorstring((0..width).map(|x|{self.get_cell(x, i)})));
        }
    }
}

/// A general structure representing a simple 2D graph.
///
/// Each 'pixel' on this graph can be either on or off, so heat maps are out unless dithering or
/// contour lines are involved. `Graph` makes use of unicode block elements (eg. `▚`,  `▛`, `▗`) to
/// double its effective resolution.
pub struct Graph<D> {
    buf: sf::Buffer<bool>,
    data: D,
    renderer: Box<Fn(&D, usize, usize) -> bool>
}

impl <D> Graph<D> {
    /// Render the graph data to the internal buffer. This should be called automatically in all
    /// cases.
    pub fn render(&mut self) {
        for x in 0..self.buf.width {
            for y in 0..self.buf.height {
                self.buf.set(x, y, (*self.renderer)(&self.data, x, y));
            }
        }
    }
    /// Set the graph data, returning ownership of the previous data.
    pub fn set_data(&mut self, mut data: D) -> D{
        mem::swap(&mut data, &mut self.data);
        self.render();
        data
    }
}

impl <V> Graph<Vec<V>> where V: 'static{
    /// Create a graph configured as a histogram. `width` and `height` are effective values, not
    /// `GridPrint` values. Divide by two to get the number of characters the graph will take up.
    /// `key` is a function that translates values from the data vector into the domain of [0.0,
    /// 1.0).
    pub fn hist(width: usize, height: usize, key: Box<Fn(&V) -> f32>) -> Graph<Vec<V>>
    {
        let mut buf = sf::Buffer::new(width, height, false);
        let thing = move |dat: &Vec<V>, x:usize, y:usize| {
            if dat.len() <= 1 {
                false
            } else {
                let pos = dat.len() as f32 * x as f32 / width as f32;
                let index = pos as usize;
                let h = y as f32 / height as f32 ;
                key(&dat[index]) >= h
            }
        };
        Graph {
            buf: buf,
            data: Vec::new(),
            renderer: Box::new(thing)
        }
    }
    pub fn scatter(width: usize, height: usize, hkey: Box<Fn(&V) -> f32>, vkey: Box<Fn(&V) -> f32>) -> Graph<Vec<V>>
    {
        let mut buf = sf::Buffer::new(width, height, false);
        let thing = move |dat: &Vec<V>, x: usize, y: usize|
        {
            for val in dat {
                let (a, b) = (hkey(val), vkey(val));
                let (a, b) = (a*width as f32, b*height as f32);
                if a.floor() == x as f32 && b.floor() == y as f32 {
                    return true
                }
            }
            false
        };
        Graph {
            buf: buf,
            data: Vec::new(),
            renderer: Box::new(thing)
        }
    }
}

impl <D> GridPrint for Graph<D> {
    fn get_size(&self) -> (usize, usize)
    {
        (self.buf.width / 2,self.buf.height / 2)
    }
    fn get_cell(&self, x:usize, y:usize) -> sf::ColorChar
    {
        sf::ColorChar(0xE7, 0x10, sf::grid_cell(&self.buf, x*2, self.buf.height - (y+1)*2))
    }
}

/// A simple structure representing a single horizontal bar.
///
/// This can be used to represent a single line of a horizontal bar plot. It makes use of unicode
/// block elements to increase the horizontal resolution to eight values per character.
pub struct HBar {
    /// A value from 0.0 to 1.0 that represents how much of the bar is filled.
    pub value: f32,
    /// How many characters wide the bar is.
    pub width: usize
}

impl HBar {
    /// Create a new `HBar`
    pub fn new(width: usize, value: f32) -> HBar {
        HBar {
            value: value,
            width: width
        }
    }
}

impl GridPrint for HBar {
    fn get_size(&self) -> (usize, usize)
    {
        (self.width, 1)
    }
    fn get_cell(&self, x:usize, y:usize) -> sf::ColorChar
    {
        let index = (9.0*(self.value*self.width as f32 - x as f32)).max(0.0).min(8.0) as usize;
        sf::ColorChar(0xE7, 0x10, hblocks[index])
    }
}
