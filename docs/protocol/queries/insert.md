---
has_children: false
layout: default
title: Inserting Data
parent: Querying Data
grand_parent: Documentation
nav_order: 3
---

# Insert

When the client wishes to insert values into a table, the following fields must be present, `FIELDS`, `VALUES`
```
INSERT my_database.A
FIELDS: "name", "email", "pass"
VALUES: "John Doe", "email@provider.tld", "abc123"
"John Doe", "email@provider.tld", "abc123"
"John Doe", "email@provider.tld", "abc123"
```