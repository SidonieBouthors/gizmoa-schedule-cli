# Gizmoa Schedule CLI

This is a small CLI tool that takes a `.csmo` schedule save from [Gizmoa Schedule Maker](https://gizmoa.com/college-schedule-maker/) and outputs an `.svg` image of the schedule.

## Usage

```
cargo run -- schedule.csmo
```

## Why ?

I often use this online schedule maker to make a schedule of my semester. 
It is simple but has all the features I need, the only downside is that the quality of the exported image is not quite as good as I would want.

So, as my first Rust mini project, I have made a CLI tool that parses the .csmo save file (that you can download from the website), and outputs an .svg image of your schedule.

The main aspects of this project are JSON parsing and SVG generation, but it was mostly something small and useful I could make to get startd with Rust.
