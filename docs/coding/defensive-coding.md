# Defensive Coding

The [recent Cloudflare outage](https://blog.cloudflare.com/18-november-2025-outage/) caused a major global disruption, and it's a good reminder of why defensive coding matters.

In the post-mortem, one issue stood out: `append_with_names()` returns a `Result`, but the caller used `.unwrap()` without handling the error. That triggered the panic:

```rust
pub fn fetch_features(...) -> Result<(), (ErrorFlags, i32)> {
    // ...
    let (feature_values, _) = features.append_with_names(...).unwrap();
    // ...
}
```

This is a classic example of a missing defensive guard. The [lecture by Kian Paimani](https://youtu.be/y8ZnmM555UY) focuses on defensive practices in Polkadot's Rust codebase, but the principles apply everywhere.

> Defensive Programming is a form of defensive design to ensure the continuing function of a piece of software under **unforeseen** circumstances, where high availability, safety, or security is needed.

Rust gives you powerful guarantees, but those guarantees don't make your code immune to bugs. An unchecked `unwrap()` is simply another form of null-pointer dereference or unhandled exception. Proper error handling with `Result`, `match`, or the `?` operator would have prevented the outage.

## A Few Practical Defensive Habits

It's helpful to highlight a few habits mentioned by Kian's lecture that fit directly with this outage scenario:

* **Assume your assumptions can be wrong** - even when something "should never fail", code as if it might.
* **Prefer handling over panicking** - `unwrap` and `expect` are convenient, but mapping or propagating errors keeps the system running.
* **Avoid hidden panics** - indexing (`v[i]`), popping from empty collections, or using `unreachable!()` are quiet traps.
* **Document panic conditions** - state clearly when your APIs can panic, following the standard library's pattern.

These small habits prevent exactly the kind of scenario seen in the Cloudflare incident.

## Defensive Coding Beyond Rust

Rust makes defensive lapses easy to spot: if you see an `unwrap()`, you already know you're dealing with a `Result` or `Option`. In other languages this isn't always obvious. For example, in Java you can't tell what `appendWithNames` returns without reading its signature:

```java
public void fetchFeatures(...) throws InvalidFeatureException {
    // ...
    var featureValues = features.appendWithNames(...);
    // ...
}
```

Two simple defensive checks would already help:

1. Check whether `featureValues` is `null`.
2. Throw an exception if `appendWithNames` encounters an error.

Neither is complicated, but both make the system more resilient.

Are these redundant? Yes - and that's exactly why they're valuable. As [Herb Sutter](https://youtu.be/oitYvDe4nps) argues, defensive checks are not about type or memory safety; they're about **functional correctness**. They:

* Express intent and document assumptions.
* Give you a clear signal when reality diverges from those assumptions.
* Add redundancy, which is exactly what you need for error detection.

The least likely assertion to fail is often the most valuable one: if it ever does fire, something fundamental has broken. A check that "should never fire" is precisely the one worth writing.

So even in a language like Java - where nullability or failure modes aren't as visible as in Rust - explicit defensive checks make assumptions concrete and failure modes clear. Defensive coding may feel repetitive, but that "extra" line is often the one that prevents the next outage.

## Conclusion

Defensive coding isn't about pessimism - it's about clarity. By making assumptions explicit and handling the "impossible" cases, we give ourselves earlier, sharper signals when something drifts out of spec. Whether it's Rust, Java, or any other language, these habits turn silent failures into visible ones and stop small oversights from becoming global outages. A few extra checks cost little, but the resilience they buy is huge.
