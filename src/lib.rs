pub fn recursive(){
    let b = Box::new(5);
    println!("b = {}", b);

    // enum List {
    //     Cons(i32, List),
    //     Nil,
    // }
    // use crate::List::{Cons, Nil};
    // let list = Cons(1, Cons(2, Cons(3, Nil)));

    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    use List::{Cons, Nil};
    let list = Cons(1,
                    Box::new(Cons(2,
                        Box::new(Cons(3,
                            Box::new(Cons(4,
                                Box::new(Nil))))))));
    println!("list = {:?}", list);
}

pub fn dereference_opertator(){
    let x = 5;
    let y = &x;
    assert_eq!(5,x);
    assert_eq!(5,*y);

    // using Box
    let z = 5;
    let w = Box::new(z);
    assert_eq!(5,*w);
}

pub fn my_box_type(){
    struct MyBox<T>(T);

    impl<T> MyBox<T>  {
        fn new(x:T) -> MyBox<T> {
            MyBox(x)
        }
    }

    let x = 5;
    let y = &x;

    let y = MyBox::new(x);

    assert_eq!(5, x);
    //type `MyBox<{integer}>` cannot be dereferenced
    //assert_eq!(5, *y);

    // to be able to dereference y, MyBox should implement the Dref trait
    use std::ops::Deref;

    impl<T> Deref for MyBox<T>{
    
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }
    assert_eq!(5, *y);
}

pub fn drop_method(){
    struct CustomSmartPointer {
        data : String,
    }

    impl Drop for CustomSmartPointer{

        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    let c = CustomSmartPointer{ data: String::from("my stuff")};
    let d = CustomSmartPointer{data: String::from("other stuff")};
    println!("CustomSmartPointers created.");

    drop(d);
    println!("D is dropped before the end of the function");

}

pub fn rc_method(){
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    
    use List::{Cons, Nil};
    
    
    let a = Cons(5,
        Box::new(Cons(10,
            Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    //Demonstrating we’re not allowed to have two lists using Box<T> that try to share ownership of a third list
    //let c = Cons(4, Box::new(a));

}

pub fn rc_demo(){

    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }
    use List::{Cons, Nil};
    use std::rc::Rc;
    // Output
    // count after creating a = 1
    // count after creating b = 2
    // count after creating c = 3
    // count after c goes out of scope = 2
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

//interior_mutability
    pub trait Messenger {
        fn send(&self, msg:&str);
    }
    pub struct LimitTracker<'a, T: 'a + Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }
    
    impl<'a, T> LimitTracker<'a, T>
        where T: Messenger {
        pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }     
        pub fn set_value(&mut self, value: usize) {
            self.value = value;
        
            let percentage_of_max = self.value as f64 / self.max as f64;
        
            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                    self.messenger.send("Urgent warning: You've used up over 90% of
                    your quota!");
                } else if percentage_of_max >= 0.75 {
                    self.messenger.send("Warning: You've used up over 75% of your
                    quota!");
                }
        }
    }

    #[cfg(test)]
    mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
    fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
        fn it_sends_an_over_75_percent_warning_message() {
            let mock_messenger = MockMessenger::new();
            let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

            limit_tracker.set_value(80);

            assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
            }
    }

