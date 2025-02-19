use arrow::array::{Array, Float64Array, Int32Array, StringArray};

pub trait ArrowPrimitive: Sized {
    fn from_array(array: &dyn Array, index: usize) -> Option<Self>;
}

impl ArrowPrimitive for i32 {
    fn from_array(array: &dyn Array, index: usize) -> Option<Self> {
        array
            .as_any()
            .downcast_ref::<Int32Array>().map(|arr| arr.value(index))
    }
}

impl ArrowPrimitive for f64 {
    fn from_array(array: &dyn Array, index: usize) -> Option<Self> {
        array
            .as_any()
            .downcast_ref::<Float64Array>().map(|arr| arr.value(index))
    }
}

impl ArrowPrimitive for String {
    fn from_array(array: &dyn Array, index: usize) -> Option<Self> {
        array
            .as_any()
            .downcast_ref::<StringArray>().map(|arr| arr.value(index).to_string())
    }
}
