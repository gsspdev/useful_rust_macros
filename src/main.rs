fn main() {
    println!("x is invoked like x![adsr] and expands to");
    println!("mod adsr;");
    println!("pub use adsr::*;");

// use this to make a lib.rs file with all modules declared by x![] macros
// use the ix![] macro to import all other files in the lib.rs file

#[macro_export] macro_rules! x { 
    ($x:ident) => { 
        mod $x; 
        pub use $x::*; 
    }
// Macro expansion:
//    mod adsr;
//    pub use adsr::*;
}

#[macro_export] macro_rules! xt { 
    ($x:ident) => { 
        #[cfg(test)]
        mod $x; 
    }
// Macro expansion:
//       #[cfg(test)]
//        mod adsr;
}

#[macro_export] macro_rules! ix { 
    () => { 
        use crate::{ 
            imports::* , 
        };
        use crate::*;
    } 
// Macro expansion:
//     use crate::{
//         imports::*,
//     };
//     use crate::*
}
