pub mod start;
pub mod mate;
pub mod announce;
pub mod round_summary;
pub mod end;
pub mod info;

pub enum Windowtype {
    None,
    Announce,
    Roundsummary,
    End,
    Info(&'static str),
    Start,
    Mate,
}
