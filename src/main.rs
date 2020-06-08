
fn main() {
    // println!("Hello, world!");
    // println!("It's weird that println has an exclamation mark after it");
    // comments();
    formatted_print();
}


/*
* 1.1 comments
 */
#[allow(dead_code)]
fn comments(){
    let x= 5+ /* *90 + */ 5;
    println!("x = {}", x);
}

/*
* 1.2 formatted print
 */
#[allow(dead_code)]
fn formatted_print(){
    let str = format!("{} is a number", 35); //without using format! here, everything falls apart. str becomes an i32... for some reason. primatives are the next chapter though so hopefully it will make sense then. Wow I probably should have used a block comment for this, I'm sure it looks really bad.
    // println!(str);  //Apparently this is not kosher
    println!("{}", str); //this is fine though


    //you can specify argument positions in a string format, which is nifty
    println!("this is the {0}st argument, this is the {1}nd, and this is a picture of the first time they met: {0}{1}", 1,2);

    //there is special formatting but the book doesn't explain this so I only have a guess as to what this does before running it, but probably won't update this comment
    println!("{} of {:b} people know binary, the other half doesn't", 1,2);
    //ok idea for multi million dollar app: decimal to binary conversion app
    println!("{:b} dollars", 3456789); //apparently I need a bigger bank account to store all those $$$, this one didn't hold as much as I thought it would

    //padding a number with leading zeroes
    println!{"{number:>0width$}",number = 1, width=6}; //apparently you can also use the names for arguments and not just positions (app idea: create a counters for occurances of `apparently` in these comments)

    println!("My name is {0}, {1} {0}", "Bond", "James");


    /*
    * Add a println! macro that prints: Pi is roughly 3.142 by controlling the number of decimal places shown. For the purposes of this exercise,
    * use let pi = 3.141592 as an estimate for pi. (Hint: you may need to check the std::fmt documentation for setting the number of decimals to display)
     */
    let pi = 3.141592;
    println!("Pi is roughly {:.3}",pi);

}

