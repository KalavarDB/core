---
has_children: false
layout: default
title: Pruning Data
parent: Querying Data
grand_parent: Protocol
nav_order: 5
---
# Prune
In order to remove values from table, the client must send a `PRUNE` query, a type of query which allows the client to specify the type of logic matching they wish to achieve. The filters are always prepended with a number, matching them to any paired logical operations (see below example for further clarification).
```
PRUNE my_database.A
FILTERS: 1:"name"=="John Doe"
AND 1:"pass"=="abc123"
OR 2:"pass"==NUll
```