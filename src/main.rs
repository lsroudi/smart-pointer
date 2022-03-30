fn main() {
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
