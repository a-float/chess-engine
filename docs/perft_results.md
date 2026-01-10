# Incremental Move Generation Performance Results

Walking the move generation tree of strictly legal moves to count all the leaf nodes of a certain depth, starting from the standard chess initial position. (https://www.chessprogramming.org/Perft) Ran with release profile.

**Simple move generation without check detection**

| Depth | Nodes                   | Captures                | En passant        | Castles         | Time (ms) |
|:-----:|------------------------:|------------------------:|------------------:|----------------:|----------:|
|     1 |                 20 (+0) |                  0 (+0) |            0 (+0) |          0 (+0) |         0 |
|     2 |                400 (+0) |                  0 (+0) |            0 (+0) |          0 (+0) |         0 |
|     3 |               8902 (+0) |                 34 (+0) |            0 (+0) |          0 (+0) |         0 |
|     4 |           197742 (+461) |               1579 (+3) |            0 (+0) |          0 (+0) |        15 |
|     5 |        4896998 (+31389) |            83678 (+959) |          0 (-258) |          0 (+0) |       346 |
|     6 |    120909581 (+1849257) |        2866846 (+54838) |         0 (-5248) |          0 (+0) |      6204 |
|     7 |  3282734510 (+86832650) |    112862988 (+4533062) |       0 (-319617) |     0 (-883453) |    149988 |
