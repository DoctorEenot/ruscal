use std::fmt::Display;

macro_rules! pascal {


    // const declarations
    (
        const
        $($tail:tt)*
    ) => {
        pascal!($($tail)*);
    };

    (
        $($inner:tt),* = $value:tt;
        $($tail:tt)*
    ) => {

        $(let $inner = $value);*;
        pascal!($($tail)*);
    };

    // variable declarations
    (
        var
        $($tail:tt)*
    ) => {
        pascal!($($tail)*);
    };

    // integer
    (
        $($inner:tt),* : integer;
        $($tail:tt)*
    ) => {

        $(let mut $inner:i64 = Default::default());*;
        pascal!($($tail)*);
    };

    (
        $($inner:tt),* : integer = $value:tt;
        $($tail:tt)*
    ) => {

        $(let mut $inner:i64 = $value);*;
        pascal!($($tail)*);
    };

    // real
    (
        $($inner:tt),* : real;
        $($tail:tt)*
    ) => {

        $(let mut $inner:f64 = Default::default());*;
        pascal!($($tail)*);
    };

    (
        $($inner:tt),* : real = $value:tt;
        $($tail:tt)*
    ) => {

        $(let mut $inner:f64 = $value);*;
        pascal!($($tail)*);
    };

    // string
    (
        $($inner:tt),* : string;
        $($tail:tt)*
    ) => {

        $(let mut $inner:&str = Default::default());*;
        pascal!($($tail)*);
    };

    (
        $($inner:tt),* : string = $value:expr;
        $($tail:tt)*
    ) => {

        $(let mut $inner:String = $value);*;
        pascal!($($tail)*);
    };

    // function call
    (
        $function_name:ident($($inner:expr),*);
        $($tail:tt)*
    ) => {
        $function_name($($inner),*);
        pascal!($($tail)*);
    };

    // variable initialization
    (
        $variable_name:ident := $value:expr;
        $($tail:tt)*
    ) => {

        $variable_name = $value;
        pascal!($($tail)*);
    };

    // basic program structure
    (
        end;
        $($tail:tt)*
    ) => {
        return
    };
    (
        end.
        $($tail:tt)*
    ) => {
        return
    };
    (
        begin
        $($tail:tt)*
    ) => {
            pascal!($($tail)*);
    };
    (
        program
        $program_name: ident
        $($tail:tt)*
    ) => {
        fn main() {
            pascal!($($tail)*);
        }
    };
}

pub fn writeln<T: Display>(text: T) {
    println!("{}", text);
}

pascal!(
    program PascalTest

    const
        c, d = 5;
    var
        a, b : integer = 5;
        ar, br: real;
        s : string;

    begin
        writeln(a+b);

        writeln(ar+br);
        ar := 5.4;
        br := -0.1;
        writeln(ar+br);

        b := 332;
        a := 10 + 5 + c - b + d;
        writeln(a);

        writeln(s);
        s := "123";
        writeln(s);
    end.
);
