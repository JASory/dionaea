Application for storing/reading unstructured numerical data. 

If a text file consists solely of integers or floats then it is more space-efficient to store the values as binary data. It is also more efficient for programs to read binary data directly rather than parsing text. 

# Datatypes
32-bit, 64-bit, 128-bit integers, 32-bit and 64-bit float

# Writing
This application can read from stdin and pipes and converts the text to binary data of the selected type.
Writing is halted when 'q' is read from  stdin. Note that if reading from a pipe, values that can't be stored in the datatype will be skipped with an error message (to stderr) rather than halting the read. 
# Reading 
Data is read from the selected file and written out to stdout 
