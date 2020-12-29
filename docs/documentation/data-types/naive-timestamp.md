---
has_children: false
layout: default
title: Naive Timestamp
parent: Data Types
grand_parent: Documentation
nav_order: 21
---

# Naive Timestamp
Stores a timestamp for future reference, does not store information about the timezone, assumes UTC+0

### Information
- Format: `YYYY MM DD-HH:MM:SS.xxx`
- Encoding: Binary

### Breakdown
|Example|Year|Month|Day|Hour|Min|Sec|extended|
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| 2021 01 01-00:00:00.000 |2021|Jan|01|0|0|0|000|
