---
has_children: false
layout: default
title: Modifying Data
parent: Querying Data
grand_parent: Protocol
nav_order: 4
---

# Modify
Should the client wish to update the values of a table:
```
MODIFY my_database.A
FILTERS: "name"=="John Doe"
"pass"=="abc123"
LOGIC: AND/OR
VALUES: "abc122", "abc133", "abc321"
```