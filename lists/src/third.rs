
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    
    pub fn new() -> Self {
        List {head: None }
    }

    pub fn append(&self, elem: T) -> List<T> {
        List {head: Some(Rc::new(Node {
            elem: elem,
            next: self.head.clone(),
        }))}
    }

    pub fn tail(&self) -> List<T> {
        List {head: self.head.as_ref().and_then(|node| node.next.clone())}
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }


}


/*
Crappy version of drop that only runs in O(n) time.
impl Drop<T> for List<T> {
    fn drop(&mut self) {
        //steal list's head
        let mut cur_list = self.head.take();

        while let Some(node) = cur_list {
            // clone's the current node's next node
            cur_list = node.next.clone();
            // drops the node here if it had refcount 1
            // since we have a reference to it's next node it won't 
            // recurse and drop its child
        }
    }
}
*/


// amortized O(1) solution for drop but only works on nightly build
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            }else {
                break;
            }
        }
    }
}



pub struct Iter<'a, T:'a> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {next: self.head.as_ref().map(|node| &**node) }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.append(1)
                        .append(2)
                        .append(3);

        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

       
        let list = list.tail();
        assert_eq!(list.head(), None);
        
        //make sure it works when list is already empty too
        let list = list.tail();
        assert_eq!(list.head(), None);
        
    }
    
    #[test]
    fn iter() {

        let list = List::new().append(1)
                              .append(2)
                              .append(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));

        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);


    }
}

