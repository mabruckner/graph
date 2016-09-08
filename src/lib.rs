extern crate starfield_render;
use starfield_render as sf;
use std::mem;

pub trait GridPrint {
    fn get_size(&self) -> (usize, usize);
    fn get_cell(&self, x:usize, y:usize) -> sf::ColorChar;
    fn print(&self) {
        let (width,height) = self.get_size();
        for i in (0..height).rev() {
            println!("{}", sf::make_colorstring((0..width).map(|x|{self.get_cell(x, i)})));
        }
    }
}

pub struct Graph<D> {
    buf: sf::Buffer<bool>,
    data: D,
    renderer: Box<Fn(&D, usize, usize) -> bool>
}

impl <D> Graph<D> {
    pub fn render(&mut self) {
        for x in 0..self.buf.width {
            for y in 0..self.buf.height {
                self.buf.set(x, y, (*self.renderer)(&self.data, x, y));
            }
        }
    }
    pub fn set_data(&mut self, mut data: D) -> D{
        mem::swap(&mut data, &mut self.data);
        self.render();
        data
    }
}

impl <V> Graph<Vec<V>> where V: 'static{
    pub fn hist(width: usize, height: usize, key: Box<Fn(&V) -> f32>) -> Graph<Vec<V>> {
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
}

impl <D> GridPrint for Graph<D> {
    fn get_size(&self) -> (usize, usize)
    {
        (self.buf.width / 2,self.buf.height / 2)
    }
    fn get_cell(&self, x:usize, y:usize) -> sf::ColorChar
    {
        sf::ColorChar(7, 0, sf::grid_cell(&self.buf, x*2, y*2))
    }
}
