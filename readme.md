# ddbug output parser

This project exists to take [ddbug][0] output text files (based on C projects) and convert them to JSON so that you can create your own custom programs based on the generated JSON & DWARF file data.

With the `ddbug` parser output files serialized to Rust Structs, which then uses `serde` to serialize the data to machine parsable JSON format.

This project is also being used as a way for me to learn the Rust language string features, so the code will probably be sloppy, inefficient and not cleanest. So keep in mind that this project is a 50% learning exercise for myself.

## Notes
** IMPORTANT** Thios tool was developed using C based embedded project. It will not work with C++ or Rust based elf files.

**IMPORTANT**, this tool only works with `-p all` option. The `-p all` option makes `ddbug` output all the extra fields that it can parse.

Also note that "unit" field needs to be present because each function, base type, variable, enum belongs to a translation unit. Without the "unit" field, its much harder to associate functions, types, structures, enums, variables, etc to a specific unit.

The original ddbug tool simply dumps all this info in order for each translation unit which creates a ton duplicates. Therefore, I intentionally structured everything in a hierarchial manner so that each of the functions, variables, types, etc belong to specific translation unit.

# How to use

First, go grab *[ddbug][0]* from the repo and install it. Once installed, run the following command to generate a "output_dump.txt" file.

```ddbug -p all YOUR_BINARY_ELF.elf > output_dump.txt```

**IMPORTANT**, this tool only works with `-p all` option. The `-p all` option makes `ddbug` output all the extra fields that it can parse.

```
DDbug C to JSON 0.1
Yoofie <yoofie@gmail.com>
This project exists to take ddbug output text files and convert them to Rust structures so that you can create your own custom programs based on DWARF file data.

With the `ddbug` parser output files serialized to Rust Structs, you can also use serde to serialize the data to machine parsable JSON format.

USAGE:
    ddbugC_tojson [OPTIONS] --input <INPUT_FILE>

OPTIONS:
    -d, --rdbg                         Prints out the internal Rust structures
    -h, --help                         Print help information
    -i, --input <INPUT_FILE>           The input file into this tool. This file should have been
                                       generated from the ddbug tool
    -o, --output [<OUTPUT_FILE>...]    The name of the generated output file.  [default: output.json]
    -V, --version                      Print version information
```

## Example usage
Example #1 - Just plain JSON output that will create output.json in the same directory where this exe is run from: 
```
ddbugC_tojson -i input_file.txt
```
Example #2 - Plain JSON ouput like in example #1 except with a custom file name
```
ddbugC_tojson -i input_file.txt -o output_file.json
```

Example #3 - Display Rust struct debug output:
```
ddbugC_tojson -d -i input_file.txt -o output_file.json
```

# Todo list
 - [x] Functions not fully supported (parameters, etc are missing)
 - [x] arrays/union memebers not yet supported
   - Currently supported but the datastructure is flattened and not nested
 - [ ] Each translation unit currently uses vectors that can be empty. Convert these to Options
 - [x] ~~`padding` fields curently return a error. Make it so that they are left blank instead~~
 - [x] Integrate CLI interface 

## Wishlist maybe?
- [ ] Make tool more flexible so that `-p all` option is not necessary. If this becomes a goal, then I need to figure out a way to associate each function, type, enum, etc to belong to a translation unit when the unit data is not available.

[0]:https://github.com/gimli-rs/ddbug
