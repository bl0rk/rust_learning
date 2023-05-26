use std::ptr; 

pub struct BTree<T> { 
    root: *mut BTreeNode<T>, 
    order: usize 
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

        Self { root: ptr::null_mut(), order } 
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

    pub fn traverse(&self) -> String { 
        // Mostly used for debugging. 
        if self.root.is_null() { 
            return String::from("The BTree is empty."); 
        } 

        let layer: u32 = 0; 
        let mut to_append = String::new(); 

        unsafe { 
            to_append.push_str(&format!("root; keys: {:?}", (*self.root).keys)); 
            // println!("root; keys: {:?}", (*self.root).keys); 
            // println!("root; children: {:?}", (*self.root).children.len()); 

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
        // println!("keys {:?}; layer {}", self.keys, layer); 

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
                    insert_i += 1 ;
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
        let drain_iter = (*child).keys.drain(middle+1..ck_len); 

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

        // Insert into the right places. Well i hope they're the right places at least. 
        self.keys.insert(index, middle_key); 
        // self.children.insert(index, child); 
        self.children.insert(index+1, Box::into_raw(Box::new(right_child))); 
    } 
} 

impl<T> Drop for BTree<T> { 
    fn drop(&mut self) { 
        if self.root.is_null() { 
            return; 
        } 

        unsafe { 
            let _root = Box::from_raw(self.root); 
            drop(_root); 
        } 
    } 
} 

impl<T> Drop for BTreeNode<T> { 
    fn drop(&mut self) { 
        for i in 0..self.children.len() { 
            // Safety check, as far as i know (which isn't a lot to be fair) there shouldn't be any null pointer in the children vector but better to be safe. 
            if self.children[i].is_null() { 
                continue; 
            } 

            unsafe { 
                let node = Box::from_raw(self.children[i]); 
                // Since i don't know a lot about recursive things, i'm just gonna hope this can't blow the stack. 
                drop(node); 
            } 
        } 
    } 
} 