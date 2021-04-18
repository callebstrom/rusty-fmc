pub mod crj;

pub trait BaseFmc {
    fn increase_altitude(&self) -> ();
    fn decrease_altitude(&self) -> ();
    fn increase_speed(&self) -> ();
    fn decrease_speed(&self) -> ();

    fn altitude_mode() -> ();
    fn vertical_speed() -> ();
}

#[derive(Debug)]
pub struct BaseFmcData {
    altitude: i8,
    speed: i8,
    vertical_speed: i8,
}

impl Default for BaseFmcData {
    fn default() -> BaseFmcData {
        BaseFmcData {
            altitude: 0,
            speed: 0,
            vertical_speed: 0,
        }
    }
}
