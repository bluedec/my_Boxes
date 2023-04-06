

fn main() {
    println!("Hello!"); 
    
    println!("--------------------");
    //1_ created a MyBox and applied Deref to make it dereferenceable.
    box_is_dereferenceable();


    println!("--------------------");
    //2_ testing the usage of the Drop trait and how to apply it to a certain custom data Type.
    testing_drop_trait();
        
    println!("--------------------");
    //3_ testing the Rc<T> smart pointer which keeps track of the amount of references
    //to a value. It also allow multiple owners to share ownership of said value.
    //It also won't run the Drop trait if there are references still active.  
    rc_insteadof_box();

    println!("--------------------");
    //4_ seeing how the reference count increases with each creation and drop of a reference to a
    //value, in this case the value is a Cons list.
    counting_strong_references();
}
//4
fn counting_strong_references() {
    use Crate::List::{Cons, Nil};
    use std::rc::Rc;

    let a = Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))));
    println!("count after initializing a is: {}", Rc::strong_count(&a));

    let b = Cons(1, Rc::clone(&a));
    println!("count after initializing b is: {}", Rc::strong_count(&a));
    {
        let c = Cons(1, Rc::clone(&a));
        println!("count after initializing c is: {}", Rc::strong_count(&a));

    }
    println!("count after c went out of scope: {}", Rc::strong_count(&a));

}

//3_
fn rc_insteadof_box() {
    use Crate::List::{Cons,Nil};
    use std::rc::Rc;

    let a = Rc::new(Cons(2, Rc::new(Cons(4, Rc::new(Nil)))));
    let b = Cons(1, Rc::clone(&a));  
    let c = Cons(2, Rc::clone(&a));
}
//2_
fn testing_drop_trait() {
    //Crate should use snake_case.
    use Crate::CustomSmartPointer;

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


