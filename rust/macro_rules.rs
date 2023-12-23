// There are three ways why your should use
// macro can help you use special syntax into source code.
// avoid repeat code.
// could accept a number of argument.
//
//
// `() indicates that the macro takes no argument.`
macro_rules! say_hello{
	() => {
		println!("hello world");
	}
}

macro_rules! args {
    () => {
       macro_rules!(); 
    }
}

fn main(){
	say_hello!();
    args!();
}
