extern crate cmd_pattern;
use cmd_pattern::base::*;

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

struct HP {
    val: i32,
}
impl Component for HP {
    fn get_type(&self) -> String {
        String::from("HP")
    }
}

struct PoisonDamage {
    val: i32,
}
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
     }
    fn repeat(&self) {
    }
 }

fn main() {
    let mut cmds = CommandCollection::new();
    cmds.add(Box::new(CommandPoison{who:0, val:6}));
    cmds.process();
    cmds.process();
    cmds.process();
    cmds.process();

    let mut comp = ComponentCollection::new();
    comp.add( 0, Box::new(Point{x:9, y:9}) );
    comp.add( 0, Box::new(HP{val:66}) );
    comp.update();
}


