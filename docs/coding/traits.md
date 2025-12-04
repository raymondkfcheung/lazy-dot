# Designing Traits That Age Well

Early in my career I tended to design trait-based APIs the way one first approaches Rust: favouring ergonomics over rigour, assuming the common case, and quietly relying on the caller to handle the edges.

The abstractions felt pleasant when execution stayed within the intended invariants. They became brittle the moment real workloads drifted away from the assumptions embedded in the crate.

## Implicit Contracts are Technical Debt

Many of my earlier traits exposed methods that accepted `&self` while silently depending on interior state changes. Some returned `Option<T>` with the unwritten rule that `None` was "not expected in real use".

Preconditions lived in comments, postconditions in the author's head, and versioning was something to tidy up later.

The system compiled. It even shipped. Yet each release quietly increased the gap between the public trait surface and the invariants it could actually guarantee.

## When Hidden Assumptions Surface

The breaking point was never a single catastrophic failure. It emerged as a series of subtle runtime surprises:

* a method returning values outside its original semantic range;
* a trait implementation that unintentionally relaxed a bound another crate still relied on;
* an `async` boundary whose cancellation behaviour shifted subtly;
* an internal `unsafe` block whose safety note no longer described reality

No one change was unreasonable. Taken together they produced undefined behaviour at the trait boundary. The compiler could not protect me, and downstream crates began to feel the effect.

## Redesign Principles Eventually Applied

The goal was clear: replace hidden assumptions with explicit, enforceable boundaries.

1. Make invariants explicit and enforceable. I introduced marker types, sealed traits, capability traits, and const generics to push guarantees into the type system.
2. Version before you break. Staged deprecation allowed old items to coexist with new modules while signalling preferred patterns.
3. Replace panics with structured errors. Public `.unwrap()` and `.expect()` calls became `Result<T, E>`, letting callers handle failures gracefully.
4. Introduce adapters and fallback layers. Where removal was too disruptive, lightweight adapters (`impl From<OldType> for NewType`) and feature-gated shims preserved compatibility.
5. Explain intent at the boundary. Each public trait and type now carries a note on its stability class (stable, experimental, internal) and evolution policy.

## Resulting Crate

The new version requires slightly more ceremony to use, but it behaves consistently:

* Semver issues surface at compile time rather than in production;
* Downstream crates migrate gradually instead of abruptly;
* Unexpected input no longer triggers abrupt termination; the crate returns structured errors or degrades gracefully.

## Measure Twice, Cut Once

A trait that feels effortless on day one but quietly accumulates undefined behaviour carries hidden costs. True long-term productivity comes from traits that demand clarity upfront yet remain dependable for years.

I still provide ergonomic wrappers for the common path, but they are thin adapters layered over a well-defined core rather than forming the foundation itself. Longevity, not initial speed, has become my measure of success.
