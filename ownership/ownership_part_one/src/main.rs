fn main() {
   // To transfer data without copying (expensive operation) we use pointers in rust (aka boxes)
   // b now owns the pointer to the array 
   let a = Box::new([0; 10]);
   let b = a;
   println!("{:?}", b);
   // rust automatically handles memory management once a stack frame ends when a variable owns a pointer it automatically gets deallocated
   // String::from("string") creates a heap allocated string versus "string" which is a string literal (immutable and can be shared - aka shared reference &'static str)
   let first = String::from("Ferris");
   // here the pointer ownership is moved from first to name
   let full = add__suffix(first);

   println!("{}", full);
   //printing first will cause undefined behavior since it has been freed from memory!

   // cloning can also be used to avoid moves (first.clone())

   // we can simplify the above by using references 
   // references are also known as a 'borrow' pointers
   // below is how the borrow checker ensures memory safety

   let mut vec: Vec<i32> = vec![1, 2, 3];
   // the above variable has read write own permissions
   let num: &i32 = &vec[2]; // this variable now has read and own perms (no write perms since it has no mut)    // the above variable (vec) loses it's Own and Write perms (it still keeps its read perms!)
   println!("The number is {}", *num); // *num has read perms on its path
   // read and own perms are now lost for num and vec regains its perms
   vec.push(3);

   // if we were to push a number to vec before accessing it (*num) this could potentially invalidate the memory being accessed. Rust's borrow checker prevents this!
   
   // mutable references provide unique and non owning access to data 
   let mut_ref: &mut i32 = &mut vec[3]; // vec now loses its Read Own Write perms mut_ref gains read and own perms 
   *mut_ref += 1; // *mut_ref gains read and write perms


   // the rust borrow checker also checks for flow permissions
   // the core concept of this permission is that data must outlive all of its references 
   // for simple examples rust checks to see if the owner perms have been dropped before being used again
   // ex.

   let s = String::from("Hello World");
   let s__ref = &s;
   // drop(s); // 
   print!("{}", s__ref);
}
// pointer is then destroyed when the stack frame ends (the original pointer that was owned by first)
fn add__suffix(mut name: String) -> String {
    name.push_str("Bueller");
    name
}
  