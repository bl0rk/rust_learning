// Disclaimer: This is basically a demonstration and is a bad idea. 

use std::rc::Rc; 
use std::cell::RefCell; 

pub struct List<T> { 
    head: Link<T>, 
    tail: Link<T> 
} 

type Link<T> = Option<Rc<RefCell<Node<T>>>>; 

struct Node<T> { 
    elem: T, 
    next: Link<T>, 
    prev: Link<T> 
} 

impl<T> Node<T> { 
    pub fn new(elem: T) -> Rc<RefCell<Self>> { 
        Rc::new(RefCell::new(Node { 
            elem, 
            prev: None, 
            next: None 
        })) 
    } 
} 

impl<T> List<T> { 
    pub fn new() -> Self { 
        List { head: None, tail: None } 
    } 

    pub fn push_front(&mut self, elem: T) { 
        // Initialise a new head (a Rc<RefCell<Node<T>>>). 
        let new_head = Node::new(elem); 

        // Match on self.head.take() which returns a Option containing either None or a Rc<RefCell<Node<T>>>. 
        // self.head will be None after the take() call. 
        match self.head.take() { 
            // If self.head is Some, put the value in the Some into old_head. 
            Some(old_head) => { 
                // Borrow a mutable reference to the owned value of old_head (which is a Node<T>) and set it's .prev property to a shared reference 
                // of new_head and wrap it in a Some. 
                old_head.borrow_mut().prev = Some(new_head.clone()); 
                // Borrow a mutable reference to the owned value of new_head (which is a Node<T>) and set it's .next property to a shared reference 
                // of old_head and wrap it in a Some. 
                new_head.borrow_mut().next = Some(old_head); 
                // Set self.head to a shared reference to new_head and wrap it in a Some. 
                self.head = Some(new_head); 
            }, 
            // If self.head was None which means the List was empty. 
            None => { 
                // Set self.tail to a shared refrence of new_head. 
                self.tail = Some(new_head.clone()); 
                // Set self.head to a shared reference of new_head. 
                self.head = Some(new_head); 
            } 
        } 
    } 

    pub fn pop_front(&mut self) -> Option<T> { 
        // Take the value out of self.head, which leaves a None in it's place. (self.head can either be None or Some(T), it either just returns None or T) 
        // Then map over the value. 
        self.head.take().map(|old_head| { 
            // match on the value within next which is mutably borrowed from old_head. This will either be None or Some(T). 
            match old_head.borrow_mut().next.take() { 
                // If there is a next node after the old head. 
                Some(new_head) => { 
                    // Take a mutable reference from new_head and take out the value in the .prev property without doing anything with it. 
                    // If .prev is a None nothing will happen, if it is a Some(T), or more accurately, a Some(Rc<...>) this operation would delete the shared 
                    // reference. This shared reference would be a reference to old_head. Since self.head() and now .prev here had their shared references removed 
                    // there are no references to the old head left except the one within this scope. 
                    new_head.borrow_mut().prev.take(); 
                    // Set self.head to new_head wrapped in a Some. 
                    self.head = Some(new_head); 
                }, 
                // If there is no node after the old head. 
                None => { 
                    // If there is no node after the old head, the list will be empty after this operation. As such self.tail needs to be removed. 
                    // This is done by taking the value out of the option from self.tail and doing nothing with the value which deletes the shared reference 
                    // as such the value within it would be dropped. 
                    self.tail.take(); 
                } 
            } 

            // Since old_head should have no other references left, the value within the Rc<...> can be taken out. This is done using Rc::try_unwrap() 
            // which returns the inner value, a RefCell<...>. .unwrap() cannot be used immeaditly because it needs to be able to debug print the 
            // error case and thus Node<T> would need to implement Debug. To get around this it's turned into an Option and then unwrapped. 
            // This leaves the RefCell<...> which inner value can be extracted using .into_inner() after which a Node<T> remains. 
            // The .elem property of the Node is returned. 
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem 
        }) 
    } 
} 