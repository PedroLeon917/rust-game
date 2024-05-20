use std::fmt::{self};

#[derive(Debug)]
enum Gem {
    Diamond = 1,
    Sapphire,
    Ruby,
    Topaz,
    Onyx,
    Jade    
}
impl fmt::Display for Gem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Gem::Diamond => write!(f, "diamond"),
            Gem::Sapphire => write!(f, "sapphire"),       
            Gem::Ruby => write!(f, "ruby"),
            Gem::Topaz => write!(f, "topaz"),       
            Gem::Onyx => write!(f, "onyx"),
            Gem::Jade => write!(f, "jade"),       
        }
    }
}
fn main() {
    let mut map = [[0; 5]; 5];
    // println!("{map:?}");

    map[4][2] = 1;
    map[1][2] = 2;
    map[3][3] = 3;
    map[0][2] = 4;
    map[1][4] = 5;

    for row in map {
        println!("{row:?}")
    }

    let mut found: Vec<Gem>= Vec::new();

    found.push(Gem::Diamond);

    println!("{found:?}")
}
