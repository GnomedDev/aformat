# 0.1.3

- `aformat` and `aformat_into` both now support expressions as arguments, allowing `aformat!("2 + 2 = {}", 2_u8 + 2)` to compile.
- The macros now avoid generating empty `push_str` calls.

# 0.1.2

- Swapped out internal implementation from `feature(generic_const_exprs)` to `typenum`-based, now supporting stable.

# 0.1.1

- Added `aformat_into`, allowing formatting into an existing `ArrayString`.
