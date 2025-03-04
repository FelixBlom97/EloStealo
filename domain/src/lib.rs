pub mod chessgame;
mod movefilter;
pub mod stringtomove;

mod filters {
    pub mod cantcapture;
    pub mod moveafter;
    pub mod moveto;
    pub mod nofilter;
    pub mod openingmove;
}
