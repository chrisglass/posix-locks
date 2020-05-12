package main

import (
	// "io"
	"log"
	"os"
	"syscall"
	"time"
)

const (
	filename = "test.lock"
)

func main() {
	file, err := os.Create(filename)
	if err != nil {
		log.Printf("error opening file: %s", err)
		return

	}
	defer file.Close() // Closing the file will release the lock.

	flockT := syscall.Flock_t{
		Type:   syscall.F_WRLCK,
		Whence: 0,
		Start:  0,
		Len:    0,
	}
	// Passing the syscall.F_SETLKW syscall insteat (mind the W) will let
	// the program wait for the lock to be available (blocking) before it
	// proceeding.
	err = syscall.FcntlFlock(file.Fd(), syscall.F_SETLK, &flockT)
	if err != nil {
		log.Printf("Could not acquire lock! %s", err)
		return

	}

	log.Print("lock acquired, sleeping forever...")
	for {
		time.Sleep(time.Second * 2)
	}
}
