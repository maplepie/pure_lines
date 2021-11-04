use pure_lines;

fn main(){
    let a = "
    hello
    world";
    println!("==============");
    println!("before:");
    println!("{}",&a);
    println!("==============");
    let s = pure_lines::pure_with(a,"> ");
    println!("after:");
    println!("{}",s);
    println!("==============");

    // Output:
    // ==============
    // before:
    //
    //     hello
    //     world
    // ==============
    // after:
    // > hello
    // > world
    // ==============
}