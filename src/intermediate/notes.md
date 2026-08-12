## Ownership

1. Variables live in the stack.

A pointer is a value that describes a location in memory. The value that a pointer points-to is called its pointee.
Rust provides a construct called `Box` for putting data on the heap.

> Box deallocation principle (fully correct): If a variable owns a box, when Rust deallocates the variable’s frame, then Rust deallocates the box’s heap memory.

2. Variables Cannot Be Used After Being Moved

> Moved heap data principle: if a variable x moves ownership of heap data to another variable y, then x cannot be used after the move.
