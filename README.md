# aoc24
Solutions to AoC24 in rust

Posible arguments of cargo run:
- -t {true or false}: if runs with test input, default = false
- -b {true or false}: use bruteforce if implemented, default = false
- -i {0..}: number of iterations to estimate computation time (0 means no estimation), default = 0
- -fd {1..25}: from which day to compute puzzles, default = 1
- -ld {1..25}: to which day to compute puzzles, default = 25

What to learn:
- **Day 01**. Vec, HashMap, itertools:izip, Result<T,E>; concepts of trait and dynamic dispatch; reading from a file
- **Day 02**. Option, match, iter(), skip(), take(), chain(), collect()
- **Day 03**. Option, Result, match, find(), rfind()
- **Day 04**. Enum, Struct, Hash, filter(), flat_map(), map(), operator overload, generic trait, closure
- **Day 05**. lifetime parameter, operator overload, or_insert_with()
- **Day 06**. HashSet, next(), filter(), map_or(), par_windows(), loop, nested closure
- **Day 07**. Enum with arguments, BTreeMap, passing function as argument, format!, pow(), log(), floor(), par_iter()
- **Day 08**. HashMap and HashSet, iterators and loops