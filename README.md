**FModifier** is a file modifier that allows you to operate directly with bytes.
## Quick Guide
Let's run the shell with the command `fmodifier`.
```bash
fmodifier
```
Next, let's create a buffer. A buffer is simply a collection of bytes over which numerous operations can be performed.
```fmodifier
make_buffer some_buffer 100
```
In code above, we created a buffer of 100 bytes filled with zeros. Let's make sure.
```fmodifier
show_buffer some_buffer 0 100
```
This code outputs byte starting at 0 and ending at 100:
```
Table:
0  ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
16 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
32 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
48 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
64 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
80 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
96 ┃ 00 00 00 00
```
Now let's assume that for some reason we need to set the 5th byte (*counting start from zero*) to the value 13.
```fmodifier
buffer_set_byte some_buffer 5 13
```
And now if we execute `show_buffer some_buffer 0 100`, we would get the following output.
```
0  ┃ 00 00 00 00 00 0d 00 00 00 00 00 00 00 00 00 00
16 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
32 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
48 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
64 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
80 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
96 ┃ 00 00 00 00
```
If you didn't see the changes, look at the 5th byte, it has the value *0x0d* instead of *0x00* now.
Now I want to fill bytes 10 to 30 with the value `0xFF`. To complete this task, run the code below.
```fmodifier
fill_buffer some_buffer 0xFF 10 30
```
This command will do exactly what we intended. Now, if you want to view the contents of the buffer, then enter the command you already know.
```
0  ┃ 00 00 00 00 00 0d 00 00 00 00 ff ff ff ff ff ff
16 ┃ ff ff ff ff ff ff ff ff ff ff ff ff ff ff 00 00
32 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
48 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
64 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
80 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
96 ┃ 00 00 00 00
```
Now, let's turn this buffer into a file.
<!-- TODO: Replace this code into one command turn_buffer_to_file -->
```fmodifier
create_file some_file 100
from_buffer_to_file some_buffer some_file 100 0 0
```
I understand that it can be hard to take but the second command in code above just copies 100 bytes from the buffer to the file created in the first command starting at 0 in the buffer and at 0 in the file. Look on [commands](#commands).
## Example. Creating MBR partition
Suppose we want to make an MBR partition. We have a binary file that contains the necessary code.
```
00000000  7f 45 4c 46 02 01 01 00  00 00 00 00  00 00 00 00
00000010  01 00 3e 00 01 00 00 00  00 00 00 00  00 00 00 00
00000020  00 00 00 00 00 00 00 00  78 01 00 00  00 00 00 00
00000030  00 00 00 00 40 00 00 00  00 00 40 00  0a 00 09 00
00000040 [55 48 89 e5 c7 45 fc 0a  00 00 00 83  45 fc 01 b8
00000050  00 80 00 00 8b 55 fc 66  89 10 eb ef] 00 47 43 43
00000060  3a 20 28 47 4e 55 29 20  31 33 2e 32  2e 30 00 00
00000070  14 00 00 00 00 00 00 00  01 7a 52 00  01 78 10 01
00000080  1b 0c 07 08 90 01 00 00  1c 00 00 00  1c 00 00 00
00000090  00 00 00 00 1c 00 00 00  00 41 0e 10  86 02 43 0d
000000a0  06 00 00 00 00 00 00 00  00 00 00 00  00 00 00 00
000000b0  00 00 00 00 00 00 00 00  00 00 00 00  00 00 00 00
000000c0  01 00 00 00 04 00 f1 ff  00 00 00 00  00 00 00 00
000000d0  00 00 00 00 00 00 00 00  00 00 00 00  03 00 01 00
...
```
The text above shows a part of the binary file in which the code we need is in square brackets. The bytes of this code start at index 0x40 and end at 0x5B.
We want to copy these bytes to the beginning of a 512 byte file, and at the end of this file we need to insert bytes 0x55 and 0xAA.
Below are the **FModifier**'s commands for creating such a file.
```
fmod> make_buffer mbr 512
- Buffer with name mbr and size 512 is created.

fmod> open_file main.o
- The file main.o is opened.

fmod> from_file_to_buffer main.o mbr 28 0x40 0x0
- Bytes of file main.o in the amount of 28 pieces were written to buffer mbr.

fmod> buffer_set_byte mbr 510 0x55
- Index 510 in buffer with name mbr was set to 85.

fmod> buffer_set_byte mbr 511 0xAA
- Index 511 in buffer with name mbr was set to 170.
```
That's it! Now let's print the result.
```
fmod> show_buffer mbr 0 512
Table:

0   ┃ 55 48 89 e5 c7 45 fc 0a 00 00 00 83 45 fc 01 b8 
16  ┃ 00 80 00 00 8b 55 fc 66 89 10 eb ef 00 00 00 00 
32  ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
48  ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
64  ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
80  ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
96  ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
112 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
128 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
144 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
160 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
176 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
192 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
208 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
224 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
240 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
256 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
272 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
288 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
304 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
320 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
336 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
352 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
368 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
384 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
400 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
416 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
432 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
448 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
464 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
480 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
496 ┃ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 55 aa 
512 ┃                                                 
```
Yes, it may be inconvenient to do ths kind of work every time, but for a simple check it will do fine.
## Basic knowledge
There are two types of data in FModifier: *buffers* and *files*. Files can only be created or opened read-only, whereas buffers are not part of the file system and serve to perform all operations on them.
When you realize that all the necessary operations on the buffer have been done, you can create a new file and write data from the buffer there.
## Commands
<a id="commands"></a>
After entering `fmodifier` in your terminal there'll be allowed following commands for you.
1. *It's not done yet* `help [<command>]` - prints help in general or for command.
2. `make_buffer <buffer_name> <buffer_size>` - creates a buffer named `<buffer_name>` and sized `<buffer_size>`.
3. `fill_buffer <buffer_name> <value> <start> <end>` - fills bytes from `<start>` inclusive to `<end>` not inclusive (bytes are counted from zero) of the buffer with the value `value`.
4. `show_buffer <buffer_name> <start> <end>` - outputs the contents of the buffer `<buffer_name>` to the console starting from `<start>` inclusive and up to `<end>` not inclusive (bytes are counted from zero).
5. `buffer_info <buffer_name>` - gives information about the buffer `<buffer_name>`.
6. `buffer_set_byte <buffer_name> <index> <value>` - sets the value of the index `<index>` of the buffer `<buffer_name>` to the value `<value>`.
7. `create_file <file_name> <file_size>` - creates a file with the name `<file>` and the size `<file_size>`.
8. `from_file_to_buffer <file_name> <buffer_name> <bytes_count> <file_start> <buffer_start>` - copies bytes in the amount of `<bytes_count>` pieces from the file `<file_name>` starting from the `<file_start>`th byte (bytes count from zero) to the buffer, which is being written starting from the `<buffer_start>` th byte.
9. `from_buffer_to_file <buffer_name> <file_name> <bytes_count> <buffer_start> <file_start>` - copies bytes in the amount of `<bytes_count>` pieces from the buffer `<buffer_name>` starting from the `<buffer_start>`th byte (bytes are counted from zero) to the file `<file_name>`, which is recorded starting from `<file_start>`.
10. `buffer_write_bytes <buffer_name> <start> <...bytes>` - writes a sequence of bytes `<...buffer>` (bytes are separated by a space) in the buffer `<buffer_name>` starting from the position `<start>` (bytes count starts from zero).
11. `pull_out_slice <buffer_name> <new_buffer_name> <start> <end>` - creates a new buffer `<new_buffer_name>` with the exact size of `<end> - <start>` bytes and writes there the values from the buffer `<buffer_name>` starting with `<start>` and ending with `<end>`.
12. `merge_buffers <left_buffer_name> <right_buffer_name> <new_buffer_name>` - creates a new buffer `<new_buffer_name>`, in which the first part of the bytes is copied from `<left_buffer_name>`, and the second part is copied from the buffer `<right_buffer_name>`. That is, the size of the new buffer is equal to the sum of the other two.
13. `open_file <file_name>` - opens a file named `<file_name>` in the current directory *strictly for reading*.
14. `show_file <file_name> <start> <end>` - reads a file named `<file_name>` starting with `<start>` and ending with `<end>`.
