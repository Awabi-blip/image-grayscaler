## Datatyps:
**RGB:**
-defines a struct that represents a pixel in a bitmap image, r= Red, b= Blue, g = Green, each having 8 bits of unsigned integers, because color value can not be in negatives.

**Image:**
-defines a struct for an image, that stores the width and the height of the image.
-stores pixels in a 1D Array because the rust compiler does not like undefined 2D arrays where it has to figure the size at compile time

## Why manual threading?
I am using manual threads because this was my first ever time writing rust, and I wanted to learn the ins and outs of concurrency at a fundamental level. In production you'd always use built-in rust methods

## The 2D Array formula:
**y * image.width + x**:
-What this does it, takes our current position of y, so let's say 3 and multiplies it with the width so let's say 5, and adds the x value
so let's say maybe, 3
-This is how 2d Arrays work underneath the hood too:
If I declared a **2D Array of [10][5]** it just means 10 rows, of 5 columns, if I then did a lookup using arr[3][5] it would need a way to mathematically calculate where to go, because in computers Ram there is no structure like 2D array unless explicitly defined, so everything is defined as single line of pixels. So like [rgb,rgb,rgb,rgb,rgb] * 10 which means total of 50 items! so if when i say, arr[3][3] I mean, go to the 3 * width which is 5, and add 3 to it so 18 which 
This is how the 3rd element in the 3rd row would look like: 
notice how arr[3,3] points to 18? it is implemented like a series of numbers in the memory but multiplying by the width give us the jumps in rows (you can think of it this way) and then add to the value to get the exact number on that jump/row
[
[0,1,2,3,4]
[5,6,7,8,9]
[10,11,12,13,14]
[15,16,17,18,19]....
]

**PS this was my first time working with Rust, so don't mind the lac of idiomacy!**
