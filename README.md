# numbercrunching

[![Release](https://img.shields.io/github/release/Ascor8522/number_crunching?display_name=tag&sort=semver&style=for-the-badge)](https://github.com/Ascor8522/number_crunching/releases)
[![License](https://img.shields.io/github/license/Ascor8522/number_crunching?style=for-the-badge)](https://github.com/Ascor8522/number_crunching/blob/master/LICENSE.md)


A simple implementation to solve [subset sum problems](https://en.wikipedia.org/wiki/Subset_sum_problem).

See the [related Reddit post](https://www.reddit.com/r/programmingrequests/comments/wr8i85/looking_to_have_what_i_believe_is_a_fairly_simple/).

## Compile the code

To compile the code, run the following command in a command prompt:

	cargo build

You need to have the [Rust toolchain installed](https://rustup.rs/).

## Run the program

To run the program, open a command prompt and type:

	numbercrunching.exe <path_to_file_with_the_numbers> [the_sum_to_find, default=0]

Required parameters are surrounded by `<>` and optional parameters are surrounded by `[]`.

* The file containing the numbers should have one number per line.
* The numbers should all be decimal (as in "not integers").
* The numbers must have two (and only two) digits after the decimal point.
* Empty lines are supported (e.g. between the numbers, and at the end of the file).

## Examples

### Default use case (sum = 0)

File with the numbers (`numbers.txt`):

```txt
-2.00
-1.00
1.00
2.00
3.00
```

	numbercrunching.exe numbers.txt

Result:

```txt
[-1.00, 1.00]
[-2.00, 2.00]
[-1.00, -2.00, 3.00]
```

> **Note**: the result doesn't include "duplicates", as per [OP's request](https://www.reddit.com/r/programmingrequests/comments/wr8i85/comment/ikswivk/).
>
> E.g. `[-2.00, -1.00, 1.00, 2.00]` shouldn't be included, since both `[-2.00, 2.00]` and `[-1.00, 1.00]` are already included in the results.

### Custom sum

File with the numbers (`numbers.txt`):

```txt
-2.00
-1.00
1.00
2.00
3.00
```

	numbercrunching.exe numbers.txt 1.0

Result:

```txt
[1.00]
[-1.00, 2.00]
[-2.00, 3.00]
```
