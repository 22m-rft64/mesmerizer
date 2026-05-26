use crate::error::MesError;

pub fn resolve(category: &str) -> Result<(&'static str, &'static str), MesError> {
    match category {
        "pwn" | "pwn/userland" => Ok(("solve.py", PWN_USERLAND)),
        "pwn/kernel" => Ok(("solve.c", PWN_KERNEL)),
        "crypto" | "crypto/math" => Ok(("solve.sage", CRYPTO_SAGE)),
        "crypto/comp" => Ok(("solve.py", CRYPTO_PY)),
        "crypto/guess" => Ok(("solve.py", CRYPTO_PY)),
        "rev" | "rev/userland" => Ok(("solve.py", REV_PY)),
        "web" | "web/client-side" | "web/server-side" | "web/browser" => Ok(("solve.py", WEB_PY)),
        "misc" => Ok(("solve.py", MISC_PY)),
        other => Err(MesError::NotFound(format!("scaffold category: {other}"))),
    }
}

const PWN_USERLAND: &str = r#"#!/usr/bin/env python3
from pwn import *

HOST = "TODO"
PORT = 0
BIN = "./chal"

context.binary = BIN
context.log_level = "info"

def conn():
    if args.REMOTE:
        return remote(HOST, PORT)
    if args.GDB:
        return gdb.debug(BIN)
    return process(BIN)

def main():
    io = conn()
    # TODO: exploit
    io.interactive()

if __name__ == "__main__":
    main()
"#;

const PWN_KERNEL: &str = r#"// Kernel exploit skeleton
// Build: gcc -static -o solve solve.c -no-pie
// Then transfer to target.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/ioctl.h>
#include <sys/mman.h>
#include <stdint.h>

int main(void) {
    // TODO: open device, prepare overlap / spray / overwrite
    return 0;
}
"#;

const CRYPTO_SAGE: &str = r#"#!/usr/bin/env sage
# Crypto challenge solver (sage)

from Crypto.Util.number import long_to_bytes, bytes_to_long
import sys

# TODO: load chal params

def solve():
    pass

if __name__ == "__main__":
    solve()
"#;

const CRYPTO_PY: &str = r#"#!/usr/bin/env python3
# Crypto challenge solver (pure python / compute-heavy)

from Crypto.Util.number import long_to_bytes, bytes_to_long
from sympy import isprime, gcd

# TODO: load chal params

def solve():
    pass

if __name__ == "__main__":
    solve()
"#;

const REV_PY: &str = r#"#!/usr/bin/env python3
# Reverse engineering helper

import struct
import sys

# TODO: analyze target

def main():
    pass

if __name__ == "__main__":
    main()
"#;

const WEB_PY: &str = r#"#!/usr/bin/env python3
# Web challenge solver

import requests

BASE = "TODO"
session = requests.Session()

# TODO: implement attack chain

def main():
    pass

if __name__ == "__main__":
    main()
"#;

const MISC_PY: &str = r#"#!/usr/bin/env python3
# Misc challenge solver

# TODO: load chal

def main():
    pass

if __name__ == "__main__":
    main()
"#;
