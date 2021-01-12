# kalavar-core
A database written purely using Rust (WIP)

This project is under heavy development and does not expect to see any releases until at the very soonest, February 2021. Please keep an eye on this space for updates.


You can view the current specifications and documentation [here](https://kalavar.cf/documentation)

## Helping Test Kalavar:

Whilst there are currently no active testing events ongoing at this time, you are welcome to clone this repository and launch the dev kit in your terminal. You can do so by running `bash test.sh`, where you will be greeted with the following menu:
```
---------------------------------
       Kalavar Dev Kit V0.1      
---------------------------------
1.   Check Dependencies
2.   Build and run
3. Δ Build and run as super user
4.   Build and run tests
5. Δ Purge target directory
6.   Install Dependency Checker
7.   Exit
---------------------------------
 Δ - requires SUDO priveleges
---------------------------------
Enter your choice [1-7] : 
```
This dev kit is intended for use on unix systems, as that is what I use to develop and test Kalavar on, and this kit is primarily meant to make my life easier.

> Note: The dev kit requires that you have Cargo in your PATH, and that you have Rust and all of it's dependencies installed correctly.

If you have any feedback about the developer kit, or you want to ask for new commands, please open an issue with the "dev kit" tag.
