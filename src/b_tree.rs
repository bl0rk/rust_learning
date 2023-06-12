#![allow(unused_variables)] 

use std::ptr; 
use std::marker::PhantomData; 

// TODO: 
// - Add deletion. 
// - Make a BTreeMap version of this. Currently it behaves like a BTreeSet. 
// - Make design decision: Should the Vectors in the BTreeNodes have their max space pre-allocated to save allocation costs on insertion or not (as is right now) 
//   to save space. 
// - Remove hacks for get_ref(). 

// Notes: 
// - get_mut() would be impossible at it's current state i think, since the value could be changed and thus mess up the ordering. Since this particular implementation 
//   (at the moment) would behave almost identical to a BTreeSet to a outside observer, get_mut() will not be implemented in this data structure. It will be implemented 
//   in a BTreeMap version of this datastructure and will then only be able to change the values, not the keys. 

pub struct BTree<T> { 
    root: *mut BTreeNode<T>, 
    order: usize, 
   _boo: PhantomData<T>  // No idea if this is needed tbh. But it shouldn't matter (i think) since this should only be for the compiler. 
} 

struct BTreeNode<T> { 
    order: usize, // Minimum Degree. 
    keys: Vec<T>, 
    children: Vec<*mut BTreeNode<T>>, 
    leaf: bool 
} 

impl<T> BTree<T> where T: PartialOrd + std::fmt::Debug { 
    /// Creates a new empty BTree. `order` must be greater or equal to 2. 
    /// # Example 
    /// ```
    /// let mut btree = BTree::new(2); 
    /// ```
    pub fn new(order: usize) -> Self { 
        if order < 2 { 
            panic!("BTree order must be equal to or greater than 2."); 
        } 

        Self { root: ptr::null_mut(), order, _boo: PhantomData } 
    } 

    pub fn insert(&mut self, key: T) { 
        // This implies that i thoroughly checked that the code here is safe to use. Have i done that? 
        // Who knows. :3 
        unsafe { 
            // If the tree is empty. 
            if self.root.is_null() { 
                let mut new_root = BTreeNode::new(self.order, true); 
                new_root.keys.push(key); 
                self.root = Box::into_raw(Box::new(new_root)); 
                return; 
            } 

            // If the root is full. 
            if (*self.root).keys.len() == (self.order*2-1).into() { 
                // The root is full, as such the tree must grow. 
                let mut new_root: BTreeNode<T> = BTreeNode::new(self.order, false); 

                new_root.children.push(self.root); 
                new_root.split_child(0); 

                self.root = Box::into_raw(Box::new(new_root)); 
            } 

            (*self.root).insert_non_full(key); 
        } 
    } 

    pub fn remove(&mut self, key: T) -> Option<T> { 
        if self.root.is_null() { 
            return None; 
        } 

        todo!(); 
    } 

    pub fn get_ref(&self, key: T) -> Option<&T> { 
        if self.root.is_null() { 
            return None; 
        } 

        let mut current_node = unsafe { &(*self.root) }; 

        loop { 
            let mut next = current_node.children.len(); 

            for (i, k) in current_node.keys.iter().enumerate() { 
                if *k == key { 
                    return Some(k); 
                } 

                if *k > key { 
                    next = i+1; 
                } 
            } 

            if current_node.leaf { 
                break; 
            } 

            // Note: next-1 as well as next = i+1 are both hacks to get around overflow. 
            current_node = unsafe { &(*current_node.children[next-1]) } 
        } 

        None 
    } 

    pub fn traverse(&self) -> String { 
        // Mostly used for debugging. 
        if self.root.is_null() { 
            return String::from("The BTree is empty."); 
        } 

        let layer: u32 = 0; 
        let mut to_append = String::new(); 

        unsafe { 
            to_append.push_str(&format!("root; keys: {:?}", (*self.root).keys)); 

            for child in (*self.root).children.iter() { 
                (**child)._traverse(&mut to_append, layer); 
            } 
        } 

        return to_append; 
    } 
} 

impl<T> BTreeNode<T> where T: PartialOrd + std::fmt::Debug { 
    pub fn new(order: usize, leaf: bool) -> Self { 
        Self { 
            order, 
            keys: Vec::new(), 
            children: Vec::new(), 
            leaf 
        } 
    } 

    fn _traverse(&self, to_append: &mut String, mut layer: u32) { 
        layer += 1; 

        to_append.push_str(&format!("\nkeys {:?}; layer {}", self.keys, layer)); 

        if self.leaf { 
            return; 
        } 

        for child in self.children.iter() { 
            unsafe { 
                (**child)._traverse(to_append, layer); 
            } 
        } 
    } 

    unsafe fn insert_non_full(&mut self, insert_key: T) { 
        // Note: It is assumed that the node this method is called on is not full. 
        if self.leaf { 
            // Insert key at correct position. Don't have to care about children since this is a leaf node. 
            for i in 0..self.keys.len() { 
                if self.keys[i] > insert_key { 
                    self.keys.insert(i, insert_key); 
                    return; 
                } 
            } 

            self.keys.insert(self.keys.len(), insert_key); 
        } else { 
            // Look for appropiate child to insert into. 
            let mut insert_i = self.keys.len(); 

            for (i, k) in self.keys.iter().enumerate() { 
                if *k > insert_key { 
                    insert_i = i; 
                    break; 
                } 
            } 

            if (self.order*2)-1 == (*self.children[insert_i]).keys.len() { 
                // If the selected child is full. 
                self.split_child(insert_i); 

                if self.keys[insert_i] < insert_key { 
                    insert_i += 1; 
                } 
            } 

            (*self.children[insert_i]).insert_non_full(insert_key); 
        } 
    } 

    unsafe fn split_child(&mut self, index: usize) { 
        // index is the index inside the children vector where the child to be split is at 
        let child = self.children[index]; 
        // it is assumed that the child is full 
        // child will become the left child of the key that shall travel up 

        let mut right_child: BTreeNode<T> = BTreeNode::new(self.order, (*child).leaf); 

        // index of the middle 
        let ck_len = (*child).keys.len(); 
        let middle = ck_len / 2; 

        // Need the last keys after the middle for the right child. 
        let drain_iter = (*child).keys.drain(middle+1..ck_len); // Check if there is a better and more efficent way to do this. 

        for key in drain_iter { 
            right_child.keys.push(key); 
        } 

        if !(*child).leaf { // If the child is a not a leaf node, transfer all right children to the new right child. 
            // Same as above only for children. 
            let drain_iter = (*child).children.drain(middle+1..=ck_len); 

            for d_child in drain_iter { 
                right_child.children.push(d_child); 
            } 
        } 

        // The key that will be moved up. 
        let middle_key = (*child).keys.remove(middle); 
        
        self.keys.insert(index, middle_key); 
        self.children.insert(index+1, Box::into_raw(Box::new(right_child))); 
    } 

    unsafe fn delete(&mut self, key: T) -> Option<T> { 
        todo!(); 
    } 
} 

impl<T> Drop for BTree<T> { 
    fn drop(&mut self) { 
        if self.root.is_null() { 
            return; 
        } 

        let _root = unsafe { Box::from_raw(self.root) }; 
        drop(_root); 
    } 
} 

impl<T> Drop for BTreeNode<T> { 
    fn drop(&mut self) { 
        // Could do a self.children.map() and while let thingy but idk if that'd make sense. 
        for child in self.children.iter() { 
            let node = unsafe { Box::from_raw(*child) }; 
            // Since i don't know a lot about recursive things, i'm just gonna hope this can't blow the stack. 
            drop(node); 
        } 
    } 
} 