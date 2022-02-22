[![crates.io](https://img.shields.io/crates/v/align_constr.svg)][`align_constr`]
[![crates.io](https://img.shields.io/crates/d/align_constr.svg)][`align_constr`]

# [Alignment][alignment]-constrained [newtype]

[Newtype][newtype] whose [alignment] is constrained not only by the [inherent alignment requirements](https://doc.rust-lang.org/reference/type-layout.html) of the underlying type but also by the alignment requirements of the "alignment constraint archetype". Within this context, "alignment constraint archetype" `AlignConstrArchetype` is a type whose alignment constraint is imposed on the underlying type `T` to produce [`AlignConstr<T, AlignConstrArchetype>`][`AlignConstr`].

## Notes

* "alignment constraint archetype" is a
[stipulative](https://www.ucfmapper.com/education/various-types-definitions/#:~:text=Stipulative%20definitions)
[functional](https://www.ucfmapper.com/education/various-types-definitions/#:~:text=Functional%20definitions)
definition.
* [`AlignConstr<T, AlignConstrArchetype>`][`AlignConstr`] for some underlying type `T` and "alignment constraint archetype" `AlignConstrArchetype` can also be seen as a [refinement type](https://en.wikipedia.org/wiki/Refinement_type) [reified](https://en.wikipedia.org/wiki/Reification_(computer_science)) in the form of a [parameterized](http://www.angelikalanger.com/GenericsFAQ/FAQSections/ParameterizedTypes.html#FAQ001) [newtype].

# Resources on alignment

* ["Data Alignment"](http://www.songho.ca/misc/alignment/dataalign.html), personal website of Song Ho Ahn - a professor in the Computer Science department at Sheridan College (Oakville)
> Not so visually appealing yet incredibly well written article that explains not only what alignment is but also why it exists and where one might need to overalign data.
>
> Dmitrii Demenev
* ["What is overalignment of execution regions and input sections?", Stack Overflow][overalignment]
* ["Data Structure alignment", GeeksforGeeks](https://www.geeksforgeeks.org/data-structure-alignment/)
* ["Type Layout", The Rust Referece](https://doc.rust-lang.org/reference/type-layout.html)

# [`align_constr`] vs [`aligned`]

[`aligned`] is a popular library that [served as a prototype](https://github.com/JohnScience/align_constr/blob/main/HISTORY.md) for [`align_constr`]. At the time of writing, [`align_constr`] featurewise and idiomatically superior and provides not only more extensive documentation but also a selection of resources on the subject. To pay respect to the contributors of [`aligned`] crate, the quality of their creation's [CI] is still unmatched by [`align_constr`]. However, the limited [scope] of both libraries nonetheless makes [`align_constr`] arguably better for any use case.

[overalignment]: https://stackoverflow.com/questions/8732441/what-is-overalignment-of-execution-regions-and-input-sections
[`AlignConstr`]: https://docs.rs/align_constr/latest/align_constr/struct.AlignConstr.html
[`aligned`]: https://crates.io/crates/aligned
[`align_constr`]: https://crates.io/crates/align_constr
[newtype]: https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html
[alignment]: https://www.geeksforgeeks.org/data-structure-alignment/
[CI]: https://en.wikipedia.org/wiki/Continuous_integration
[scope]: https://en.wikipedia.org/wiki/Scope_(project_management)

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
