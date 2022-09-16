# Example Instructions

First run the following:
```
ddbug -p all Example_LPC54114_Project.axf > output_raw_dump.txt
```

Then run:
```
ddbugC_tojson -i output_raw_dump.txt -o output_file.json
```
The final resulting file is `output_file.json`
# Example output
See You can see example input file and the example generated JSON file. This example input/output is based on a pure C microcontroller project using the LPC 54114 MCU (made with NXPs MCU Expresso IDE).

The input file (from [ddbug][1]) is `input_file.txt` which also the same as `output_raw_dump.txt`.

The generated output file will look like `output_file.json`.
**Please note this tool only works on C only projects. C++ is _not_ supported**

[1]:https://github.com/gimli-rs/ddbug
