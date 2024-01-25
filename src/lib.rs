use std::fmt::{Debug, Display};

pub trait DisplayPrintable {
    fn println(&self);
    fn print(&self);
}

impl<T: Display> DisplayPrintable for T {
    fn println(&self) {
        println!("{}", self)
    }
    fn print(&self) {
        print!("{}", self)
    }
}

pub trait MDisplay {
    fn display(&self) -> String;
}

impl<T: Display> MDisplay for T {
    fn display(&self) -> String {
        format!("{}", self)
    }
}

pub trait MDebug {
    fn debug(&self) -> String;
}

impl<T: Debug> MDebug for T {
    fn debug(&self) -> String {
        format!("{:#?}", self)
    }
}
