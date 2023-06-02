#![allow(dead_code)] 

mod change_filetype; // Misleading name to be honest. 
mod sorting_testing; 
mod linked_lists; // From a book, not self made. (mod.rs file in linked_lists folder) 
mod b_tree; 

fn main() { 
    filetype(); 
} 

fn filetype() { 
    change_filetype::change_filetype_run(); 
} 