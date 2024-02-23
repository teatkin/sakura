# Rust Internals

A lot of this project relies on a reasonable knowledge of how the rust `core` crate
works. I'm used to the `core` crate being abstracted away behind `sys` but since
this is an embedded RTOS, I can't use anything in `sys` so I'll have to learn how to use
the memory, atomics, and whatever else I need to use from `core` instead. 

As such, this is where I'm going to keep a record of all the notes I make on how the 
`core` crate works.
