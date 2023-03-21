

fn main() {
    println!("Hello!"); 
    
    //1_ created a MyBox and applied Deref to make it dereferenceable.
    box_is_dereferenceable();

    //2_ testing the usage of the Drop trait and how to apply it to a certain custom data Type.
    testing_drop_trait();
    
    //3_ testing the Rc<T> smart pointer which keeps track of the amount of references
    //to a value. It also allow multiple owners to share ownership of said value.
    //It also won't run the Drop trait if there are references still active.  
    rc_insteadof_box();
}

//3_
fn rc_insteadof_box() {
    use List::{Cons,Nil};
    use std::rc::Rc;

    enum List<T> {
        Cons(T, Rc<List<T>>),
        Nil,
    }

    let a = Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))));
    let b = Cons(1, Rc::clone(&a));
    let c = Cons(1, Rc::clone(&a));
}
//2_
fn testing_drop_trait() {
    //Crate should use snake_case.
    use Crate::CustomSmartPointer;
    use colored::*;

    let new_smart_pointer = CustomSmartPointer {
        data: String::from("Some data that inside the pointer."),
    };
    let second_smart_pointer = CustomSmartPointer {
        data: String::from("Data."),
    };

}
//1_
fn box_is_dereferenceable() { 
    use Crate::MyBox;

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(x, *y)
}


