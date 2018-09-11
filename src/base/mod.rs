
use std::collections::HashMap;
use std::ops::{Index,IndexMut};

pub trait Component {
    fn get_type(&self) -> String;
}
pub struct ComponentCollection {
    comps: HashMap<i32, Vec<Box<Component>>>,
}
impl ComponentCollection {
    pub fn new() -> Self {
        Self { comps: HashMap::new(), }
    }
    pub fn add(&mut self, entity:i32, data: Box<Component>) {
        let ent = self.comps.entry(entity).or_insert(Vec::new());

        // make sure the component does not already exist
        // for this entity
        let mut found_type = false;
        for d in ent.iter() {
            if d.get_type() == data.get_type() {
                found_type = true;
                break;
            }
        }
        if found_type == false {
            ent.push( data );
        }
    }

    pub fn remove(&mut self, entity:i32, data: Box<Component>) {
        match self.comps.get_mut(&entity) {
            None => {},
            Some(ent) => {
                let index = ent.iter().position( |i| {
                    *i.get_type() == data.get_type()
                });

                match index {
                    None => {},
                    Some(i) => { ent.remove(i); },
                }
            }
        }
    }
}

impl Index<i32> for ComponentCollection {
    type Output = Vec<Box<Component>>;
    fn index(&self, index: i32) -> &Self::Output {
        &self.comps[&index]
    }
}

//impl IndexMut<i32> for ComponentCollection {
//    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
//        let mut q:() = &mut self.comps[&index];
//        q
//    }
//}

pub trait Command {
    type WriteData;
    fn apply(&self);
    fn repeat(&self);
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
    pub fn process(&mut self, systemCollection:&mut SystemBase) {
        for c in self.cmds.iter() {
            c.process();
        }
        self.cmds.clear();
    }
    pub fn add(&mut self, cmd: Box<dyn CommandBase>) {
        self.cmds.push(cmd);
    }
}

pub trait System {
    fn update(&mut self, comp_coll:&mut ComponentCollection);
}

pub trait SystemBase;
impl<T> SystemBase for T
where T: System
{
}



