# Prusa Slicer Web API

### What is this?
This is an API made on Actix to determinate the time to slice some STL in PrusaSlicer with a particular config.
Endpoint receives:
- STL File 
- INI File
As form data with ANY key at "host:3080/slice"
Endpoint returns:
The response is a JSON with this format
```json

{
  "response": {
  "filament_used_mm":"f32",
  "filament_used_cm3": "f32",
  "filament_used_g": "f32",
  "estimated_printing_time": "String",
  "slicer_output": "String"
  }
}

```
### How to run
All working on docker, just:
```d
docker build -t "slicer_api:dockerfile" .
```
Or go to the code(slicer/src) and compile it using, given slicer file was build for linux.
```r
cargo build --release.
```
Then copy the data to dockerfile directory and build again.

### How to debug
On container debug data is the time for each step, from writing it to the FS to return time.
I suggest you to run it in virtual memory as this post says:
https://stackoverflow.com/questions/39193419/docker-in-memory-file-system
Just because it writes, slices and dispose the config, the stl and the gcode generated, might wear SSD on long term
Or just read the code, not that hard, it's less than 200 lines lol
### Why I did this?
It's part of a quoting system for 3d makers, in order to make an estimate of the price of the object, we need to know the consumed filament and the time it takes for our machine with our custom config.
## TODO:
- Slice multiple stl files with single config








