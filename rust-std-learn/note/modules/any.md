# Module std::any

- Utilities for dynamic typing or type reflection

## Any and TypeId
- Any itself can be used to get a TypId, and has more features when used as a trait object.

-  &dyn Any (a borrowed trait object)

- is()

- downcast_ref()

- Note that &dyn Any is limited to testing whwther a value is of a specified concrete type, and cannot be used to test whethrt a t ype implements a trait

## Smart pointers and dyn Any

## Provider and Demand

- data flow