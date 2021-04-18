mod simconnect;

pub trait SimulatorAdapter {
    fn get_parameter<T>() -> T;
    fn set_parameter<T>() -> T;
}
