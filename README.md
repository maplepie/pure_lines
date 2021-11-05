# pure_lines

A tool that beautify multiple lines.
* Trim first line if empty
* Trim min indent
  
## Problem

What you need str:
``` rust
hello
world
```
You had to do:
``` rust
let s = "hello
world";
println!("{}",s);
```
Now you can:
``` rust
let s = "
    hello
    world";
println!("{}",pure_lines::pure(s));
```

## Example
### Basic Example
``` rust
use pure_lines;

fn main(){
    let a = "
    hello
    world";
    println!("==============");
    println!("before:");
    println!("{}",&a);
    println!("==============");
    let s = pure_lines::pure(a);
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
    // hello
    // world
    // ==============
}
```  
### Example with prefix
``` rust
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
```
  
## Advance
If you want your code more faster, using the `quick` feature.
quick feature trim indent based on the first line which is not empty.
> ⚠Note: make sure the after lines indent must bigger than the first，or you will lose some string.

your `Cargo.toml` could look like this:
``` toml
[dependencies]
pure_lines = { version = "0.2.0", features = ["quick"] }
```
