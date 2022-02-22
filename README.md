[![crates.io](https://img.shields.io/crates/v/align_constr.svg)][`align_constr`]
[![crates.io](https://img.shields.io/crates/d/align_constr.svg)][`align_constr`]

# [Alignment][alignment]-constrained [newtype]

[Newtype][newtype] whose [alignment] is constrained not only by the [inherent alignment requirements](https://doc.rust-lang.org/reference/type-layout.html) of the underlying type but also by the alignment requirements of the "alignment constraint archetype".

**Note**: "alignment constraint archetype" is a
[stipulative](https://www.ucfmapper.com/education/various-types-definitions/#:~:text=Stipulative%20definitions)
[functional](https://www.ucfmapper.com/education/various-types-definitions/#:~:text=Functional%20definitions)
definition. Within this context, it means a type whose alignment constraint is imposed on `T`.

# Resources on alignment

* ["Type Layout", The Rust Referece](https://doc.rust-lang.org/reference/type-layout.html)
* ["Data Structure alignment", GeeksforGeeks](https://www.geeksforgeeks.org/data-structure-alignment/)
* ["Data Alignment"](http://www.songho.ca/misc/alignment/dataalign.html), personal website of Song Ho Ahn - a professor in the Computer Science department at Sheridan College (Oakville)

# Alternatives of [`align_constr`] crate

* [`aligned`](https://crates.io/crates/aligned), the library that [served as a prototype](https://github.com/JohnScience/align_constr/blob/main/HISTORY.md) for [`align_constr`]. At the time of writing, [`align_constr`] featurewise and idiomatically superior and provides not only more extensive documentation but also a selection of resources on the subject.

[`align_constr`]: https://crates.io/crates/align_constr
[newtype]: https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html
[alignment]: https://www.geeksforgeeks.org/data-structure-alignment/

# License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>