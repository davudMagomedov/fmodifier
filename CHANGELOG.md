This file contains all the changes made to the **FModifier** project.

The log template is shown below.
```
## VERSION - YYYY-MM-DD
Basic description of what this change is for.
#### Added
- Items
#### Removed
- Items
#### Fixed
- Items
#### Changed
- Items
```
## 0.1.1 - Unreleased
#### Added
- Simple CLI.
- File execution.
- Comments.
- Command `exit`.
- Command `help`.
- Command `turn_buffer_to_file`. This command, surprisingly, turns the buffer into a file.
- Command `turn_file_to_buffer`. This command, surprisingly, turns the file into a buffer. File must be opened.
- A little flexibility for commands. For example, `fill_buffer some_buffer 0xFF` will not raise error but behave like `fill_buffer some_buffer 0xFF 0 <buffer_size>` (fill whole buffer by the value).
#### Fixed
- Improved *README* file.
- Bug with infinity loop using non-terminal *stdin stream*.

## 0.1.0
The project just created.
