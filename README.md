# Datatypes:
**RGB:**
- defines a struct that represents a pixel in a bitmap image, r= Red, b= Blue, g = Green, each having 8 bits of unsigned integers, because color value can not be in negatives.

**Image:**
- defines a struct for an image, that stores the width and the height of the image.
- stores pixels in a 1D Array because the rust compiler does not like undefined 2D arrays where it has to figure the size at compile time

# Why manual threading?
I am using manual threads because this was my first ever time writing rust, and I wanted to learn the ins and outs of concurrency at a fundamental level. In production you'd always use built-in rust methods

# The 2D Array Formula: `y * image.width + x`

What this does is take our current position of `y`, so let's say 3, and multiply it with the width, so let's say 5, and add the `x` value, so let's say maybe 1.

This is how 2D arrays work underneath the hood too. A 2D array is not a struct, like you may assume it is, that is made by the language at compile time. It is simply an array that has a specialised indexer. Let me explain further:

Let's declare the array as `DECLARE arr[3][5]` — so 3 rows (top to bottom) and 5 columns (left to right):

**Table View / Human View:**
```
      i:  0   1   2   3   4
row 0  [  0,  1,  2,  3,  4 ]
row 1  [  5,  6,  7,  8,  9 ]
row 2  [ 10, 11, 12, 13, 14 ]
```

This is how we would look at a 2D array. But the computer wouldn't — for a computer it merely looks like this in memory (RAM/Disk):

**Computer's View in RAM:**
```
[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]    metadata: width = 5
```

Now let's apply our function `y * width + x`. Let's say I query `arr[2][3]`:

```
      i:  0   1   2   3   4
row 0  [  0,  1,  2,  3,  4 ]
row 1  [  5,  6,  7,  8,  9 ]
row 2  [ 10, 11, 12, 13, 14 ]
```

You can tell that is going to be the 2nd row, 3rd column, and that value is **13** (starting from 0 for each row).

Now let's apply the formula. The computer always treats values as `[row][column]`, where the first value is `row[y]` (vertical axis) and the second is `column[x]` (horizontal axis):

```
arr[2][3]  →  y = 2,  x = 3,  width = 5

y * width + x  =  2 * 5 + 3  =  13 ✓
```

That's all.
  
**PS this was my first time working with Rust, so don't mind the lack of idiomacy!**

# Result looks like this:
## Before:
<img width="1306" height="710" alt="Screenshot From 2026-03-25 09-48-21" src="https://github.com/user-attachments/assets/ffd48776-b120-430a-b067-5c2706a5c5cb" />

## After:
<img width="1306" height="710" alt="Screenshot From 2026-03-25 09-48-37" src="https://github.com/user-attachments/assets/2253c333-2bfa-45f4-acbd-211eab964c7e" />

//Devnote: Doktor turn off my pain inhibitors
