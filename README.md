# numbercrunching

See the [related Reddit post](https://www.reddit.com/r/programmingrequests/comments/wr8i85/looking_to_have_what_i_believe_is_a_fairly_simple/)

## Compile

	cargo build

## Run the program

To run the program, type:

```bash
numbercrunching <path to file with the numebers>
```

The file containing the numbers should have one number per line.
The numbers must have two digit after the decimal point.
Empty lines are supported.

## To Do

* Remove "duplicates" results (the sets should be as small as possible).
E.g. `[[-2, 2], [-1, 1], [-2, -1, 1, 2]]` should be `[[-2, 2], [-1, 1]]`.
* Improve error messages.
