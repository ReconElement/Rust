fn function(){
    println!("Called function");
}
mod cool{
    pub fn function(){
        println!("Called cool::function()");
    }
}

mod my{
    fn function(){
        println!("Called my::function()");
    }
    mod cool{
        pub fn function(){
            println!("Called my::cool::function()");
        }
    }
    pub fn indirect_call(){
        //let's access all the function named as function from this scope!
        println!("Called my::indirect_call");
        //the 'self' keyword refers to the current module scope - in this case 'my'
        //calling self::function() and calling function() both give directly the same result because they refer to the same function
        self::function();
        function();
        //we can also use self to access another module inside 'my'
        self::cool::function();
        //The 'super' keyword refers to the parent scope outside 'my' module
        super::function();
        //This will bind to the cool::function() in the crate scope
        //in this case crate scope is the outermost scope
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}
fn main(){
    my::indirect_call();
}