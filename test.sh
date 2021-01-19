#!/bin/bash

# display message and pause
pause() {
  local m="$@"
  echo "$m"
  read -p "Press [Enter] key to continue..." key
}

# set an
while :; do
  # show menu
  clear
  echo "+----------------------------------+"
  echo "|      Kalavar Dev Kit V0.1        |"
  echo "+----------------------------------+"
  echo "| 1.   Check Dependencies          |"
  echo "| 2.   Check source code           |"
  echo "| 3.   Build and run               |"
  echo "| 4. Δ Build and run as super user |"
  echo "| 5.   Build and run tests         |"
  echo "| 6.   Build internal docs         |"
  echo "| 7. Δ Purge target directory      |"
  echo "| 8.   Install Dependency Checker  |"
  echo "| 9.   Exit                        |"
  echo "+----------------------------------+"
  echo "| Δ - requires SUDO priveleges     |"
  echo "+----------------------------------+"
  read -r -p "Enter your choice [1-9] : " c
  # take action
  case $c in
  1)
    version-checker
    echo
    pause
    ;;
  2)
    cargo check
    echo
    pause
    ;;
  3)
    cargo run
    echo
    pause
    ;;
  4)
    cargo build && sudo target/debug/kalavar-core
    echo
    pause
    ;;
  5)
    cargo test --all --jobs 1 -- --nocapture
    echo
    pause
    ;;
  6)
    cargo doc --no-deps --target-dir ./docs
    echo
    pause
    ;;
  7)
    sudo rm -r ./target
    echo
    pause
    ;;
  8)
    cargo install version-checker
    echo
    pause
    ;;
  9) break ;;
  *) pause "Select between 1 to 8 only" ;;
  esac
done
