

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

struct ComponentCollection {
    comps: Vec<Box<Component>>,
}
impl ComponentCollection {
    fn update(&self) {
        for c in self.comps.iter() {
            println!("asdf");
        }
    }
}

struct CommandCollection {
    cmds: Vec<Box<dyn CommandBase>>,
}
impl CommandCollection {
    fn new() -> Self {
        Self { cmds: Vec::new(), }
    }
    fn process(&mut self) {
        for c in self.cmds.iter() {
            c.process();
        }
        self.cmds.clear();
    }
    fn add(&mut self, cmd: Box<dyn CommandBase>) {
        self.cmds.push(cmd);
    }
}

fn main() {
    let mut cmds = CommandCollection::new();
    cmds.add(Box::new(CommandMoveTo{x:3,y:9,who:0}));
    cmds.add(Box::new(CommandTakeDamage{who:0,delta:13,}));
    cmds.process();

    let mut comp = ComponentCollection { comps:Vec::new(), };
    comp.comps.push( Box::new(Point{x:99, y:99}) );
    comp.comps.push( Box::new(HP{val:11,}) );
   comp.update();
}


