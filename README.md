# Disco Ball Garbage Collector

This is a garbage collection designed for purely functional programming language runtime, which greatly use immutable data structures and non-strict evaluation strategy.

### Throughput & Latency
For traditional functional programming lanugage that is using immutable data structures, it will allocate a great amount of thunks and garbage collections.

And GHC garbage collector is very good on this. But now Garbage collection schemes are generally forced to trade-off between latency and throughput. And in some cases, when low latency is needed, [GHC GC can't handle it well](https://making.pusher.com/latency-working-set-ghc-gc-pick-two/). So we have [Low latency GC for Haskell](https://well-typed.com/blog/2019/10/nonmoving-gc-merge/).

### Status

Based on sundial design, creating one garbage collector.

- [x] GC struct, and roots
- [ ] Arena that holds multiple GC struct
- [ ] How to scan nodes in the GC
- [ ] How to pass nodes between GC using PhantomPinned
