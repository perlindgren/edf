# Edf

This crate emulates Edf scheduling of tasks with relative deadlines.

We emulate the pended interrupts by a vector of tasks.

```rust
struct Task {
    _id: &'static str,
    abs_dl: i8,
}
```

For the example we use `i8` to represent absolute deadlines.

The interrupt controller is emulated:

```rust
struct Edf {
    now: i8,
    tasks: Vec<Task>,
}
```

`now` represents a (partially) monotonic timer (using wrapping arithmetics), while tasks are the currently pending interrupts.

The `pend` method takes a relative deadline, and creates an absolute deadline under wrapping arithmetics.

The `schedule` method, increments the monotonic timer (`now`) under wrapping arithmetics, and (linearly) searches for the minimum absolute deadline in relation to `now`. This is robust, such that tasks with maximally passed deadlines will get scheduled first (under the wrapping arithmetic range).

## Proof of concept

As a simple test, the `main` function, iterate a process of scheduling three tasks, "t1", "t2", "t3", to be executed in order, while ensuring to stress wrapping edge cases.

The takeaway here is to provide a (golden) reference for hardware implementation counterparts.

## Disclaimer

Rust is unfortunately not (yet) capable of native arbitrary precision integers, so we chose `i8` for the purpose of demonstration.
