# sortuniq

`sortuniq` provides optimised versions of various `| sort | uniq`
constructions which are common in shell scripting. This construction
finds all the unique lines in an input, regardless of the line order.

For example:
```
1
2
3
2
1
```

becomes:

```
1
2
3
```


### Usage

Without arguments, `| sortuniq` generates the same *set* of results
as `| sort | uniq`, and produces them immediately. This is, uh,
slightly disconcerting. They are *not* sorted, unlike in the original.

`| sortuniq -c` generates the similar output to `| sort | uniq -c`.


### `local`

`| sortuniq --local` will drop "local" duplicates from a stream, again
immediately. This can be useful if your data is very large, and has many
useless values in, and you only want the eventual `uniq` values, or some
other idea of the stream. For example, for some data which looks like:

```
one
two
ponies!
two
one
one
horses!
two
one
ponies!
```

`| sortuniq --local --size-hint 3` will (immediately) print:

```
one
two
ponies!
horses!
ponies!
```

It has eliminated many of the `one` and `two` entries, but can't
eliminate the second `ponies!` as it runs out of "memory". You can
(arbitrarily) increase the `size-hint` (default: `64`).

You can't really do that with `uniq`, which can't look more than
one line back in the history.


## Indicative benchmarks

I took a gigabyte of rendered Wikipedia pages, and extracted the "words",
giving >200 million lines.

For this input:

 * `| sortuniq -c` takes 17.5s (single core) and 190MB of memory (max RSS)
 * `| sort | uniq -c` takes 111s (405s total user time) and around 4gb of memory.

That's a 6x-23x speedup, and a 21x memory improvement.

---

Here's a subset of the input (via. [wikiextractor](https://github.com/attardi/wikiextractor)
and `| perl -pe 's/\b/\n/g;s/[ \t]//g' | egrep -v '^$'`:

```
Helena
Carroll
Helena
Winifred
Carroll
(
13
November
1928
–
31
March
2013
)
was
a
veteran
film
,
television
and
stage
actress
.
```

The most common words are, unsurprisingly:

```
   1195445 by
   1229458 with
   1274390 on
   1360706 as
   1405234 for
   1416371 '
   1675168 is
   1777549 The
   1931603 -
   2074956 was
   3038885 "
   3462954 a
   3630791 to
   4267812 in
   4999315 and
   5732322 of
   8336207 .
   9153796 ,
  10748102 the
```
