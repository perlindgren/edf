# EnClic - EDF Nested CLIC in Veryl

This repo gives a possible implementation of the interrupt arbitration, using a recursive function `min` to compute the minimal normalized deadline (relative to the current time).

An entry in the vector table is defined as:

```sv
struct Entry {
    addr   : u32     ,
    rel_dl : DeadLine,
    abs_dl : DeadLine,
    pend   : logic   ,
    enable : logic   ,
    running: logic   ,
}
```

On `pend`, the absolute deadline is computed as `abs_dl = now + rel_dl` and stored in the entry.

To compute the normalized deadlines we introduce a function:

```sv
function normalize (
    int_vec: input IntVec  ,
    now    : input DeadLine,
) -> RelDeadLines {
    var rel_dl: RelDeadLines;
    for i: u32 in 0..NrVec {
        if (int_vec[i].pend & int_vec[i].enable) | int_vec[i].running {
            // either pended and enabled OR already running
            rel_dl[i] = int_vec[i].abs_dl - now;
        } else {
            // MAX_DEADLINE indicate a task not being part of arbitration
            rel_dl[i] = MAX_DEADLINE;
        }
    }
    return rel_dl;
}
```

The `MAX_DEADLINE` is the largest positive value, and used as a marker for telling this entry should not be considered for the arbitration, i.e., its either not-pended, not-enabled. Notice, running tasks SHOULD be considered in the arbitration, but NOT dispatched.  

This approach allows the arbitration tree to be simplified, as not needing to care about pending, disabled or otherwise non contending tasks (we might filter out tasks that should be blocked from execution here, but its not yet implemented). 

The arbitration tree now becomes a recursive function:

```sv
function min (
    arr        : input RelDeadLines,
    range_start: input IntVecIdx   ,
    range_end  : input IntVecIdx   ,
) -> TreeItem {
    if (range_end - range_start) >= 2 {
        let middle: IntVecIdx = range_start + ((range_end - range_start) >> 1);
        let left  : TreeItem  = min(arr, range_start, middle);
        let right : TreeItem  = min(arr, middle + 1, range_end);
        if (left.value <: right.value) {
            return left;
        } else {
            return right;
        }
    } else if ((range_end - range_start) == 1) {
        if ((arr[range_start]) <: arr[range_end]) {
            var item      : TreeItem;
            item.value = arr[range_start];
            item.idx   = range_start;
            return item;
        } else {
            var item      : TreeItem;
            item.value = arr[range_end];
            item.idx   = range_end;
            return item;
        }
    } else {
        var item      : TreeItem;
        item.value = arr[range_start];
        item.idx   = range_start;
        return item;
    }
}
```
Essentially a binary comparison tree returning the winning item. (This could likely be reduced to just returning the index, in order to reduce complexity.)

Now we can tie everything together:

```sv
import Types::*;

module EdfNestedClic #() (
    i_clk         : input  clock    ,
    i_nrst        : input  reset    ,
    i_now         : input  DeadLine ,
    i_pend        : input  PendVec  ,
    i_pend_idx    : input  IntVecIdx,
    i_ret         : input  logic    ,
    i_ret_idx     : input  IntVecIdx,
    o_dispatch    : output logic    ,
    o_dispatch_idx: output IntVecIdx,
) {
    var iv: IntVec; // iv will be assigned on reset

    ...
    always_ff (i_clk, i_nrst) {
        if_reset {
            for i: u32 in 0..NrVec {
                iv[i].pend     = 0; // clear interrupt vector pending/enable bits
                iv[i].enable   = 0;
                iv[i].running  = 0;
                o_dispatch     = 0; // clear dispatch outputs
                o_dispatch_idx = 0;
            }
        } else {
            // clear running status of returned task
            if i_ret {
                iv[i_ret_idx].running = 0;
            }
            for i: u32 in 0..NrVec {
                if i_pend[i] & !iv[i].pend {
                    iv[i].pend   = 1;
                    iv[i].abs_dl = i_now + iv[i].rel_dl;
                }
            }
            let rel_dl: RelDeadLines = normalize(iv, i_now);

            let min_ti: TreeItem = min(rel_dl, 0, (NrVec - 1) as IntVecIdx);

            // we check that it is not infinite in the future, and not already running
            if min_ti.value != MAX_DEADLINE & !iv[min_ti.idx].running {
                iv[min_ti.idx].pend    = 0;
                iv[min_ti.idx].running = 1;
                o_dispatch             = 1;
            } else {
                o_dispatch = 0;
            }
            o_dispatch_idx = min_ti.idx;
        }
    }
}
```

Essentially in the sequential logic we first clear running status of tasks that has been returned. (Detection or returned interrupts can be detected either by `mret` instruction or by magic number return address, as currently implemented in Hippo, the latter is preferred as allowing to treat interrupts and functions alike, a problem not possible to solve in C/C++/Rust without adding extra overhead in trampoline code.)

Then we process the set of pending interrupts, here one could think of async process level triggered but we keep it nice and tidy and spin our edge detection. Essentially it guarantees we will compute the absolute deadline based on the earliest time of pend.

After that we normalize the absolute deadlines to the current time, to obtain `rel_dl`, and find the minimal entry, by calling the recursive function.

Finally we determine if the minimal entry should be dispatched. If it has the `MAX_DEADLINE` or belongs to the currently executing task, it should be discarded. (The latter prevents re-trigger dispatch of a started task in case it (still) wins the arbitration.)















## Testing

```shell
veryl test  --wave ./src/en_clic_test.veryl  ./src/en_clic.veryl ./src/cfg.veryl --verbose
```

This will generate `en_click.fst`.

```shell
surfer src/en_clic.fst
```
