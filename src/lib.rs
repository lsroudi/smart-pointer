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