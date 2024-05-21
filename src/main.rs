use std::fmt::{self};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug, FromPrimitive)]
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

// Implementando manualmente la funcion from_number 
// para procesar el vector 
//
// impl Gem {
//     fn from_number(num: u8) -> Option<Gem> {
//         match num {
//             1 => Some(Gem::Diamond),
//             2 => Some(Gem::Sapphire),
//             3 => Some(Gem::Ruby),
//             4 => Some(Gem::Topaz),
//             5 => Some(Gem:: Onyx),
//             6 => Some(Gem::Jade),
//             _ => None,
//         }
//     }
// }

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

    // found.push(Gem::Diamond);

    let data = map[1][4];

    // Usando la funcion implementada manualmente.
    // found.push(Gem::from_number(data).expect("Invalid Number"));

    // Usando al crate num-derive num-traits 
    found.push(Gem::from_u8(data).expect("Invalid Number"));
    
    println!("{found:?}")
}
