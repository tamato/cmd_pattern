
use std::collections::HashMap;
use std::ops::Index;

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

pub trait Command {
    type WriteData;
    fn apply(&self, system_collection:&mut SystemCollection);
    fn repeat(&self);
}

pub trait CommandBase {
    fn process(&self, system_collection:&mut SystemCollection);
}

impl<T> CommandBase for T
where T: Command
{
    fn process(&self, system_collection:&mut SystemCollection) {
        self.apply(system_collection);
    }
}

pub struct CommandCollection {
    cmds: Vec<Box<dyn CommandBase>>,
}
impl CommandCollection {
    pub fn new() -> Self {
        Self { cmds: Vec::new(), }
    }
    pub fn process_collection(&mut self, system_collection:&mut SystemCollection) {
        for c in self.cmds.iter() {
            c.process(system_collection);
        }
        self.cmds.clear();
    }
    pub fn add(&mut self, cmd: Box<dyn CommandBase>) {
        self.cmds.push(cmd);
    }
}

pub trait System {
    type Data;
    fn update(&self, comp_coll:&mut ComponentCollection);
    fn add(&mut self, data: Self::Data);
}
pub trait SystemBase
{
    fn process(&self, comp_coll:&mut ComponentCollection); 
}
impl<T> SystemBase for T
where T: System
{
    fn process(&self, comp_coll:&mut ComponentCollection) {
        self.update(comp_coll);
    }
}

pub struct SystemCollection {
    systems: Vec<Box<dyn SystemBase>>,
}
impl SystemCollection {
    pub fn new() -> Self {
        SystemCollection { systems:Vec::new() }
    }
    pub fn add(&mut self, system: Box<dyn SystemBase>) {
        self.systems.push(system);
    }
    pub fn process_collection(&self, comp_coll:&mut ComponentCollection) {
        for sys in self.systems.iter() {
            sys.process(comp_coll);
        }
    }
}

impl Index<i32> for SystemCollection {
    type Output = Box<SystemBase>;
    fn index(&self, index: i32) -> &Self::Output {
        &self.systems[index as usize]
    }
}



