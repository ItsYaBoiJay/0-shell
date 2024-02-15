# Simple Shell Project
This project is a simple Unix shell implemented in Rust. The shell provides a basic command-line interface and supports a set of built-in commands. It is designed to be a minimalistic implementation, similar to BusyBox, and does not include advanced features such as pipes, redirection, or scripting.

## Features
Displays a simple prompt ($) and waits until you type a command line, which will be validated by pressing enter.

Executes built-in commands and system commands.

Supports the following built-in commands:
- echo
- cd
- ls
- pwd
- cat
- cp
- rm
- mv
- mkdir
- exit

Handles errors and displays appropriate error messages\.
>
Manages program interruption with Ctrl + D.
>
### How to use the program
To use the shell, simply run the executable. You will be presented with a prompt where you can enter commands. The shell will execute the commands and display the output.

> cd into shell and then;
`cargo run`

which will show this\: ~/Desktop/0-shell/shell $ 

### Example Commands
- Echo\: echo Hello, World! (can also creat txt files echo txt > txt.file)
- Change Directory\: cd /path/to/directory
- List Directory\: ls -l, -a, -F
- Print Working Directory\: pwd
- Concatenate Files\: cat file.txt
- Copy File\: cp (source) (destination) (be specific)
- Remove File\: rm -r file.txt (can also use -f)
- Move File\: mv (source) (destination) (can also rename files/folders)
- Make Directory\: mkdir new_directory (-p for parent folder)
- Exit Shell\: exit

##### Error Handling
The shell handles errors by displaying appropriate error messages to the user. If a command fails, the shell will display an error message and continue to accept input.