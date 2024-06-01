//a module named my_mod
mod my_mod{
    //Items in modules default to private visibility
    fn private_function(){
        println!("Called my_mod::private_function()");
    }
    //use the 'pub' modifier to override default visibility
    pub fn function(){
        println!("Called my_mod::function()");
    }
    //Items can access other items in the same module, even when private, item here can refer to anything whether a struct, function, impl method etc
    pub fn indirect_access(){
        println!("Called my_mod indirect access is being undertaken here: my_mod::indirect_access()");
        private_function();
    }
    //modules can also be nested
    pub mod nested{
        pub fn function(){
            println!("Called my_mod::nested::function()");
        }

        #[allow(dead_code)]
        fn private_function(){
            println!("Called my_mod::nested::private_function()");
        }
        //functions declared using 'pub(in path) syntax are only visible within the given path where path must be a parent or ancestor module
        pub(in crate::my_mod) fn public_function_in_my_mod(){
            println!("Called my_mod::nested::public_function_in_my_mod()");
            public_function_in_nested();
        }
        //functions declared using pub(self) are only visible within the current module which is the same as leaving them private
        pub(self) fn public_function_in_nested(){
            println!("Called my_mod::nested::public_function_in_nested()");
        }
        //functions declared using pub(super) are only visible within the parent module
        pub(super) fn public_function_in_super_mod(){
            println!("Called my_mod::nested::public_function_in_super_mod()");
        }
    }
    pub fn call_public_function_in_my_mod(){
        println!("Called my_mod::call_public_function_in_my_mod()");
        nested::public_function_in_my_mod();
        println!(">");
        nested::public_function_in_super_mod();
    }
    //pub(crate) makes the function visible from anywhere but only within the current crate
    pub(crate) fn public_function_in_crate(){
        println!("Called my_mod::public_function_in_crate()");
    }
    //nested modules follow the same rules of visibility
    mod privately_nested{
        #[allow(dead_code)]
        pub fn function(){
            println!("Called my_mod::privately_nested::function()");
        }
        //private parent items will still restrict the visibility of child items even if they're still declared as visible within the larger context
        #[allow(dead_code)]
        pub(crate) fn restricted_function(){
            println!("Called my_mod::privately_nested::restricted_function()");
        }
    }
}
fn function(){
    println!("Called function");
}
fn main() {
    //modules allow disambiguation between items that have the same name
    function();
    my_mod::function();
    //public items including those inside nested modules can be accessed from outside the parent module
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();
    //pub(crate) items can be called from anywhere in the same crate
    my_mod::public_function_in_crate();
}
