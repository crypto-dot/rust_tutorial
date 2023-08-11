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

   // the above example can be simplifed by using references which are considered bowe
}
// pointer is then destroyed when the stack frame ends (the original pointer that was owned by first)
fn add__suffix(mut name: String) -> String {
    name.push_str("Bueller");
    name
}
  