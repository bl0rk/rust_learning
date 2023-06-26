#![allow(unused_variables)] 

use std::ptr; 
use std::marker::PhantomData; 

// This is a very naive implementation of a BTree. 

// TODO: 
// - Make a BTreeMap version of this. Currently it behaves like a BTreeSet. 
// - Make design decision: Should the Vectors in the BTreeNodes have their max space pre-allocated to save allocation costs on insertion or not (as is right now) 
//   to save space. 
// - Maybe find a way to get rid of code repition in get_ref(). 
// - Maybe find a better array type then Vector<T> and maybe add field (is it called a field?) n to BTreeNode<T>. 

// Notes: 
// - get_mut() would be impossible at it's current state i think, since the value could be changed and thus mess up the ordering. Since this particular implementation 
//   (at the moment) would behave almost identical to a BTreeSet to a outside observer, get_mut() will not be implemented in this data structure. It will be implemented 
//   in a BTreeMap version of this datastructure and will then only be able to change the values, not the keys. 
// - Insertion implementation may be weird. If during insertion a node is made full by the insertion operation, this overflow will not be corrected until the next 
//   insertion that comes across the full node. 
// - There was a bug in remove() where a child was double dropped if the whole tree was dropped after the root was empty at the end of the remove algorithm, but while 
//   trying to drop that node again didn't work as it shouldn't exist anymore, it was somehow still possible to access the dropped child node. How this works i have no idea, to 
//   replicate this simply change the line `self.root = root.children.remove(0); ` to `self.root = root.children[0]; `in BTree::remove(). 

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

impl<T> BTree<T> where T: PartialOrd { 
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

    pub fn remove(&mut self, key: T) { 
        if self.root.is_null() { 
            return; 
        } 

        let root = unsafe { &mut (*self.root) }; 

        unsafe { 
            root.remove(key); 
        } 

        if root.keys.len() == 0 { 
            let old_root = self.root; 
            if root.leaf { 
                // The tree is empty. 
                self.root = ptr::null_mut(); 
            } else { 
                // Only the root is empty. 
                // BUG FIXED: child was not removed and so was double dropped. 
                self.root = root.children.remove(0); 
            } 

            drop(unsafe { Box::from_raw(old_root) }); 
        } 
    } 

    pub fn get_ref(&self, key: T) -> Option<&T> { 
        if self.root.is_null() { 
            return None; 
        } 

        let mut current_node = unsafe { &(*self.root) }; 

        loop { 
            if current_node.leaf { 
                for (i, k) in current_node.keys.iter().enumerate() { 
                    if *k == key { 
                        return Some(k); 
                    } 
                } 

                break;  
            } 

            let mut next = current_node.children.len()-1; 

            for (i, k) in current_node.keys.iter().enumerate() { 
                if *k == key { 
                    return Some(k); 
                } 

                if *k > key { 
                    next = i; 
                    break; 
                } 
            } 

            current_node = unsafe { &(*current_node.children[next]) } 
        } 

        None 
    } 
} 

impl<T> BTreeNode<T> where T: PartialOrd { 
    pub fn new(order: usize, leaf: bool) -> Self { 
        Self { 
            order, 
            keys: Vec::new(), // Maybe change keys and children to be at the max size immeaditly? Would save allocation costs but waste memory. 
            children: Vec::new(), 
            leaf 
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

    unsafe fn remove(&mut self, key: T) { 
        let n = self.keys.len(); 
        let mut idx: usize = 0; // Index at which points to either the keys index at which the to be removed key sits or to the subtree it should be in. 
        for k in self.keys.iter() { 
            if *k == key || *k > key { 
                break; 
            } 
            idx += 1; 
        } 

        if n != idx && self.keys[idx] == key { 
            if self.leaf { 
                // Remove from leaf. The Algorithm should have made sure that deleting out of this node is valid and as such no further action should be required. 
                self.keys.remove(idx); 
            } else { 
                let remove_next = self.remove_from_non_leaf(idx); 
                (*self.children[remove_next]).remove(key); 
            } 
        } else { 
            if self.leaf { 
                return; // Key is not in the tree. 
            } 

            // let flag = idx == n; // Probably not needed. If problems occur try uncommenting this and putting "flag &&" in the last if statement. 

            if (*self.children[idx]).keys.len() < self.order { 
                self.fill(idx); // Fill it up to `order` elements to allow for deletion. 
            } 

            if idx > self.keys.len() { 
                (*self.children[idx-1]).remove(key); 
            } else { 
                (*self.children[idx]).remove(key); 
            } 
        } 
    } 

    /// Helper method for deletion. When a key that is supposed to be removed is in an internal node\
    /// it can't just be removed because of the children nodes. A Key will have to replace the node\
    /// or the two children must merge. The key to be removed will be moved one node down. 
    unsafe fn remove_from_non_leaf(&mut self, idx: usize) -> usize { 
        // Switch the if and else if statements and their contents for a very small possible performance boost. 
        if (*self.children[idx]).keys.len() >= self.order { 
            let pred = self.get_pred(idx) as *mut _; // Absolutely a hack, should probably change this to something else. 
            std::mem::swap(&mut (*pred), &mut self.keys[idx]); 

            return idx; 
        } else if idx != self.keys.len() && (*self.children[idx+1]).keys.len() >= self.order { 
            let succ = self.get_succ(idx+1) as *mut _; // Same as above. 
            std::mem::swap(&mut (*succ), &mut self.keys[idx]); 

            return idx+1; 
        } else { 
            if idx == self.keys.len() { 
                self.merge(idx-1); 
            } else { 
                self.merge(idx); 
            } 
            return idx; 
        } 
    } 

    unsafe fn get_pred(&mut self, idx: usize) -> &mut T { 
        let mut current_node = &mut (*self.children[idx]); 
        loop { 
            if current_node.leaf { 
                return &mut current_node.keys[self.keys.len()-1]; 
            } 

            current_node = &mut (*current_node.children[current_node.keys.len()]); 
        } 
    } 

    unsafe fn get_succ(&mut self, idx: usize) -> &mut T { 
        let mut current_node = &mut (*self.children[idx]); 
        loop { 
            if current_node.leaf { 
                return &mut current_node.keys[0]; 
            } 

            current_node = &mut (*current_node.children[0]); 
        } 
    } 

    /// Helper method for deletion. If a node that is part of the subtree which a to be deleted node is in\
    /// has the minimum amount of elements `order-1` it needs to be filled up with at least one more key\
    /// to allow for deletion. 
    unsafe fn fill(&mut self, idx: usize) { 
        if idx != 0 && (*self.children[idx-1]).keys.len() >= self.order { 
            // Get key from the left sibling. 
            let child = &mut (*self.children[idx]); 
            let sibling = &mut (*self.children[idx-1]); 
            let sk_len = sibling.keys.len()-1; 

            // Swap the keys from the current node and ths sibling and then remove the wanted key out of the silbing. 
            std::mem::swap(&mut sibling.keys[sk_len], &mut self.keys[idx]); 
            child.keys.insert(0, sibling.keys.remove(sk_len)); 
        } else if idx != self.keys.len() && (*self.children[idx+1]).keys.len() >= self.order { 
            // Get key from the left sibling. 
            let child = &mut (*self.children[idx]); 
            let sibling = &mut (*self.children[idx+1]); 

            // Swap the keys from the current node and ths sibling and then remove the wanted key out of the silbing. 
            std::mem::swap(&mut sibling.keys[0], &mut self.keys[idx]); 
            child.keys.push(sibling.keys.remove(0)); 
        } else { 
            // Merge with one sibling. 
            if idx == self.keys.len() { 
                self.merge(idx-1); 
            } else { 
                self.merge(idx); 
            } 
        } 
    } 

    /// Merges the child nodes c\[idx] and c\[idx+1] with the current nodes key k\[idx]. 
    unsafe fn merge(&mut self, idx: usize) { 
        let child = &mut (*self.children[idx]); 
        // sibling will be dropped at the end of the method. 
        let mut sibling = *Box::from_raw(self.children.remove(idx+1)); 

        child.keys.push(self.keys.remove(idx)); 
        child.keys.append(&mut sibling.keys); 

        if !child.leaf { 
            child.children.append(&mut sibling.children); 
        } 
    } 
} 

impl<T> Drop for BTree<T> { 
    fn drop(&mut self) { 
        if self.root.is_null() { 
            return; 
        } 

        let root = unsafe { Box::from_raw(self.root) }; 
        drop(root); 
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

// For debugging. 
struct TraverseResult<T> { 
    levels: Vec<Vec<Vec<T>>> 
} 

impl<T> TraverseResult<T> where T: std::fmt::Debug { 
    pub fn new() -> Self { 
        Self { levels: vec![] } 
    } 

    pub fn insert(&mut self, level: usize, values: Vec<T>) { 
        if level > self.levels.len() { 
            return; 
        } 

        if level == self.levels.len() { 
            self.levels.push(vec![values]); 
            return; 
        } 

        self.levels[level].push(values); 
    } 

    pub fn print_all(&self) { 
        for (i, level) in self.levels.iter().enumerate() { 
            println!("{i}: {:?}", level); 
        } 
    } 
} 

impl<T> BTree<T> where T: PartialOrd + std::fmt::Debug + Clone { 
    pub fn debug_traverse(&self) { 
        // Mostly used for debugging. 
        if self.root.is_null() { 
            println!("The BTree is empty."); 
        } 

        let layer: usize = 0; 
        let mut traverse: TraverseResult<T> = TraverseResult::new(); 

        unsafe { 
            // for key in &(*self.root).keys { 
            //     traverse.insert(layer, key.clone()); 
            // } 
            traverse.insert(layer, (*self.root).keys.clone()); 

            for child in (*self.root).children.iter() { 
                (**child)._traverse(&mut traverse, layer); 
            } 
        } 

        traverse.print_all(); 
    } 
} 

impl<T> BTreeNode<T> where T: PartialOrd + std::fmt::Debug + Clone { 
    fn _traverse(&self, traverse: &mut TraverseResult<T>, mut layer: usize) { 
        layer += 1; 

        traverse.insert(layer, self.keys.clone()); 

        if self.leaf { 
            return; 
        } 

        for child in self.children.iter() { 
            unsafe { 
                (**child)._traverse(traverse, layer); 
            } 
        } 
    } 
} 