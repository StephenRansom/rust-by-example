/**
* If you're reading this, you must be very bored. There's not really much substantive in this pile of code
* I'm just learning rust syntax in my free time, and creating random puddles of code just for practice.
* check out https://doc.rust-lang.org/stable/rust-by-example/ if you want to learn rust, because this
* program is likely not going to help you.
*/


fn main() {
    // println!("Hello, world!");
    // chapter_one::comments();
    // chapter_one::formatted_print();
    // chapter_two::scalar_types();
    chapter_two::compound_types();
}



mod chapter_one{
    /*
* 1.1 comments
 */
    #[allow(dead_code)]
    pub fn comments() -> (){
        let x= 5+ /* *90 + */ 5;
        println!("x = {}", x);
    }

    /*
    * 1.2 formatted print
     */
    #[allow(dead_code)]
    pub fn formatted_print() -> (){
        let str = format!("{} is a number", 35); //without using 'format!' here, everything falls apart. str becomes an i32... for some reason. primatives are the next chapter though so hopefully it will make sense then. Wow I probably should have used a block comment for this, I'm sure it looks really bad.
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
}

#[allow(dead_code)]
mod chapter_two{
    pub fn scalar_types(){
        print_ints();
        print_floats();
        print_char_bools();
        literals_and_operators();
    }

    pub fn compound_types(){
        array_stuff();
        tuple_trouble();
    }

    fn print_ints(){
        let eight: i8 = 127;
        let ueight = 255u8;  //suffix annotation, do not like
        let sixteen:i16 = 32767;
        let usixteen:u16= 65535;
        let thirty_two:i32 = 2147483647;
        let uthirty_two:u32 = 4294967295;
        let sixty_four:i64=9223372036854775807;
        let usixty_four:u64=18446744073709551615;
        let one_twenty_eight:i128 = 170141183460469231731687303715884105727;
        let u_one_twenty_eight:u128 = 170141183460469231731687303715884105727*2;
        //TODO format this without using a ton of tabs
        println!("Integers and max (positive) values");
        println!("\t signed\t\t\t\t\t\t\t\t\t\tunsigned\t");
        println!("8\t {}\t\t\t\t\t\t\t\t\t\t{}\t",eight,ueight);
        println!("16\t {}\t\t\t\t\t\t\t\t\t\t{}\t", sixteen, usixteen);
        println!("32\t {}\t\t\t\t\t\t\t\t\t{}\t", thirty_two, uthirty_two);
        println!("64\t {}\t\t\t\t\t\t{}\t", sixty_four, usixty_four);
        println!("128\t {}\t{}\t", one_twenty_eight, u_one_twenty_eight);
    }

    fn print_floats(){
        let three : f32 = 3.3;
        let pie : f64 = 3.14455555555555555555555555555555555555555555555555555555555555;
        println!("{:.20},{:.20}",three, pie);
    }

    fn print_char_bools(){
        let k: char = 'k';
        let bob_bool: bool = true;
        if(bob_bool){
            println!("Bob's favorite letter is {}", k);
        }else{
            println!{"Who knows what his favorite letters is, and who really cares anyway?"};
        }
    }

    fn literals_and_operators(){
        // Integer addition
        println!("1 + 2 = {}", 1u32 + 2);

        // Integer subtraction
        println!("1 - 2 = {}", 1i32 - 2);

        // Short-circuiting boolean logic
        println!("true AND false is {}", true && false);
        println!("true OR false is {}", true || false);
        println!("NOT true is {}", !true);

        // Bitwise operations
        println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
        println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
        println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
        println!("1 << 5 is {}", 1u32 << 5);
        println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

        // Use underscores to improve readability!
        println!("One million is written as {}", 1_000_000u32);
    }

    fn array_stuff(){
        //no messing around with multi type arrays, use a tuple
        // let an_array = [1,4,3,76,567,3456,true, "asdf"];
        let bool_array:[bool; 2] = [true, false];
        let mut an_array:[i32;6] = [1,2,3,4,5,6];

        an_array[1]=4;
        // println!("{}", an_array);
    }

    fn tuple_trouble(){
        let tribble = ("troublesome", true, false);
        println!("True or false: tribbles are {} ", tribble.0);
        println!("{}",tribble.1);
    }
}

#[allow(dead_code)]
mod chapter_five{
    fn five_point_one(){

    }
}