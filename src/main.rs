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
    fn apply(&self, system_collection:&mut SystemCollection) {
        println!("hitting {} with {} poison damage", self.who, self.val);
        println!("Tells poisonSystem who is poisoned"); 
        
        let a = &system_collection[0];
        a.add(0);
    }
    fn repeat(&self) {
    }
}

struct PoisonSystem {
    id_list: Vec<i32>,
}
impl System for PoisonSystem 
{
    type Data = i32;
    fn update(&self, comp_coll:&mut ComponentCollection) {
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
                Some(thing) => {
                    println!("Thing: {}", thing.get_type())
                },
                None => (),
            }
        }
    }

    // Add target to be poisoned
    fn add(&mut self, who: Self::Data) {
        let mut found = false;
        // only add them once
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
}

fn main() {
    let mut comp = ComponentCollection::new();
    comp.add( 0, Box::new(Point{x:9, y:9}) );
    comp.add( 0, Box::new(HP(66)) );
    comp.add( 0, Box::new(PoisonComponent{dam:Vec::new()}) );

    let mut sc = SystemCollection::new();
    let pm = PoisonSystem{ id_list:Vec::new() };
    sc.add(Box::new(pm));
    sc.process_collection(&mut comp);

    let mut cmds = CommandCollection::new();
    cmds.add(Box::new(CommandPoison{who:0, val:6}));
    cmds.process_collection(&mut sc);
    cmds.process_collection(&mut sc);  // process clears out previous commands
                            // need a way to "re-add" any damage over time commands.
}

/***
    order of operations
    CommandCollection holds commands to be processed
    ComponentCollection associates an ID to a list of components

    PoisonSystem 
        find the target to be poisoned
        finds the poison compoent
        applies poison
**/

