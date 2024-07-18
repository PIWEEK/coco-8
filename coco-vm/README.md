# coco-vm

The COCO-8 virtual machine is built on top of a COCO-8 CPU and uses devices in the fashion of [Varvara](https://wiki.xxiivv.com/site/varvara.html) and NES.

The COCO-8 CPU has a 256-byte device page, that contains 16 devices with 16 bytes for ports. Some of the ports take just one byte, but others take a short (2 bytes).

| Address | Device                   |
| ------- | ------------------------ |
| `0x00`  | [System](#system-device) |
| `0x10`  | Video                    |

## System device

<table>
  <tr><th><code>0x00</code></th><td rowspan="2"><i>unused*</i></td></tr>
  <tr><th><code>0x01</code></th></tr>
  <tr><th><code>0x02</code></th><td>debug</td></tr>
</table>

Instead of a customizable vector, the system is always to be assumed to have `0x100` as the address of its vector (which is the reset vector and it's called when the ROM is booted).

Sending a non-zero byte to the **`debug` port** will ouput CPU debug information.

## Video device

<table>
  <tr><th><code>0x10</code></th><td rowspan="2">vector*</td><th><code>0x18</code></th><td rowspan="2">address*</td></tr>
  <tr><th><code>0x11</code></th><th><code>0x19</code></th></tr>
  <tr><th><code>0x12</code></th><td>x</td><th><code>0x1a</code></th><td>sprite</td></tr>
  <tr><th><code>0x13</code></th><td>y</td><th><code>0x1b</code></th><td>--</td></tr>
  <tr><th><code>0x14</code></th><td>pixel</td><th><code>0x1c</code></th><td>--</td></tr>
  <tr><th><code>0x15</code></th><td>--</td><th><code>0x1d</code></th><td>--</td></tr>
  <tr><th><code>0x16</code></th><td>--</td><th><code>0x1e</code></th><td>--</td></tr>
  <tr><th><code>0x17</code></th><td>--</td><th><code>0x1f</code></th><td>--</td></tr>
</table>

The **screen <code>vector\*</code>** is called at a rate of 60 fps, and it's meant to run any drawing operations.

The ports `x*` and `y*` contain the X and Y coordinates used by the drawing or buffer reading operations: `pixel`, `read` and `sprite`.

The **`pixel` port** is used to put pixels into the video buffer. It follows this layout:

<table>
  <tr>
    <th><code>7</code></th>
    <th><code>6</code></th>
    <th><code>5</code></th>
    <th><code>4</code></th>
    <th><code>3</code></th>
    <th><code>2</code></th>
    <th><code>1</code></th>
    <th><code>0</code></th>
  </tr>
  <tr>
    <td>flip x</td>
    <td>flip y</td>
    <td>fill</td>
    <td>layer</td>
    <td colspan="4">color</td>
  </tr>
</table>

- `color` is an index in the 16-color COCO-8 palette, and can range from `0` to `f`.
- `layer` indicates the layer to put the pixel in; background is `0` and foreground is `1`. Color `0` in the foreground will be considered transparent.
- `fill` sets wether to draw a single pixel, `0`, or to use the `x` and `y` coordinates as the position of a rectangle to fill, `1`. By default it takes the bottom right quadrant.
- `flip x` will use a quadrant on the left for filling. It has no effect when `fill` is `0`.
- `flip y` will use a quadrant on the top for filling. It has no effect when `fill` is `0`.

Some examples:

```uxn
PUSH 60 PUSH 12 DEO # x = 0x60
PUSH 48 PUSH 13 DEO # y = 0x48
PUSH 18 PUSH DEO # put pixel with color 0x8 in the foreground
```

```uxn
PUSH 00 PUSH 12 DEO # x = 0x00
PUSH 00 PUSH 13 DEO # y = 0x00
PUSH 30 PUSH 13 DEO # fills the foreground with transparent color
```

The **`sprite` port** is used to draw sprites (or tiles). A sprite a 8x8 pixel image, with 4 bits per pixel. Writing to this port will take the sprite addressed by the **`address` port** paint it at the coordinates set by the **`x` and `y` ports**.

<table>
  <tr>
    <th><code>7</code></th>
    <th><code>6</code></th>
    <th><code>5</code></th>
    <th><code>4</code></th>
    <th><code>3</code></th>
    <th><code>2</code></th>
    <th><code>1</code></th>
    <th><code>0</code></th>
  </tr>
  <tr>
    <td>flip x</td>
    <td>flip y</td>
    <td>1bpp</td>
    <td>layer</td>
    <td colspan="4">color</td>
  </tr>
</table>

> **TODO**: `flip x`, `flip y`, `1bpp` and `color` are currently unimplemented as per 2024-07-18.

Sprite example:

```
00777700
07777770
67177176
7f7777f7
77111177
77728777
76777767
76077067
```

![Sprite screenshot](../docs/sprite_screenshot.png)
