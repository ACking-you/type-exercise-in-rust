use self::iterator::ArrayIterator;

pub mod iterator;
pub mod primitive_array;
pub mod string_array;
/// [`Array`] is a collection of data of the same type.
pub trait Array: Send + Sync + Sized + 'static {
    /// The corresponding [`ArrayBuilder`] of this [`Array`].
    ///
    /// We constriant the associated type so that `Self::Builder::Array = Self`.
    type Builder: ArrayBuilder<Array = Self>;

    /// The owned item of this array.
    type OwnedItem: 'static + std::fmt::Debug;

    /// Type of the item that can be retrieved from the [`Array`]. For example, we can get a `i32`
    /// from [`Int32Array`], while [`StringArray`] produces a `&str`. As we need a lifetime that is
    /// the same as `self` for `&str`, we use GAT here.
    type RefItem<'a>: Copy + std::fmt::Debug;

    /// Retrieve a reference to value.
    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>>;

    /// Number of items of array.
    fn len(&self) -> usize;

    /// Indicates whether this array is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Get iterator of this array.
    fn iter(&self) -> ArrayIterator<Self>;
}

/// [`ArrayBuilder`] builds an [`Array`].
pub trait ArrayBuilder {
    /// The corresponding [`Array`] of this [`ArrayBuilder`].
    ///
    /// Here we use associated type to constraint the [`Array`] type of this builder, so that
    /// `Self::Array::Builder == Self`. This property is very useful when constructing generic
    /// functions, and may help a lot when implementing expressions.
    type Array: Array<Builder = Self>;

    /// Create a new builder with `capacity`.
    fn with_capacity(capacity: usize) -> Self;

    /// Append a value to builder.
    fn push(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>);

    /// Finish build and return a new array.
    fn finish(self) -> Self::Array;
}

#[cfg(test)]
mod tests {
    use super::{primitive_array::PrimitiveArray, string_array::StringArray, *};

    fn build_array_from_vec<T: Array>(items: &[Option<T::RefItem<'_>>]) -> T {
        let mut builder = T::Builder::with_capacity(items.len());
        for v in items {
            builder.push(*v);
        }
        builder.finish()
    }

    fn assert_array_eq<'a, T: Array>(array: &'a T, items: &[Option<T::RefItem<'a>>])
    where
        T::RefItem<'a>: PartialEq,
    {
        for (a, b) in array.iter().zip(items.iter()) {
            assert_eq!(&a, b);
        }
    }

    #[test]
    fn test_primitive() {
        let array: PrimitiveArray<i32> = build_array_from_vec(&[None, Some(3), Some(2)]);
        assert_array_eq(&array, &[None, Some(3), Some(2)]);
    }
    #[test]
    fn test_string() {
        let items = &[None, Some("abc"), Some("ab"), Some("abb")];
        let array: StringArray = build_array_from_vec(items);
        assert_array_eq(&array, items);
    }
}
