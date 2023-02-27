# Embedded Audio Tools

Toolbox for creating audio effects with focus on the embedded aspect of things.

### Memory

With `MemSlice` and `MutMemSlice` statically allocated buffers can easily and safely be manipulated.
Creating `SubSlice`s of existing buffers is easy an can be either mutable or non-mutable. They also
implement `Send` as long as the underlying buffer is considered static. When the size of a buffer is
known at compile time, then can this crate handle the task.