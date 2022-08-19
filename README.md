# numbercrunching

A simple implementation to solve [subset sum problems](https://en.wikipedia.org/wiki/Subset_sum_problem).

See the [related Reddit post](https://www.reddit.com/r/programmingrequests/comments/wr8i85/looking_to_have_what_i_believe_is_a_fairly_simple/).

## Compile

	cargo build

## Run the program

To run the program, type:

	numbercrunching.exe <path to file with the numbers> [the sum to find]

Required parameters are surrounded by `<>` while optional parameters are surrounded by `[]`.

* The file containing the numbers should have one number per line.
* The numbers must have two digit after the decimal point.
* Empty lines are supported.

## Example

### Default use case

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

```txt
[1.00]
[-1.00, 2.00]
[-2.00, 3.00]
```

## To Do

* Improve error messages.
