# Teapot
The teapot go spin spin and you can WASD to move

I made this while learning [glium](https://github.com/glium/glium) which is an OpenGL wrapper for Rust

Music is [Lipps Inc. - Funkytown](https://www.youtube.com/watch?v=uhzy7JaU2Zc)

# Usage
Default (No parameters)
```bash
./spinning-teapot-but-you-can-wasd-to-move-linux
```
Custom (parameters)
```bash
./spinning-teapot-but-you-can-wasd-to-move-linux <amount> <range> <follow speed> <spawn speed>
```
Default parameters (10000 teapots with -64 - 64 range)
```bash
./spinning-teapot-but-you-can-wasd-to-move-linux 10000 64
```
one single teapot
```bash
./spinning-teapot-but-you-can-wasd-to-move-linux 1 0.1
```
Make a teapot that follows you :)
```bash
./spinning-teapot-but-you-can-wasd-to-move-linux 1 0.1 1
```
Spawn teapots over time
```bash
./spinning-teapot-but-you-can-wasd-to-move-linux 0 64 0 1
```