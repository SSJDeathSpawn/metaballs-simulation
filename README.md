# Metaballs Simulation

## What is a Metaball?

According to [Wikipedia](https://en.wikipedia.org/wiki/Metaballs),

> In computer graphics, metaballs, also known as blobby objects, are organic-looking n-dimensional isosurfaces, characterised by their ability to meld together when in close proximity to create single, contiguous objects. 

More simply, they are circular objects that deform to stick to other metaballs.

## How to make metaballs?

You have a metaball function which looks similar to the circular function, but in the form $f(x) \geq 1$, 
which results in 
$$\frac{r^2}{(x-h)^2 + (y-k)^2} \geq 1$$

where $r$ is the radius and $(h,k)$ is the centre of the circle

When you add up all the contributions and sum is above a threshold (in this case, 1), you draw.

Equation:
$$\sum f(x,y) \geq 1$$

Example when two metaballs exist:
$$\frac{r_1^2}{(x-h_1)^2 + (y-k_1)^2} + \frac{r_2^2}{(x-h_2)^2+(y-k_2)^2} \geq 1$$

## In Practice

In this specific implementation, the data of the metaballs (their radius and centre) is passed to the fragment shader using Shader Storage Buffer Objects (SSBO), more specifically std430.

In the shader, I'm checking if the distance of a fragment (in this implementation, equivalent to a pixel) from the circles and seeing if the contribution adds up to more than 1. If it does, I'm coloring the fragment with the weighted average of the color of the individual metaball according to the contribution of their $f(x,y)$.

To make the metaballs appear like rings, limit the draw only to if the contributions add up to greater than 1, but lesser than some higher number (in this case 1.2).

## Output

![Metaballs](output.gif)

## Instructions for running

There is a `flake.nix` file provided if you use the Nix package manager. Otherwise a simple `$ cargo run` should work, provided you have libsdl2 installed. If you do not or don't want to, you can add the `bundled` feature flag to sdl2 which should cause cargo to compile SDL2 independently and link to that.
