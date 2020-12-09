# arc
DVCS system written in Rust  
CSC 253: Collaborative Programming and Software Design  
Aceves, Zheng, Zimmerman  

## Required installation of Rust and Cargo
On Linux and macOS systems, this is done as follows:

````
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
````
Follow the directions on screen to conclude installation.

## How to run 
````
$ cargo run
````

## Commands available
### init
Initializes a repository in the user's current working directory.

````
$ cargo run init
````

It can be used with optional argument <repo-path> to initialize repository elsewhere.
  
````
$ cargo run init <repo-path>
````

### print
Prints out the status of the current repository. 

````
$ cargo run print
````

### clone
Clones repository from given path into the user's current working directory.
````
$ cargo run clone <repo-path>
````

It can be used with optional argument <destination-path>. 
````
$ cargo run clone <repo-path> <destination-path>
````

### add
Adds file to tracking list.
````
$ cargo run add <file-path>
````

### remove
Removes file from tracking list.
````
$ cargo run remove <file-path>
````

### checkout
Checkout to specific revision.
````
$ cargo run checkout <revision-name>
````

### commit
Commits current changes. 
````
$ cargo run commit
````

### merge
Merges two revisions. 
````
$ cargo run merge <revision-name> <revision-name>
````

### push
Pushes current commit(s). 
````
$ cargo run push
````

### pull
````
$ cargo run pull 
````
