use super::{BaseFmc, BaseFmcData};

pub trait CrjVnav {
    fn speed_mode() -> ();
}

#[derive(Debug)]
pub enum CrjVnavMode {
    None,
    Speed,
    Altitude,
    VerticalSpeed,
}

#[derive(Debug)]
pub struct CrjFmc {
    vnav_mode: CrjVnavMode,
    base_fmc: BaseFmcData,
}

impl BaseFmc for CrjFmc {
    fn increase_altitude(&self) -> () {}
    fn decrease_altitude(&self) -> () {}
    fn increase_speed(&self) -> () {}
    fn decrease_speed(&self) -> () {}

    fn altitude_mode() -> () {}
    fn vertical_speed() -> () {}
}

impl Default for CrjFmc {
    fn default() -> CrjFmc {
        CrjFmc {
            base_fmc: BaseFmcData::default(),
            vnav_mode: CrjVnavMode::None,
        }
    }
}

#[derive(Debug)]
pub struct Crj700 {
    fmc: CrjFmc,
}

impl Default for Crj700 {
    fn default() -> Crj700 {
        Crj700 {
            fmc: CrjFmc::default(),
        }
    }
}

impl Crj700 {
    pub fn new() -> Crj700 {
        Crj700::default()
    }
}
