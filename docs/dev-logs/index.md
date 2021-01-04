---
has_children: false
layout: default
title: Dev Logs
nav_order: 2
---
# Development Log - Week 2 {#week-2}
#### Written by [Thomas B.](https://github.com/fatalcenturion) | Posted on Monday, January 4th 2021
## A busy week, and a few important milestones {#week-2-a-buys-week}
This past week, we achieved a number of first time events, which represent small victories in our battle against the computer to bring Kalavar to the public. First of these milestones was:
### The first inter-process communication {#week-2-first-communication}
This week we achieved the first inter-process communication of our program. This communication was produced by the REPL and CORE components of the database. The REPL component had been heavily modified at the time compared to the build on our [repository](https://github.com/fatalcenturion/kalavar-repl), so naturally, we pushed the build which achieved this to the repository for archival purposes.

### The first inter-thread communication {#week-2-first-thread}
Another major event that happened this week, which we dubbed our "RAMpocalypse". We managed to successfully track the amount of memory our CORE system was using at regular runtime intervals, allowing us to report that to the main system thread for the program. This may seem trivial, but it is a big victory for us in bringing Kalavar closer to production. You may not agree with us, but actually it is the primary method of the program deciding when to move things into and out of system memory to and from the disk.

## Parting Words {#week-2-parting-words}
Although this weeks blog post was a short one, it was always going to be, we've been taking a break over the Christmas period, and although our last post was actually released on Christmas day, our writer had nothing better to do, nd so they threw it together for us. Work on the project since the 20th of December 2020 has been slow and uneventful up until this week, and we are hoping to ramp up our progress on the project as soon as we can. 

Thanks for reading this weeks development log, we hope you have enjoyed it!

Like what you see? Star us on [GitHub](https://github.com/fatalcenturion/kalavar-core) to let us know!

----- 

# Development Log - Week 1 {#week-1}
#### Written by [Thomas B.](https://github.com/fatalcenturion) | Posted on Monday, December 28th 2020
## Meet Kalavar {#week-1-meet-kalavar}

Hi everybody, in this dev log i wanted to take the time to introduce to you, our little project. We like to call it Kalavar and this is where you read the latest information and developer updates about the system, as we work our way towards a public alpha test release. We hope you enjoy this journey just as much as we will.

## What is Kalavar? {#week-1-what-is-kalavar}
Kalavar is an asynchronous, efficient, and secure database system, programmed using the [Rust](https://rust-lang.org) programming language and built upon the [Tokio](https://tokio.rs/) runtime.

## What does it offer that X does not? {#week-1-offerings}
Kalavar, once ready for a release candidate, will be able to offer a full featured SQL-less query environment, designed to provide asynchronous access to your data, via encrypted data streams. What does that actually mean? well, it means that Kalavar, simply out-classes the competition via the sheer power of asynchronous programming. Queries can be organised and optimised before  being submitted by the client for processing, this allows your queries to be as fast, efficient, and secure as you could ever wish for.

And, on top of this, Kalavar's official clients will also be capable of taking your regular old SQL queries* and transforming them into faster, more efficient KQL queries, before sending them off for processing!

<sup>*SQL transformation limited to PostgreSQL queries. Not all features will be available at time of release. these features will be provided via a downloadable update once they are ready for use</sup>

## Parting words {#week-1-parting-words}
Well, that's it folks, the first developer log is now finished, we hope you enjoyed reading our plans for Kalavar, and we also ask you to keep an eye on this space for more information, as development goes further.

For now though, goodbye from the Kalavar team.

Like what you see? Star us on [GitHub](https://github.com/fatalcenturion/kalavar-core) to let us know!