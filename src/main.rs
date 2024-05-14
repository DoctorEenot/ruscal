use std::fmt::Display;

macro_rules! types {
    (
        integer
    ) => {
        i64
    };
    (
       real 
    ) => {
        f64
    };
    (
        string
    ) => {
        &str
    };
}
macro_rules! var_decl {

    (
        $($inner:tt),* : $type:ident;
        $($tail:tt)* 
    ) => {
        $(let mut $inner:types!($type) = Default::default());*;
        var_decl!($($tail)*);
    };

    (
        $($inner:tt),* : $type:ident = $value:tt;
        $($tail:tt)* 
    ) => {
        $(let mut $inner:types!($type) = $value);*;
        var_decl!($($tail)*);
    };

    (
        $($inner:tt)* 
    ) => {
        pascal!($($inner)*)
    };
}

macro_rules! const_decl {

    (
        $($inner:tt),* = $value:tt;
        $($tail:tt)*
    ) => {

        $(let $inner = $value);*;
        const_decl!($($tail)*);
    };

    (
        $($inner:tt)* 
    ) => {
        pascal!($($inner)*)
    };
}

macro_rules! body_decl {
    (
        end;
        $($tail:tt)*
    ) => {
        pascal!($($tail)*);
    };
    (
        end.
    ) => {
        return
    };

    // variable initialization
    (
        $variable_name:ident := $value:expr;
        $($tail:tt)*
    ) => {

        $variable_name = $value;
        body_decl!($($tail)*);
    };

    // function call
    (
        $function_name:ident($($inner:expr),*);
        $($tail:tt)*
    ) => {
        $function_name($($inner),*);
        body_decl!($($tail)*);
    };

}

macro_rules! pascal {

    () => {};

    (
        const
        $($tail:tt)*
    ) => {
        const_decl!($($tail)*);
    };

    (
        var
        $($tail:tt)*
    ) => {
        var_decl!($($tail)*); 
    };

    //(
    //    do
    //    $($tail:tt)*
    //) => {
    //    
    //    pascal!($($tail)*);
    //    
    //};

    (
        begin
        $($tail:tt)*
    ) => {
        
        body_decl!($($tail)*);
        
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
        writeln(ar);
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
        
        a := 5;

    end.
);


