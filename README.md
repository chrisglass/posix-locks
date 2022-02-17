posix-locks
===========

This is a very simple repository demonstrating how to create mutually excluding
POSIX file locks between go, python, and rust.

Notes about POSIX locks vs. flocks
-----------------------------------

POSIX locks are created and held between the *file* and the *PID* holding the
lock, which means that a single golang process will be considered a single
*actor* no matter how many goroutines it spins up

Instead, using the flock (syscall.Flock(...)) syscall will create a (non-POSIX)
lock between the *file descriptor* and the *file*, meaning two goroutines
Open()ing the same file separately will be able to deadlock (they each are
considered separate actors).

Using this repository
---------------------

### Building the golang program

Simply invoking `go build` at the root of this project should create a
posix-locks binary you can then run.
It will grab a hardcoded lock in the current directory forever (until killed).

### Running the python program

As for the golang program, the python3 script will acquire a lock on a hardcoded
lock in the root directory of this project, and sleep forever (until killed).

### Running the rust program

As for previous programs, the rust version will acquire a lock on a hardcoded
file in the root directory of this project, and sleep forever (until killed).

### Results

Running one or the other programs will fail if the other program is currently
running.

While one of the programs is running, a POSIX lock is held, as shown with e.g.
the `lslocks` command.
