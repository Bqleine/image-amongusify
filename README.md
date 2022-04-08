# Image amongusifier
Will render your image "sus".

## What does it do?
This program will fill the input image with small amoguses discretely, with the right settings, they will almost be invisible if you don't zoom (which is the point)

## Usage
The command usage is :
```
./image-amongusify <input file name> <output file name> [sensibility] [alt color offset]
```
Sensibility is a number between 0 and 255 that will allow the program to skip drawing amonguses on places that show a lot of different colors. Smaller means less amonguses (default=255)

Alt color offset is the color offset between the amonguses body (which is determined by the average of the pixels around it) and the color of his visor. A bigger value will make the amonguses more distinguishable in the image but will make the image have a grid pattern appear from afar (default=5)

## Examples
Original:
![original](examples/original.jpg)
Amongusified:
![amongusified](examples/amongusified.png)
*Hint: Try zooming in*