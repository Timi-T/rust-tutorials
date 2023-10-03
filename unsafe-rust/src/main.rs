fn main() {
    let mut num = 5;
    let r1 = &num as *const i32; // non-mutable raw pointer
    let r2 = &mut num as *mut i32; // mutable raw pointer 
     // can have multiple mutable or non-mutable raw pointers to the same address in memory
     // Raw pointers are allowed to be null
     // Raw pointers don't implement automatic cleanu so you have to manually free.

     let address = 0x012345usize; // create an address
     let r3 = address as *const i32; // Trying to get a raw pointer to this address (adress could be null)

     // To dereference the raw pointers, we have to use the unsafe keyword
     unsafe {
        println!("r1 is: {}", *r1);
        *r2 = 10; // here we change the value of r2
        println!("r2 is: {}", *r2);
     }

     unsafe fn dangerous() {
        // Unsafe function
     }

     unsafe {
        // Must be called in an unsafe block
        dangerous();
     }

     unsafe {
        println!("Absolute of -6 according to C is: {}", abs(-6));
     }
}

// Having unsafe code in a function doesn't necessarily mean unsafe function. We can create a safe abstraction

// This an interface to use C language functions
extern "C" {
    fn abs(input: i32) -> i32;
}

// This is an interface to call rust functions from C
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a rust function from C");
}

// A trait is unsafe if one of its functions is unsafe

