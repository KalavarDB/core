---
has_children: false
layout: default
title: Joining Tables
parent: Querying Data
grand_parent: Protocol
nav_order: 2
---

# Joins
With the internal query protocol for kalavar, there are four types of join to choose from, `Inner`, `Outer` (`Full`), `Left`, and `Right`. The differences between these join types will be explained below.

### Inner join returns all rows matching the filter criteria, where the row in table `A`, has a partnering row in table `B`.
An inner join is represented by the presence of 2 `FIELDS` keys, and an `I-JOIN` key
```
GET my_database.A
FIELDS: "name", "email", "pass"
I-JOIN my_database.B
FIELDS: "banned", "two_factor", "admin"
```

__Left join__ returns all rows matching the filter criteria, from table `A`, as well as a partner row from table `B` if available.
A left join is represented by the presence of 2 `FIELDS` keys, and an `L-JOIN` key
```
GET my_database.A
FIELDS: "name", "email", "pass"
L-JOIN my_database.B
FIELDS: "banned", "two_factor", "admin"
```

__Right join__ returns all rows matching the filter criteria, from table `B`, as well as a partner row from table `A` if available.
A right join is represented by the presence of 2 `FIELDS` keys, and an `R-JOIN` key
```
GET my_database.A
FIELDS: "name", "email", "pass"
R-JOIN my_database.B
FIELDS: "banned", "two_factor", "admin"
```

__Outer/Full join__ returns all rows, regardless of if the row in table `A`, has a partnering row in table `B`.
An outer/full join is represented by the presence of 2 `FIELDS` keys, and an `O-JOIN` key
```
GET my_database.A
FIELDS: "name", "email", "pass"
O-JOIN my_database.B
FIELDS: "banned", "two_factor", "admin"
```