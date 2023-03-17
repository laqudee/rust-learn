# Module std::cell

- shared mutable containers

- rust memory safely is based on this rule: Given an  object T, it is only possoble to have one of the follow:
  - Having several immutable references(&T) to the object (also known as aliasing)
  - Having one mutable reference(&mut T) to the object (also know as mutability)

- Cell<T> and RefCell<T>
  - Cell<T> and RefCell<T> are thread safe(they do not implement Sync)

- Mutex<T>, RwLock<T> or atomic types.

- Values of the Cell<T> and RefCell<T> types may be mutated throught shared references(the common &T type)

## when to choose interior mutability

- Introducing mutability 'inside' of something immutable
- Implementation details of logically-immutable methods
- Mutating implementations of Clone.

- many shared smart pointer types, including Rc<T> and Arc<T>, provide containers that can be cloned and shared between multiple parties.

- Note that this example uses Rc<T> and not Arc<T>.
- RefCell<T>s are for single-threaded scenarios.
- Consider using RwLock<T> or Mutex<T> if you need shared mutability in a muli-threaded situation.

- Mutating implementations of Clone
  - therefore, any mutation that happens in the clone method must use cell types. For example, Rc<T> maintains its reference counts within a Cell<T>


