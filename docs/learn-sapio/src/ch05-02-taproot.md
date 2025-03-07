# Taproot

Currently Taproot is scheduled to active on Bitcoin in November 2021 but there
is limited support for it across all ecosystem tools.

Sapio scripts can become very large in size, and would greatly benefit from
being able to split up and merkelize the logic into smaller satisfiable
chunks. This makes it economical to use Sapio.

The compiler is currently relatively naive about this, and unknown (or worse,
unchecked) errors might occur as a result of pushing these limits. Hopefully,
`rust-miniscript` should catch such errors, but a malicious author might be
able to trigger an unknown unsatisfiable script.

Without full Taproot support, Sapio is probably ill-advisable to use at writing,
but this will hopefully change in the immediate future.


## Taproot Optimizations

With Taproot comes the opportunity to [Huffman
Code](https://en.wikipedia.org/wiki/Huffman_coding) spending paths to
decrease fees even further. Sapio currently uses `rust-miniscript` Policy
language to generate spending conditions, so Sapio should be able to carry
metadata from the programmer about the likelihood of various paths being
taken, but this currently only is used within a script as opposed to the
Tapscript tree itself.