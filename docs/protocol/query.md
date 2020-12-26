---
has_children: false
layout: default
title: Querying Data
nav_order: 3
---
# Queries

> ## ⚠️ Warning ⚠️
> #### The documentation you are reading is neither implemented, nor finalised. please do not begin an inplementation of the protocol until this message has been removed. Thank you.


### Table of Contents
- [GET](#get)
- [PRUNE](#prune)
- [INSERT](#insert)
- [MODIFY](#modify)
- [Responses](#responses)


## Get
In order to query values from a given database/table, the following example query should be used as reference. All fields in this query are required:
```
GET my_database.A
FIELDS: "name", "email", "pass"
```

#### Joins
With the internal query protocol for kalavar, there are four types of join to choose from, `Inner`, `Outer` (`Full`), `Left`, and `Right`. The differences between these join types will be explained below.

__Inner join__ returns all rows matching the filter criteria, where the row in table `A`, has a partnering row in table `B`.
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

## Prune
In order to remove values from table, the client must send a `PRUNE` query, a type of query which allows the client to specify the type of logic matching they wish to achieve. The filters are always prepended with a number, matching them to any paired logical operations (see below example for further clarification).
```
PRUNE my_database.A
FILTERS: 1:"name"=="John Doe"
AND 1:"pass"=="abc123"
OR 2:"pass"==NUll
```

## Insert
When the client wishes to insert values into a table, the following fields must be present, `FIELDS`, `VALUES`
```
INSERT my_database.A
FIELDS: "name", "email", "pass"
VALUES: "John Doe", "email@provider.tld", "abc123"
"John Doe", "email@provider.tld", "abc123"
"John Doe", "email@provider.tld", "abc123"
```

## Modify
Should the client wish to update the values of a table:
```
MODIFY my_database.A
FILTERS: "name"=="John Doe"
"pass"=="abc123"
LOGIC: AND/OR
VALUES: "abc122", "abc133", "abc321"
```


## Responses

### Phase 1:
> Sent immeditately after receiveing and parsing a query. contains two fields<br>
> `SNOWFLAKE` - (Always present) The `QueryID`. __KEEP THIS STORED__<br>
> `POSITION` - (Nullable) The (estimated) position in the query queue. This is not a guarantee, and should not be used for tracking query information.
```
SNOWFLAKE: 882b46ec
POSITION: 10
```
Example with `NULL` position:
```
SNOWFLAKE: 882b46ec
POSITION: NULL
```

### Phase 2:
> Sent once the query has been processed, clients are expected to recognise a `QueryID` and return the data for the correct request accordingly.
```
RESPONSE <query ID>
COLUMNS: "name":"type"
"name":"type"
"name":"type"
ROWS: John Doe|email@provider.tld|abc123
John Doe|email@provider.tld|abc123
John Doe|email@provider.tld|abc123
John Doe|email@provider.tld|abc123
John Doe|email@provider.tld|abc123
```

[Previous](auth.md)

[Next](data-types.md)

