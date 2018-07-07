extern crate cmd_pattern;
use cmd_pattern::stuff::*;

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Component for Point {
}

struct HP {
    val: i32,
}
impl Component for HP {

}


struct CommandMoveTo {
    x: i32,
    y: i32,
    who: i32,
}

impl Command for CommandMoveTo {
    type WriteData = i32;
    fn apply(&self) {
        println!("moveto...{} {}", self.x, self.y);
    }
}

struct CommandTakeDamage {
    who: i32,
    delta: i32,
}
impl Command for CommandTakeDamage {
    type WriteData = i32;
    fn apply(&self) {
        println!("takeDam: {}", self.delta);
    }
}

fn main() {
    let mut cmds = CommandCollection::new();
    cmds.add(Box::new(CommandMoveTo{x:3,y:9,who:0}));
    cmds.add(Box::new(CommandTakeDamage{who:0,delta:13,}));
    cmds.process();

    let mut comp = ComponentCollection::new();
    comp.add( Box::new(Point{x:9, y:9}) );
    comp.add( Box::new(HP{val:66}) );
    comp.update();
}


