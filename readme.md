## Quadratic search
A quadratic search implement in rust  

before use this crate please benchmark with your data if binary-search faster please use binary-search!

**This crate use unsafe! but safe as built-in binary-search**

### How to use?
basicly you can replace `slice.binary_search(&needle)` with `slice.quadratic_search(&needle)` (they should yield same result)
```rust
let heystack = vec![1,2,3,4,5,6,7,8,9,10];
let needle = 4;
// your IDE should find import for you 😅
heystack.quadratic_search(&needle);
```

### How it work?
what it does is compare and move cursor twice per loop

rust binary search may implement like this
```rust
loop{
    let mid = (right + left) >> 1;
    if elem[mid] < search {
        left = mid + 1;
    } else if elem[mid] > search {
        right = mid;
    } else {
        return Ok(mid);
    }
}
```
quadratic search will do this
```rust
loop{
    let mid = (right + left) >> 1;
    if elem[mid] < search {
        left = mid + 1;
    } else if elem[mid] > search {
        right = mid;
    } else {
        return Ok(mid);
    }
    let mid = (right + left) >> 1;
    if elem[mid] < search {
        left = mid + 1;
    } else if elem[mid] > search {
        right = mid;
    } else {
        return Ok(mid);
    }
}
```

my bench result (searching u128 in vec with 8M element)
```
$ cargo bench
test tests::bin_search ... bench:          60 ns/iter (+/- 0)
test tests::qua_search ... bench:          57 ns/iter (+/- 0)

# CPU: Xeon E5 2650v3 3Ghz
```