# Designing Traits That Age Well

Early in my career I tended to design trait-based APIs the way one first approaches Rust: favouring ergonomics over rigour, assuming the common case, and quietly relying on the caller to handle the edges.

The abstractions felt pleasant when execution stayed within the intended invariants. They became brittle the moment real workloads drifted away from the assumptions embedded in the crate.

## Implicit Contracts are Technical Debt

Many of my earlier traits exposed methods that accepted `&self` while silently depending on interior state changes. Some returned `Option<T>` with the unwritten rule that `None` was "not expected in real use".

Preconditions lived in comments, postconditions lived only in the author's head, and versioning was something to tidy up later.

The system compiled. It even shipped. But each release increased the distance between the public trait surface and the invariants it was actually capable of enforcing.

## When Hidden Assumptions Surface

The breaking point was never a single catastrophic failure. It was a slow accumulation of runtime surprises:

* a method that started yielding values outside its original semantic range;
* a trait implementation that unintentionally relaxed a constraint another crate still relied on;
* an `async` boundary whose cancellation behaviour shifted subtly over time;
* an internal `unsafe` block whose safety explanation no longer described the real codepath.

No single change was unreasonable. Taken together they produced undefined behaviour at the trait boundary. The compiler could not protect me, and downstream crates began to notice the entropy.

## Redesign Principles Eventually Applied

The goal was simple: replace hidden assumptions with explicit, enforceable boundaries.

1. Make invariants explicit and enforceable. Where I once relied on convention, I introduced marker types, sealed traits, capability traits, and const generics to push guarantees into the type system.
2. Version before you break. Instead of a sweeping breaking release, I introduced a staged deprecation plan: `#[deprecated(note = "use NewType instead")]` on the legacy items, parallel modules with v2 suffixes, and a commitment to keep the older surface compiling for two minor versions.
3. Replace panics with structured errors. Every former `.unwrap()` and `.expect()` reachable from the public surface became a `Result<(), SpecificError>`. Callers now choose how to propagate or recover instead of experiencing an abrupt abort.
4. Introduce adapters and fallback layers. Where removal was too disruptive, I added small adapters (`impl From<OldType> for NewType`) and feature-gated shims. Existing code continues to build while newer code opts into the stricter trait guarantees.
5. Explain intent at the boundary. Every public trait and type now includes a short note describing its stability class (stable, experimental, or internal) and its intended evolution plan.

## Resulting Crate

The new major version requires a touch more ceremony to use. It is also far more predictable.
* Semver issues surface through the compiler rather than through production logs.
* Downstream crates upgrade gradually rather than in a disruptive leap.
* Unexpected input no longer causes abrupt termination; the crate degrades gracefully or returns a clear, structured error.

## Closing Thought

A trait that feels effortless on day one but quietly accumulates undefined behaviour carries hidden costs. True long-term productivity comes from traits that demand clarity upfront yet remain dependable years later. I still provide ergonomic wrappers for the common path, but they now sit as thin adapters layered over a well-defined core rather than forming the foundation itself.

Longevity, not initial speed, has become my measure of success. Like any carefully maintained system, the crate continues running quietly and predictably long after the original author has moved on to other challenges.
