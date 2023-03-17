# alloc

- Memory allocation APIs(内存分配APIs)

- In a given program, the standard library has one "global" memory allocator that is used for example by Box<T> and Vec<T>

## The #[global_allocator] attribute

- this attribute allows configuring the choice of global allocator. 
- you can use this to implement a completely custom global allocator to route all default allocation request to a custom object

