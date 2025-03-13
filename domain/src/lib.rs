pub mod chessgame;
mod move_generator;
pub mod stringtomove;

mod filters {
    pub mod cantcapture;
    pub mod moveafter;
    pub mod movefilter;
    pub mod moveto;
    pub mod nofilter;
    pub mod openingmove;
}
