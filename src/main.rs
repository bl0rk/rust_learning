#![allow(dead_code)] 

mod change_filetype; // Misleading name to be honest. 
mod sorting_testing; 
mod linked_lists; // From a book, not self made. (mod.rs file in linked_lists folder) 
mod b_tree; 

fn main() { 
    // filetype(); 
    let mut btree = b_tree::BTree::new(2); 
    for x in 0..10 { 
        btree.insert(x); 
    } 

    btree.debug_traverse(); 
    println!("---"); 
    btree.remove(0); 
    btree.debug_traverse(); 
} 

fn filetype() { 
    change_filetype::change_filetype_run(); 
} 