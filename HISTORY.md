# Beginning

`align_constr` crate started as an attempt to iterate on [`aligned`] crate. The author found that the implementation of [`aligned::Aligned`][`Aligned`] struct suited for much more than aligning for fixed, "literal" values and instead could be used for generating types that are "alignment-constrained" not only by inherent alignment requirements but also by requirements for "alignment constraint archetypes".

Weirdly enough, in [`aligned::Aligned`][`Aligned`] the most important type parameter - which is the type of the stored value - for some unconvincing or unknown reason is stored **second**, not first. It sort of makes sense from the declaration, from the structure of [`aligned::Aligned`][`Aligned`], yet the author believes that semantically the type of the stored value is more important than the alignment constraint and therefore the type of the stored value must be the very first type parameter.

The case of `C++` keyword [`alignas`], where syntactically "alignment constraint archetype" (or the numerical alignment) comes first, actually advocates for the use of the stored value as the first type parameter because [`alignas`] is a specifier, the optional piece of syntax similar to attributes in Rust:

```C++, ignore
// every object of type struct_float will be aligned to alignof(float) boundary
// (usually 4):
struct alignas(float) struct_float
{
    // your definition here
};
 
// every object of type sse_t will be aligned to 32-byte boundary:
struct alignas(32) sse_t
{
    float sse_data[4];
};
 
// the array "cacheline" will be aligned to 64-byte boundary:
alignas(64) char cacheline[64];
 
 
#include <iostream>
int main()
{
    struct default_aligned { float data[4]; } a, b, c;
    sse_t x, y, z;
 
    std::cout
        << "alignof(struct_float) = " << alignof(struct_float) << '\n'
        << "sizeof(sse_t) = " << sizeof(sse_t) << '\n'
        << "alignof(sse_t) = " << alignof(sse_t) << '\n'
        << "alignof(cacheline) = " << alignof(alignas(64) char[64]) << '\n'
        << std::hex << std::showbase
        << "&a: " << &a << '\n'
        << "&b: " << &b << '\n'
        << "&c: " << &c << '\n'
        << "&x: " << &x << '\n'
        << "&y: " << &y << '\n'
        << "&z: " << &z << '\n';
}
```

On that ground, the author decided to change the declaration of "alignment-constrained" type so that it is [`align_constr::AlignConstr<T,AlignConstrArchetype>`][`AlignConstr`] where `T` is the stored value and `AlignConstrArchetype` is the "alignment constraint archetype".

The author also found that [`aligned`] used the [`core::ops::DerefMut`] and [`core::ops::Deref`] traits for accessing the nested value in [`Aligned`] with and without mutation, respectively. While 5 characters shorter than `.value`, which is the alternative chosen by [`align_constr`], it overloads the regular meanings of [`core::ops::DerefMut`] and [`core::ops::Deref`] traits and makes dereferencing the aligned references harder for the end user.

In order to give access to the stored value, the author has chosen the most pragmatic approach, making the `.value` field accessible, thereby also making the syntactically-sugared getter redundant. Thus, [`core::ops::DerefMut`] and [`core::ops::Deref`] can preserve their original meaning and be used to **dereference** the stored value instead of accessing the stored value.

[`aligned`]: https://crates.io/crates/aligned
[`aligned`]: https://crates.io/crates/align_constr
[`Aligned`]: https://docs.rs/aligned/0.4.0/aligned/struct.Aligned.html
[`AlignConstr`]: https://docs.rs/align_constr/latest/align_constr/struct.AlignConstr.html
[`core::ops::Deref`]: https://doc.rust-lang.org/beta/core/ops/trait.Deref.html
[`alignas`]: https://en.cppreference.com/w/cpp/language/alignas