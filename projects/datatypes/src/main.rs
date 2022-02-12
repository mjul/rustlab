fn main() {
    let eight_bit_signed: i8 = 42;
    let sixteen_bit_unsigned: u16 = 42;
    let tttwo_bit_unsigned: u32 = 42;
    // ...
    let oneoheight_bit_signed: i128 = 42;

    let cpu_specific_unsigned: isize = 42;
    let cpu_specific_signed: usize = 42;

    println!(
        "Numbers {} {} {} {} {} {}",
        eight_bit_signed,
        sixteen_bit_unsigned,
        tttwo_bit_unsigned,
        oneoheight_bit_signed,
        cpu_specific_signed,
        cpu_specific_unsigned
    );

    // literals
    // underscores count as whitespace in numbers
    let decimal_literal = 12_345; 
    let hex_literal = 0x2a; 
    let binary_literal = 0b0010_1010;
    let byte_literal = b'A';

    println!("Literals: {} {} {} {}", decimal_literal, hex_literal, binary_literal, byte_literal);

    // floating point
    let x = 2.0; // f64, 64-bit floating point;
    println!("Floating: {}", x);

    // Boolean
    let t = true;
    let f:bool = false;

    println!("True/false: {} {}", t, f);

    // Character
    // char is a Unicode Scalar Value, see http://unicode.org/glossary/#unicode_scalar_value
    let a_char = 'a';
    let z_char = 'Z';
    let heart_eyed_cat = '😻'; 

    println!("Unicode chars: {} {} {}", a_char, z_char, heart_eyed_cat);

    // Tuples

    let tup: (i32, f64, u8) = (500, 6.4, 42);
    // destructuring
    let (x,y,z)= tup;
    println!("Tuple: {} {} {}", x, y, z);
    println!("Tuple access: {} {} {}", tup.0, tup.1, tup.2);

    // Arrays 
    let a = [1,2,3,4];
    println!("Array: {}", a[0]);

    // with type and dimensions (access is bounds-checked)
    let a : [i64; 3] = [1,2,3];
    println!("Array: {} {} {}", a[0], a[1], a[2]);
}
