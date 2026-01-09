# Incremental Move Generation Performance Results

Walking the move generation tree of strictly legal moves to count all the leaf nodes of a certain depth, starting from the standard chess initial position with bulk-counting. (https://www.chessprogramming.org/Perft)

**Simple move generation without check detection**

| Depth | Nodes      | Time (ms) | Expected   | Difference  |
|:-----:|-----------:|----------:|-----------:|------------:|
|     1 |         20 |         0 |         20 |          +0 |
|     2 |        400 |         0 |        400 |          +0 |
|     3 |       8902 |         6 |       8902 |          +0 |
|     4 |     197742 |       109 |     197281 |        +461 |
|     5 |    4896998 |      2671 |    4865609 |      +31389 |
|     6 |  120909581 |     62006 |  119060324 |    +1849257 |