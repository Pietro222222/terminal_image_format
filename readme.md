
# Colors
these are the colors you can use when displaying images on the terminal
```
BYTES:
5A = BLUE
5B = BLACK
5C = RED
5D = GREEN
5E = PURPLE
5F = WHITE
60 = YELLOW
61 = CYAN
```

# How it works

```
####################################
# name             OFFSET  BYTES   #
# HEADER           | 0     | 5 |   #
# ---------------------------------#
# Pixels Per Line  | 6     | 1 |   #
# ---------------------------------#
# Pixel Color      | 7     | 1 |   # 
# ---------------------------------#
# Number of pixels | 8     | 1 |   #
####################################
```
<br>
NOTE: Pixel Color and Number of Pixels can be place as much as you want. example:

```
2E 54 49 46 20 0A 60 0F 5B 0F
```
output: <br>
![example](https://i.imgur.com/KVuTk7V.png)


