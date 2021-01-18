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
  echo "| 2.   Build and run               |"
  echo "| 3. Δ Build and run as super user |"
  echo "| 4.   Build and run tests         |"
  echo "| 5.   Build internal docs         |"
  echo "| 6. Δ Purge target directory      |"
  echo "| 7.   Install Dependency Checker  |"
  echo "| 8.   Exit                        |"
  echo "+----------------------------------+"
  echo "| Δ - requires SUDO priveleges     |"
  echo "+----------------------------------+"
  read -r -p "Enter your choice [1-8] : " c
  # take action
  case $c in
  1)
    version-checker
    echo
    pause
    ;;
  2)
    cargo run
    echo
    pause
    ;;
  3)
    cargo build && sudo target/debug/kalavar-core
    echo
    pause
    ;;
  4)
    cargo test
    echo
    pause
    ;;
  5)
    RUSTDOCFLAGS='-Z unstable-options --extend-css ./docs/theme.css' cargo doc --no-deps --target-dir ./docs
    echo
    pause
    ;;
  6)
    sudo rm -r ./target
    echo
    pause
    ;;
  7)
    cargo install version-checker
    echo
    pause
    ;;
  8) break ;;
  *) pause "Select between 1 to 8 only" ;;
  esac
done
