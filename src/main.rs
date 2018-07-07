extern crate cmd_pattern;
use cmd_pattern::components::*;
use cmd_pattern::commands::*;

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

struct HP {
    val: i32,
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

fn main() {
    let mut cmds = CommandCollection::new();
    cmds.add(Box::new(CommandMoveTo{x:3,y:9,who:0}));
    cmds.process();

    let mut comp = ComponentCollection::new();
    comp.add( Box::new(Point{x:9, y:9}) );
    comp.add( Box::new(HP{val:66}) );
    comp.update();
}


