use deeply::nested::function as other_function;
fn function(){
    println!("Called function");
}
mod deeply{
    pub mod nested{
        pub fn function(){
            println!("Called deeply::nested::function()");
        }
    }
}
fn main() {
    //easier access to deeply::nested::function
    other_function();
    println!("Entering block:");
    {
        //this is equivalent to using 'use deeply::nested::function'
        //this function will shadow the outer one
        use crate::deeply::nested::function;
        //use bindings have local scope, in this case the shadowing of 'function()' is only this block
        function();
        println!("Leaving this block");
    }
    function();
}
