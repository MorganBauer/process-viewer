# process-viewer [![Build Status](https://travis-ci.org/GuillaumeGomez/process-viewer.png?branch=master)](https://travis-ci.org/GuillaumeGomez/process-viewer)
A process viewer GUI in rust. It provides current status of your processes (cpu and memory usage) and your system (usage of every core and of your RAM, and the temperature of your composants if this information is available).

# WARNING!

For now, it only builds on rust __nightly__!

![screenshot](http://guillaume-gomez.fr/image/screen1.png)
![screenshot](http://guillaume-gomez.fr/image/screen2.png)

It can be run on the following platforms:

 * Linux
 * Mac OSX
 * Windows

## A few notes on Windows installation

To install and run GTK+ on Windows, a good way to do it can be found
[here](http://gtk-rs.org/docs/requirements.html).

If needed, you can take a look at the `setup.cmd` file to install and setup environment. You'll
need to modify it to fulfill your needs.
