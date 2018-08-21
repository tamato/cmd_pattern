extern crate cmd_pattern;
use cmd_pattern::*;

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}
impl Component for Point {
    fn get_type(&self) -> String {
        String::from("Point")
    }
}

struct HP (i32);
impl Component for HP {
    fn get_type(&self) -> String {
        String::from("HP")
    }
}

struct PoisonComponent {
    dam: Vec<PoisonDamage>,
}
impl Component for PoisonComponent {
    fn get_type(&self) -> String {
        String::from("PoisonComp")
    }
}

struct PoisonDamage (i32);
impl Component for PoisonDamage {
    fn get_type(&self) -> String {
        String::from("PoisonDamage")
    }
}

struct CommandPoison {
    who: i32,
    val: i32,
    // have a ticks to do full damage?
    // should the poison damage be coming from the component?
}
impl Command for CommandPoison {
    type WriteData = i32;
    fn apply(&self) {
        println!("hitting {} with {} poison damage", self.who, self.val);
        println!("Tells poisonManager/system who is poisoned"); 
        // PoisonManager.add( self.who )
    }
    fn repeat(&self) {
    }
}

struct PoisonManager {
    id_list: Vec<i32>,
}
impl PoisonManager {
    fn new() -> Self {
        PoisonManager {
            id_list: Vec::new(),
        }
    }

    fn add(&mut self, who: i32) {
        let mut found = false;
        for c in self.id_list.iter() {
            if *c == who {
                found = true;
                break;
            }
        }
        if false == found {
            self.id_list.push(who);
        }
    }

    fn update(&mut self, comp_coll:&mut ComponentCollection) {
        for i in self.id_list.iter() {
            // as this runs, it should re-add anyone that is still poisoned.
            let comps = &comp_coll[*i];
            let mut thing: Option<&Box<Component>> = None;
            for comp in comps.iter() {
                if comp.get_type() == "PoisonComp" {
                    thing = Some(comp);
                    break;
                }
            }

            match thing {
                Some(thing) => println!("Thing: {}", thing.get_type()),
                None => (),
            }
        }
    
        self.id_list.clear();
    }
}

fn main() {
    let mut cmds = CommandCollection::new();
    cmds.add(Box::new(CommandPoison{who:0, val:6}));
    cmds.process();
    cmds.process(); // process clears out previous commands
                    // need a way to "re-add" any damage over time commands.

    let mut comp = ComponentCollection::new();
    comp.add( 0, Box::new(Point{x:9, y:9}) );
    comp.add( 0, Box::new(HP(66)) );
    comp.add( 0, Box::new(PoisonComponent{dam:Vec::new()}) );

    let mut pm = PoisonManager::new();
    pm.add(0);
    pm.update(&mut comp);
}


