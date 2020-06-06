use enum_dispatch::enum_dispatch;

#[derive(Debug)]
pub enum Status {
    Success,
    Failure,
    Running
}

#[enum_dispatch]
pub trait Node {
    fn tick(&self) -> Status {
        Status::Success
    }
}

#[enum_dispatch]
pub trait Service {
    fn serve(&self) -> Status {
        Status::Success
    }
}

#[enum_dispatch]
pub trait Decorator {
    fn decorate(&self, status: Status) -> Status {
        status
    }
}
