# Custom Library for Arrays in Rust
## Implementing:
+ Custom Arrays for one/two dimensional Arrays
+ Linear Algebra for Arrays

## How to use:
Import Library with: `rarray = { git = "https://github.com/OfflineBot/RustArrayLib.git" }`.<br>
Then use `use rarray::Array1` to use one dimensional Arrays and `use rarray::Array2` for two dimensional Arrays.<br>
The Library is mainly focused on deep learning mathematical operations and for f32/f64 types.

## Functions:
### Array1
+ `new(size: usize) -> Self` creates new empty Array1 with given size
+ `from_vec(vec: Vec<T>) -> Self` creates new Array1 from Vector
+ `clean(&mut self)` deletes Array1
+ `add(&mut self, value: T)` adds a value to the Array1
+ `get(&self, index: usize) -> T` gets value on the index of Array1
+ `replace_zero(self, e_minus: T)` replaces all zeros with given `e_minus`
+ `set_index(&self, index: usize, value: T)` replaces old value with new value on given index
+ `set_vec(&self, vec: Vec<T>)` overrides whole Array1 with Vector data (must be same length)
+ `max(&self) -> T` gets the maximum number
+ `argmax(&self) -> usize` gets the index of maximum number
+ `mean(&self) -> T` calulates the mean of Array1
+ `std(&self) -> T` calculates the standard deviation of Array1
+ `random_uniform(self, min: T, max: T)` replaces whole Array1 with random numbers between `min` and `max`
+ Array1 `+, -,*, /` Array1 or single value is implemented

### Array2


## TODO:
+ Error handling
+ Better Info Texts