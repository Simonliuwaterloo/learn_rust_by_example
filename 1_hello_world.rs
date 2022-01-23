// comment, just like c

#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable
// also.
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}


//main() function, just like c
fn main() {
    println!("Hello World"); // I guss ';' is needed, just like c
    println!("I'm a Rustacean!");
    // Formatted print
    println!("I was born in {}", 1994);
    println!("{0} is red, {1} is yellow. {1} tastes sour, {0} tastes sweet", "Apple", "Lemon");
    println!("the {object} at {place} is {adj}.",
            object="Christmas Tree",
            place="Rockefeller Center",
            adj="big");
    println!("{} of {:b} people knows binary, the hald half doesn't",
            1, 2); // special formatting
    println!("{number:>width$}", number=1, width=6);
    println!("{number:>width$}", number=16, width=6); // right align text
    println!("{number:0>width$}", number=16, width=6); // pad numbers with 0
    println!("{number:z>width$}", number=16, width=6); // pad numbers with z

    println!("My name is {0}, {1} {0}", "Bond", "James");

    let pi = 3.141592;
    println!("Hello {0} is {1:.3}", "pi", pi);

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));
    
    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}