#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

pub trait Component {
}

pub trait Command {
    type WriteData;
    fn apply(&self);
}

trait CommandBase {
    fn process(&self);
}

impl<T> CommandBase for T
where T: Command
{
    fn process(&self) {
        self.apply();
    }
}




