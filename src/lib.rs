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
}

/// Alignment-constrained datatype, i.e. a type whose
/// [alignment](https://www.geeksforgeeks.org/data-structure-alignment/)
/// is constrained not only by inherent alignment requirements of the underlying type but also
/// by the alignment requirements of the "alignment constraint archetype".
/// 
/// **Note**: "alignment constraint archetype" is a
/// [stipulative](https://www.ucfmapper.com/education/various-types-definitions/#:~:text=Stipulative%20definitions)
/// [functional](https://www.ucfmapper.com/education/various-types-definitions/#:~:text=Functional%20definitions)
/// definition.
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
