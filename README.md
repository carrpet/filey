# filey 
'filey' is cool new file editor that does stuff with files.

## Setup 

## Installation 

## Usage 

### Commands

#### Create
```sh
$ filey create <file_name>
```
This command will create a file named <file_name> with default permissions in the current directory.
If an existing file with the same name exists, it will not overwrite it.

```sh
$ filey create -t "this is some text for my new file" <file_name>
```
The -t option can be specified to create the file with the text specified.

#### Copy
```sh
$ filey copy <source_file> <dst_file>
```
#### Concatenate
```sh
$ filey cat <source_file1>...<source_fileN> <dst_file>
```
This command concatenates the contents of a list of one or more source files named <source_file1> ... to <source_fileN> to a new location <dst_file>.  If a file exists with the same name as <dst_file> then the command fails.

#### Delete
```sh
$ filey del <source_file>
```
