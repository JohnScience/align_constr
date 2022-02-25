#![doc = include_str!("../README.md")]
#![no_std]
#![cfg_attr(feature = "const_trait_impl", feature(const_trait_impl))]
#![cfg_attr(feature = "const_fn_trait_bound", feature(const_fn_trait_bound))]
#![cfg_attr(feature = "const_mut_refs", feature(const_mut_refs))]

#[cfg(all(feature = "const_trait_impl", feature = "const_fn_trait_bound"))]
use remove_macro_call::remove_macro_call;
#[cfg(not(all(feature = "const_trait_impl", feature = "const_fn_trait_bound")))]
use unconst_trait_impl::unconst_trait_impl;

/// [N-ZSTs](https://github.com/rust-lang/unsafe-code-guidelines/issues/172), i.e.
/// [zero-sized datatypes](https://runrust.miraheze.org/wiki/Zero-sized_type) whose
/// [alignment](https://www.geeksforgeeks.org/data-structure-alignment/) is `N`.
///
/// If you have to deal with larger alignments, please contact the author via the
/// email provided in Cargo.toml or via an issue.
pub mod n_zst {
    /// 2-[ZST](https://runrust.miraheze.org/wiki/Zero-sized_type).
    ///
    /// Read more about n-ZSTs [here](https://github.com/rust-lang/unsafe-code-guidelines/issues/172)
    #[repr(align(1))]
    pub struct ZST1;

    /// 2-[ZST](https://runrust.miraheze.org/wiki/Zero-sized_type).
    ///
    /// Read more about n-ZSTs [here](https://github.com/rust-lang/unsafe-code-guidelines/issues/172)
    #[repr(align(2))]
    pub struct ZST2;

    /// 4-[ZST](https://runrust.miraheze.org/wiki/Zero-sized_type).
    ///
    /// Read more about n-ZSTs [here](https://github.com/rust-lang/unsafe-code-guidelines/issues/172)
    #[repr(align(4))]
    pub struct ZST4;

    /// 8-[ZST](https://runrust.miraheze.org/wiki/Zero-sized_type).
    ///
    /// Read more about n-ZSTs [here](https://github.com/rust-lang/unsafe-code-guidelines/issues/172)
    #[repr(align(8))]
    pub struct ZST8;

    /// 16-[ZST](https://runrust.miraheze.org/wiki/Zero-sized_type).
    ///
    /// Read more about n-ZSTs [here](https://github.com/rust-lang/unsafe-code-guidelines/issues/172)
    #[repr(align(16))]
    pub struct ZST16;

    /// 32-[ZST](https://runrust.miraheze.org/wiki/Zero-sized_type).
    ///
    /// Read more about n-ZSTs [here](https://github.com/rust-lang/unsafe-code-guidelines/issues/172)
    #[repr(align(32))]
    pub struct ZST32;

    /// 64-[ZST](https://runrust.miraheze.org/wiki/Zero-sized_type).
    ///
    /// Read more about n-ZSTs [here](https://github.com/rust-lang/unsafe-code-guidelines/issues/172)
    #[repr(align(64))]
    pub struct ZST64;

    /// 128-[ZST](https://runrust.miraheze.org/wiki/Zero-sized_type).
    ///
    /// Read more about n-ZSTs [here](https://github.com/rust-lang/unsafe-code-guidelines/issues/172)
    #[repr(align(128))]
    pub struct ZST128;

    /// 256-[ZST](https://runrust.miraheze.org/wiki/Zero-sized_type).
    ///
    /// Read more about n-ZSTs [here](https://github.com/rust-lang/unsafe-code-guidelines/issues/172)
    #[repr(align(256))]
    pub struct ZST256;

    /// 512-[ZST](https://runrust.miraheze.org/wiki/Zero-sized_type).
    ///
    /// Read more about n-ZSTs [here](https://github.com/rust-lang/unsafe-code-guidelines/issues/172)
    #[repr(align(512))]
    pub struct ZST512;
}

/// Alignment-constrained datatype, i.e. a type whose
/// [alignment](https://www.geeksforgeeks.org/data-structure-alignment/)
/// is constrained not only by the inherent alignment requirements of the underlying type but also
/// by the alignment requirements of the "alignment constraint archetype". Within this context,
/// "alignment constraint archetype" `AlignConstrArchetype` is a type whose alignment constraint
/// is imposed on the underlying type `T` to produce [`AlignConstr<T, AlignConstrArchetype>`][`AlignConstr`].
///
/// # Notes
///
/// * "alignment constraint archetype" is a
/// [stipulative](https://www.ucfmapper.com/education/various-types-definitions/#:~:text=Stipulative%20definitions)
/// [functional](https://www.ucfmapper.com/education/various-types-definitions/#:~:text=Functional%20definitions)
/// definition.
///
/// * [`AlignConstr<T, AlignConstrArchetype>`][`AlignConstr`] for some underlying type `T` and
/// "alignment constraint archetype" `AlignConstrArchetype` can also be seen as a
/// [refinement type](https://en.wikipedia.org/wiki/Refinement_type)
/// [reified](https://en.wikipedia.org/wiki/Reification_(computer_science)) in the form of a
/// [parameterized](http://www.angelikalanger.com/GenericsFAQ/FAQSections/ParameterizedTypes.html#FAQ001)
/// [newtype](https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html).
///
/// Unlike in [`aligned`] crate, [`Deref`][`core::ops::Deref`] and [`DerefMut`][`core::ops::DerefMut`]
/// are used not for accessing the underlying value but for dereferencing that value in case
/// **if** it is possible. Therefore, the following code should fail to compile:
///
/// ```rust, compile_fail
/// use align_constr::{AlignConstr, n_zst::ZST64};
///
/// fn deref_must_be_impossible_when_underlying_type_is_not_deref() {
///     let overaligned_u8 = AlignConstr::<u8, ZST64>::new(3);
///     // Since u8 doesn't implement Deref, neither does AlignConstr::<u8, ...>
///     let smth = *overaligned_u8;
/// }
/// ```
///
/// and the one below must succeed:
///
/// ```rust
/// use align_constr::{AlignConstr, n_zst::ZST128};
///
/// fn deref_is_performed_on_underlying_value() {
///     let overaligned_u8_ref = AlignConstr::<&u8, ZST128>::new(&3);
///     // Since &u8 implements Deref, so does AlignConstr::<&u8, ...>
///     assert_eq!(*overaligned_u8_ref, 3);
/// }
///
/// deref_is_performed_on_underlying_value();
/// ```
///
/// The underlying value can be accessed via the `.value` field and
/// it can be done in constant contexts even on stable Rust!
///
/// ```
/// use align_constr::{AlignConstr, n_zst::ZST64};
///
/// const fn underlying_value_can_be_accessed_via_value_field() {
///     let overaligned_u8 = AlignConstr::<u8, ZST64>::new(3);
///     assert!(overaligned_u8.value == 3u8);
/// }
///
/// underlying_value_can_be_accessed_via_value_field();
/// ```
///
/// Here's a comprehensive list of traits that could be expected
/// to be implemented for [`AlignConstr`]
/// constantly under [feature flags](https://doc.rust-lang.org/beta/unstable-book/)
/// and non-constantly on stable Rust, yet it is not the case:
///
/// * [~~core::marker::Copy~~][core::marker::Copy]
/// > [`core::marker::Copy`] cannot be implemented for `T: `[`core::marker::Copy`] on
/// > [`AlignConstr`]`<T, AlignConstrArchetype>` due to the implementation thereof.
/// >
/// > At the time of writing, [`AlignConstr`] relies on a zero-length array
/// > `[AlignConstrArchetype;0]`; but
/// > [zero-length arrays are non-`Copy`](https://github.com/rust-lang/rust/issues/94313)
/// > and there is no known alternative for constraining the alignment other than
/// > `#[repr(align(N))]` where N is an integer literal.
/// >
/// > The author of [`aligned`] crate hasn't opened an issue when found this shortcoming.
/// * [`core::ops::Index`]`<Idx>`
/// > WIP (Work in Progress)
/// >
/// > The author of [`aligned`] crate implemented the trait only for
/// > `Idx = `[`RangeTo`][core::ops::RangeTo]`<`[`usize`]`>`.
///
/// [`aligned`]: https://crates.io/crates/aligned
// repr(C) enforces the order of fields
#[repr(C)]
pub struct AlignConstr<T, AlignConstrArchetype>
where
    T: ?Sized,
{
    _alignment_constraint: [AlignConstrArchetype; 0],
    pub value: T,
}

// At the time of writing, `aligned` crate used the following constructor
// for the analogous `Aligned` generic type:
//
// ```rust
// #[allow(non_snake_case)]
// pub const fn Aligned<A, T>(value: T) -> Aligned<A, T> {
//     Aligned {
//         _alignment: [],
//         value,
//     }
// }
// ```
//
// While shorter, it is believed to be currently non-idiomatic.
//
// Moreover, the alternative (the code below) can be rewritten as constant
// implementation of New<T> trait
impl<T, AlignConstrArchetype> AlignConstr<T, AlignConstrArchetype> {
    pub const fn new(value: T) -> AlignConstr<T, AlignConstrArchetype> {
        AlignConstr {
            _alignment_constraint: [],
            value,
        }
    }
}

// At the time of writing, `aligned` crate unconditionally
// non-constantly implemented Deref for
// accessing the `value` field.
#[cfg_attr(
    all(feature = "const_trait_impl", feature = "const_fn_trait_bound"),
    remove_macro_call
)]
unconst_trait_impl! {
    impl<T,AlignConstrArchetype> const core::ops::Deref for AlignConstr<T,AlignConstrArchetype>
    where
        T: ?Sized + ~const core::ops::Deref,
    {
        type Target = <T as core::ops::Deref>::Target;

        fn deref(&self) -> &Self::Target {
            self.value.deref()
        }
    }
}

// At the time of writing, `aligned` crate unconditionally
// non-constantly implemented DerefMut for
// accessing the `value` field.
#[cfg_attr(
    all(
        feature = "const_trait_impl",
        feature = "const_fn_trait_bound",
        feature = "const_mut_refs"
    ),
    remove_macro_call
)]
unconst_trait_impl! {
    impl<T, AlignConstrArchetype> const core::ops::DerefMut for AlignConstr<T, AlignConstrArchetype>
    where
        T: ?Sized + ~const core::ops::DerefMut,
    {
        fn deref_mut(&mut self) -> &mut <T as core::ops::Deref>::Target {
            self.value.deref_mut()
        }
    }
}

// The purpose of this implementation is still unclear to the author
#[cfg_attr(
    all(feature = "const_trait_impl", feature = "const_fn_trait_bound"),
    remove_macro_call
)]
unconst_trait_impl! {
    impl<T, AlignConstrArchetype> const core::ops::Index<core::ops::RangeTo<usize>>
        for AlignConstr<[T], AlignConstrArchetype>
    where
        [T]: ~const core::ops::Index<core::ops::RangeTo<usize>, Output=[T]>,
    {
        type Output = <[T] as core::ops::Index<core::ops::RangeTo<usize>>>::Output;

        fn index(&self, range: core::ops::RangeTo<usize>) -> &Self::Output {
            // The unsafe block has been this way in `aligned`
            // TODO: figure out the intention and fix the code.
            unsafe {
                &*(&self.value[range] as *const [T] as *const Self::Output )
            }
        }
    }
}

#[cfg_attr(
    all(feature = "const_trait_impl", feature = "const_fn_trait_bound"),
    remove_macro_call
)]
unconst_trait_impl! {
    impl<T, AlignConstrArchetype> const ::as_slice::AsSlice for AlignConstr<T, AlignConstrArchetype>
    where
        T: ~const ::as_slice::AsSlice,
    {
        type Element = T::Element;

        fn as_slice(&self) -> &[T::Element] {
            self.value.as_slice()
        }
    }
}

#[cfg_attr(
    all(feature = "const_trait_impl", feature = "const_fn_trait_bound"),
    remove_macro_call
)]
unconst_trait_impl! {
    impl<T, AlignConstrArchetype> const ::as_slice::AsMutSlice for AlignConstr<T, AlignConstrArchetype>
    where
        T: ~const ::as_slice::AsMutSlice,
    {
        fn as_mut_slice(&mut self) -> &mut [T::Element] {
            self.value.as_mut_slice()
        }
    }
}

#[cfg_attr(
    all(feature = "const_trait_impl", feature = "const_fn_trait_bound"),
    remove_macro_call
)]
unconst_trait_impl! {
    impl<T, AlignConstrArchetype> const Clone for AlignConstr<T, AlignConstrArchetype>
    where
        T: ~const Clone + ~const Drop
    {
        fn clone(&self) -> Self {
            Self {
                _alignment_constraint: [],
                value: self.value.clone(),
            }
        }

        fn clone_from(&mut self, source: &Self) {
           self.value = source.value.clone();
        }
    }
}

#[cfg_attr(
    all(feature = "const_trait_impl", feature = "const_fn_trait_bound"),
    remove_macro_call
)]
unconst_trait_impl! {
    impl<T, AlignConstrArchetype> const Default for AlignConstr<T, AlignConstrArchetype>
    where
        T: ~const Default,
    {
        fn default() -> Self {
            Self {
                _alignment_constraint: [],
                value: T::default(),
            }
        }
    }
}

#[cfg_attr(
    all(feature = "const_trait_impl", feature = "const_fn_trait_bound"),
    remove_macro_call
)]
unconst_trait_impl! {
    impl<T, AlignConstrArchetype> const core::fmt::Debug for AlignConstr<T, AlignConstrArchetype>
    where
        T: ~const core::fmt::Debug,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            self.value.fmt(f)
        }
    }
}

#[cfg_attr(
    all(feature = "const_trait_impl", feature = "const_fn_trait_bound"),
    remove_macro_call
)]
unconst_trait_impl! {
    impl<T, AlignConstrArchetype> const core::fmt::Display for AlignConstr<T, AlignConstrArchetype>
    where
        T: ~const core::fmt::Display,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            self.value.fmt(f)
        }
    }
}

#[cfg_attr(
    all(feature = "const_trait_impl", feature = "const_fn_trait_bound"),
    remove_macro_call
)]
unconst_trait_impl! {
    impl<T, AlignConstrArchetype> const PartialEq for AlignConstr<T, AlignConstrArchetype>
    where
        T: ~const PartialEq,
    {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value
        }
    }
}

#[cfg_attr(
    all(feature = "const_trait_impl", feature = "const_fn_trait_bound"),
    remove_macro_call
)]
unconst_trait_impl! {
    impl<T, AlignConstrArchetype> const Eq for AlignConstr<T, AlignConstrArchetype>
    where
        T: ~const Eq,
    {
        // The following is implemented due to
        // error: const trait implementations may not use non-const default functions
        fn assert_receiver_is_total_eq(&self) {}
    }
}

#[cfg_attr(
    all(feature = "const_trait_impl", feature = "const_fn_trait_bound"),
    remove_macro_call
)]
unconst_trait_impl! {
    impl<T, AlignConstrArchetype> const core::hash::Hash for AlignConstr<T, AlignConstrArchetype>
    where
        T: ~const core::hash::Hash,
    {
        fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
            self.value.hash(state);
        }

        // The following is implemented due to
        // error: const trait implementations may not use non-const default functions
        fn hash_slice<H>(data: &[Self], state: &mut H)
        where
            Self: Sized,
            H: ~const core::hash::Hasher,
        {
            let mut i = 0;
            while i < data.len() {
                data[i].hash(state);
                i+=1;
            }
        }
    }
}

#[cfg_attr(
    all(feature = "const_trait_impl", feature = "const_fn_trait_bound"),
    remove_macro_call
)]
unconst_trait_impl! {
    impl<T, AlignConstrArchetype> const core::cmp::Ord for AlignConstr<T, AlignConstrArchetype>
    where
        T: ~const Ord,
    {
        fn cmp(&self, other: &Self) -> core::cmp::Ordering {
            self.value.cmp(&other.value)
        }
    }
}

#[cfg_attr(
    all(feature = "const_trait_impl", feature = "const_fn_trait_bound"),
    remove_macro_call
)]
unconst_trait_impl! {
    impl<T, AlignConstrArchetype> const core::cmp::PartialOrd for AlignConstr<T, AlignConstrArchetype>
    where
        T: ~const PartialOrd,
    {
        fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
            self.value.partial_cmp(&other.value)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        n_zst::{ZST1, ZST128, ZST16, ZST2, ZST256, ZST32, ZST4, ZST512, ZST64, ZST8},
        AlignConstr,
    };

    // This function tests the assumption about the relation
    // between core::mem::align_of::<u8>() and core::mem::align_of::<ZST512>().
    //
    // Other tests might fail to check the intended behevior if this test fails.
    #[test]
    const fn align_of_u8_le_align_of_zst512() {
        use core::mem::align_of;
        assert!(align_of::<u8>() < align_of::<ZST512>());
    }

    #[test]
    const fn check_alignments_of_n_zsts() {
        use core::mem::align_of;

        assert!(align_of::<ZST1>() == 1);
        assert!(align_of::<ZST2>() == 2);
        assert!(align_of::<ZST4>() == 4);
        assert!(align_of::<ZST8>() == 8);
        assert!(align_of::<ZST16>() == 16);
        assert!(align_of::<ZST32>() == 32);
        assert!(align_of::<ZST64>() == 64);
        assert!(align_of::<ZST128>() == 128);
        assert!(align_of::<ZST256>() == 256);
        assert!(align_of::<ZST512>() == 512);
    }

    #[test]
    const fn align_constr_allows_overaligning() {
        use core::mem::align_of;

        assert!(align_of::<AlignConstr::<u8, ZST512>>() > align_of::<u8>());
    }

    #[test]
    const fn align_constr_doesnt_reduce_alignment() {
        use core::mem::align_of;

        assert!(align_of::<AlignConstr::<ZST512, u8>>() == align_of::<ZST512>());
    }

    #[test]
    const fn check_new() {
        let _overaligned_u8 = AlignConstr::<u8, ZST128>::new(3);
    }
}
