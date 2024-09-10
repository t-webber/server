#![allow(unused)]

use core::fmt;

pub type Res<T> = Result<T, String>;

pub trait ToR<T> {
    fn to_r(self) -> Res<T>;
}

impl<T, E: fmt::Debug> ToR<T> for Result<T, E> {
    fn to_r(self) -> Res<T> {
        self.map_err(|err| format!("{err:?}"))
    }
}

pub type Check = Res<()>;

pub trait ToCheck {
    fn to_check(self) -> Check;
}

impl<T, E: fmt::Debug> ToCheck for Result<T, E> {
    fn to_check(self) -> Check {
        self.map_or_else(|err| Err(format!("{err:?}")), |_| Ok(()))
    }
}
