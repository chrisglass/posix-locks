#!/bin/env python3
import fcntl
import os
import sys
import time

FILENAME = "test.lock"

try:
    with open(FILENAME, "wb") as target_file:
        # You can make the lock blocking by removing the LOCK_NB part.
        # The program will then wait until the lock is availabe before proceeding.
        fcntl.lockf(target_file, fcntl.LOCK_EX | fcntl.LOCK_NB)

        # We wait forever keeping the current process running doing nothing to hold
        # onto the lock until killed.
        print("Lock acquired, sleeping forever!")
        while True:
            time.sleep(2)

except BlockingIOError:
    print("Could not acquire lock!")
    exit(1)
