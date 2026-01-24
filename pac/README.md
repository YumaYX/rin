# pac

This program reads a text file line by line and builds a hierarchical structure based on a specified trigger string. Lines that contain the trigger become parent nodes, and subsequent lines become their child nodes until another trigger line appears. After constructing this hierarchy, the program prints only the parent nodes that contain a specified keyword, along with their associated child lines.
It is useful for analyzing logs, structured text, or any file where sections are separated by identifiable markers.

## Usage

### Command format

```sh
program <file> <trigger-text> <keyword>
```

### Arguments
- file
  - Path to the text file to read
- trigger-text
  - Marker used to detect and start a new parent node
- keyword
  - Used to filter which parent nodes will be printed

## install

```sh
cargo install --path .
```
