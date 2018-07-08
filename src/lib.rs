#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]


pub mod components {
    use std::collections::HashMap;

    pub trait Component {
        fn getType(&self) -> String;
    }
    pub struct ComponentCollection {
        comps: HashMap<i32, Vec<Box<Component>>>,
    }
    impl ComponentCollection {
        pub fn new() -> Self {
            Self { comps: HashMap::new(), }
        }
        pub fn update(&self) {
        }
        pub fn add(&mut self, entity:i32, data: Box<Component>) {
            let ent = self.comps.entry(entity).or_insert(Vec::new());

            // make sure the component does not already exist
            // for this entity
            let mut foundType = false;
            for d in ent.iter() {
                if d.getType() == data.getType() {
                    foundType = true;
                    break;
                }
            }
            if foundType == false {
                ent.push( data );
            }
        }

        pub fn remove(&mut self, entity:i32, data: Box<Component>) {
            let ent = self.comps.entry(entity);

            // make sure the component does not already exist
            // for this entity
            for d in ent.iter() {
                if d.getType() == data.getType() {
                    foundType = true;
                    break;
                }
            }
        }
    }
}

pub mod commands {
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

