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

As a simple test, the `main` function, iterates a process of scheduling three tasks, "t1", "t2", "t3", to be executed in order. The relative deadlines are out of order and designed to stress wrapping edge cases.

The takeaway here is to provide a (golden) reference for hardware implementation counterparts.

## Disclaimer

Rust is unfortunately not (yet) capable of native arbitrary precision integers, so we chose `i8` for the purpose of demonstration.

## Hardware implementation

A efficient hardware implementation can be obtained by leveraging an arbitration tree approach with logarithmic (log2) depth, preceded by a single pre-processing layer. In the below figure, we depict the arbitration between 8 interrupt vectors (tasks), yielding a tree of depth 3.

![edf][edf]

The vector table will store absolute deadlines (`now + rel_dl` at the time of pending corresponding task).

The pre-processing node takes the absolute deadline (`abs_dl[n]`) and subtracts the current `now` time to obtain a normalized deadline for comparison. The result (`dl[n]`) together with its pending bit (`p[n]`) is connected to the top layer of the arbitration tree.  

![pre][pre]

The arbitration tree consist of comparators, selecting the shortest (normalized) deadline between `dl_a` and `dl_b`. Pending information (`p_a ∨ p_b`), along with shortest deadline information (`min(dl_a, dl_b)`) are connected to next layer of the arbitration tree. In case `p_b` is false (not pended) `dl_a` is selected.

For brevity vector index forwarding is omitted. Its implementation is straightforward, by for each layer selecting winning index based on the deadline mux control signal.

At the bottom of the tree, we can determine if any interrupt is pending, and if so its index in the vector table (allowing us to unpend the interrupt, and dispatch corresponding task for execution).

![cmp][cmp]

[edf]: ./drawio/edf.drawio.svg
[pre]: ./drawio/edf_pre.drawio.svg
[cmp]: ./drawio/edf_cmp.drawio.svg
