#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

pub mod stuff {
pub trait Component {
}
pub trait Command {
    type WriteData;
    fn apply(&self);
}

pub trait CommandBase {
    fn process(&self);
}

impl<T> CommandBase for T
where T: Command
{
    fn process(&self) {
        self.apply();
    }
}
pub struct ComponentCollection {
    comps: Vec<Box<Component>>,
}
impl ComponentCollection {
    pub fn new() -> Self {
        Self { comps: Vec::new(), }
    }
    pub fn update(&self) {
        for c in self.comps.iter() {
            println!("asdf");
        }
    }
    pub fn add(&mut self, data: Box<Component>) {
        self.comps.push( data );
    }
}

pub struct CommandCollection {
    cmds: Vec<Box<dyn CommandBase>>,
}
impl CommandCollection {
    pub fn new() -> Self {
        Self { cmds: Vec::new(), }
    }
    pub fn process(&mut self) {
        for c in self.cmds.iter() {
            c.process();
        }
        self.cmds.clear();
    }
    pub fn add(&mut self, cmd: Box<dyn CommandBase>) {
        self.cmds.push(cmd);
    }
}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

