# Color Flipper

I decided to start this project when I was looking through [this list of
projects.](https://www.freecodecamp.org/news/javascript-projects-for-beginners/)
I was just a little bit bored and I wanted to do some programming. Recently, I
had also been thinking about the process of finding the inverse of a color, and
I suspected that it was as simple as flipping each bit of the hex code, and it
turns out that's pretty spot-on.

The app is made with Macroquad, a Rust library for simple 2D graphics. I chose
it because it advertised its simplicity, which is good for this kind of basic 2D
rendering.

The main functionality of the app is already done. To use it, just execute the
application and type in the hex code for a color. Note that it must be in the
format `#xxxxxx`, where `x` is a stand-in for a hexit (hexadecimal digit). Then,
press enter. If there was a problem parsing, nothing will happen. Otherwise, the
left half of the screen will display the color entered, and the right half will
display its inverse. Pretty simple!

## Future Plans

### Short Term

* Change label system to use text size rather than static measurements
* Modularize code
* Perform redundant call optimization

### Long Term

* Migrate UI to built-in macroquad UI system
