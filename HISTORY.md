# Beginning

`align_constr` crate started as an attempt to iterate on [`aligned`] crate. The author found that the implementation of [Aligned](https://docs.rs/aligned/0.4.0/aligned/struct.Aligned.html) struct suited for much more than aligning for fixed, "literal" values and instead could be used for generating types that are "alignment-constrained" not only by inherent alignment requirements but also by requirements for "alignment constraint archetypes".

The author found that [`aligned`] 

[`aligned`](https://crates.io/crates/aligned)