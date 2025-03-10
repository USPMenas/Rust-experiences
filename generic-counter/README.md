# Generic counter

In this exercise you will take a very simple data structure and make it generic. It uses a `std::collections::HashMap` to keep track of which values have been seen and how many times each one has appeared.

The initial version of `Counter` is hard coded to only work for `u32` values. Make the struct and its methods generic over the type of value being tracked, that way `Counter` can track any type of value.

To got further, try using the `entry` method to halve the number of hash lookups required to implement the `count` method.

Check that your code works using `cargo run`.