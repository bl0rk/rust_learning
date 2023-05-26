pub struct List<T> { 
    head: Option<Box<Node<T>>> 
} 

impl<T> List<T> { 
    pub fn new() -> Self { 
        Self { 
            head: None 
        } 
    } 

    pub fn push(&mut self, elem: T) { 
        let node = Box::new(Node { 
            elem, 
            // Replace self.head with None and set next to the old value of self.head. 
            // next: mem::replace(&mut self.head, None) 
            next: self.head.take() // Same as the mem::replace above. 
        }); 

        self.head = Some(node); 
    } 

    pub fn pop(&mut self) -> Option<T> { 
        // Replaces head with None and returns the head, which is then matched. 
        // match mem::replace(&mut self.head, None) { 
        // self.head.take() is the same as the replace above. 
        // match self.head.take() { 
        //     None => None, 
        //     // If head was a node and not Empty. `node` is the previous head. 
        //     Some(node) => { 
        //         // Replace the current head with the previous's next which is either None or Some(node). 
        //         self.head = node.next; 
        //         // Return the previous's head elem. 
        //         Some(node.elem) 
        //     } 
        // } 

        // - 

        // take out of self.head (leaving None in place), then map it. if take() returns None, map() will return None as well 
        // If take() returns Some(T) map() will take ownership of T, the closure inside map() will take T (called node), set 
        // self.head to node.next (which will be either None or Some(Box<Node<T>>)), after which it will return node.elem. 
        // map() will return the value returned by the given closure. 
        self.head.take().map(|node| {
            self.head = node.next; 
            node.elem 
        }) 
    } 

    pub fn peek(&self) -> Option<&T> { 
        self.head.as_ref().map(|node| { 
            &node.elem 
        }) 
    } 

    pub fn peek_mut(&mut self) -> Option<&mut T> { 
        self.head.as_mut().map(|node| { 
            &mut node.elem 
        }) 
    } 
} 

impl<T> Drop for List<T> { 
    fn drop(&mut self) { 
        // let mut cur_link = mem::replace(&mut self.head, None); 
        let mut cur_link = self.head.take(); // Same as the mem::replace above. 
        // `while let` == "do this thing until this pattern doesn't match" 
        while let Some(mut boxed_node) = cur_link { 
            // cur_link = mem::replace(&mut boxed_node.next, None); 
            cur_link = boxed_node.next.take(); // Same as the mem::replace above. 
            // boxed_node goes out of scope and gets dropped here; 
            // but its Node's `next` field has been set to None 
            // so no unbounded recursion occurs. 
        } 
    } 
} 

struct Node<T> { 
    elem: T, 
    next: Option<Box<Node<T>>> 
} 

pub struct IntoIter<T>(List<T>); 

impl<T> List<T> { 
    pub fn into_iter(self) -> IntoIter<T> { 
        IntoIter(self) 
    } 
} 

impl<T> Iterator for IntoIter<T> { 
    type Item = T; 
    fn next(&mut self) -> Option<Self::Item> { 
        self.0.pop() 
    } 
} 

pub struct Iter<'a, T> { 
    next: Option<&'a Node<T>> 
} 

impl<T> List<T> { 
    pub fn iter(&self) -> Iter<'_, T> { 
        Iter { 
            // next: self.head.as_ref().map(|node| &**node) 
            next: self.head.as_deref() 
        } 
    } 
} 

impl<'a, T> Iterator for Iter<'a, T> { 
    type Item = &'a T; 

    fn next(&mut self) -> Option<Self::Item> { 
        self.next.map(|node| { 
            // self.next = node.next.as_ref().map(|node| &**node); 
            self.next = node.next.as_deref(); 
            &node.elem 
        }) 
    } 
} 

pub struct IterMut<'a, T> { 
    next: Option<&'a mut Node<T>>, 
} 

impl<T> List<T> { 
    pub fn iter_mut(&mut self) -> IterMut<'_, T> { 
        IterMut { next: self.head.as_deref_mut() } 
    } 
} 

impl<'a, T> Iterator for IterMut<'a, T> { 
    type Item = &'a mut T; 

    fn next(&mut self) -> Option<Self::Item> { 
        // self.next is a Option<&mut Node<T>>, .take() takes the mutable reference out of it, which is then mapped over 
        self.next.take().map(|node| { 
            // self.next (so the next node which should be accessed after this one) is set to the current node's next, which can be none or Some(&mut T) 
            self.next = node.next.as_deref_mut(); 
            // return a mutable reference to the element of the current node 
            &mut node.elem 
        }) 
    } 
} 