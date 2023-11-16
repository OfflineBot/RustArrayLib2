# Custom Library for Arrays in Rust
## Implementing:
+ Custom Arrays for one/two dimensional Arrays
+ Linear Algebra for Arrays

## How to use:
Import Library with: `rarray = { git = "https://github.com/OfflineBot/RustArrayLib.git" }`.<br>
Then use `use rarray::Array1` to use one dimensional Arrays and `use rarray::Array2` for two dimensional Arrays.<br>
The Library is mainly focused on deep learning mathematical operations and for f32/f64 types.

## Containing:
Custom Implementation for one and two dimensional arrays.

## Functions:
### Array1
+ `new(size: usize)` creates new empty Array1 with given size
+ `from_vec(vec: Vec<T>)` creates new Array1 from Vector
+ `clean()` deletes Array1
+ `add(value: T)` adds a value to the Array1
+ `get(index: usize)` gets value on the index of Array1
+ `replace_zero(e_minus: T)` replaces all zeros with given `e_minus`
+ `set_index(index: usize, value: T)` replaces old value with new value on given index
+ `set_vec(vec: Vec<T>)` overrides whole Array1 with Vector data (must be same length)
+ `max()` gets the maximum number
+ `argmax()` gets the index of maximum number
+ `mean()` calulates the mean of Array1
+ `std()` calculates the standard deviation of Array1
+ `random_uniform(min: T, max: T)` replaces whole Array1 with random numbers between `min` and `max`
+ Array1 `+, -,*, /` Array1 or single value is implemented

### Array2


## Still not done:
+ Error handling
+ Better Info Texts