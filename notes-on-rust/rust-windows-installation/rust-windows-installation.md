# Rust Windows Installation

Rust needs to be installed on the computer to be used for development of a Rust project. This section discusses installation of Rust on a computer running the Windows operating system.

## C++ Build Tools

A set of C++ Visual Studio build tools are required when developing in Rust. These can be downloaded from [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/).

When running the installer of the build tools there will be  a pane with components to choose to install. Choose at least the components in the following list:

- Desktop development with C++ (a package that Rust requires)
  - Included (by default)
    - C++ Build Tools core features
    - C++ 2022 Redistributable Update
    - C++ core desktop features
  - Optional (that shall be chosen due to required by Rust)
    - MSVC ...
    - Windows 11 SDK ...
    - C++ CMake tools for Windows
    - Testing tools core features - Build Tools
    - C++ AddressSanitizer

## Rustup

rustup is an installer for the programming language Rust. rustup provides installers for multiple operating systems, including Windows. rustup is also used to update Rust to new versions as well as to uninstall Rust

### Rustup Rust Installation

Searching for the term "rustup" online should lead to the site [rustup.rs](https://rustup.rs/). The [rustup.rs](https://rustup.rs/) site should be able to detect our used operating system and provide specific instructions.

For example, if on a PC running Windows 64-bit, using the web browser to navigate to [rustup.rs](https://rustup.rs/), the expected result is that a Rust installation instruction for Windows 64-bit is provided automatically.

```txt
C:\temp>rustup-init.exe

Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  C:\Users\myUser\.rustup

This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory is located at:

  C:\Users\myUser\.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  C:\Users\myUser\.cargo\bin

This path will then be added to your PATH environment variable by
modifying the HKEY_CURRENT_USER/Environment/PATH registry key.

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:


   default host triple: x86_64-pc-windows-msvc
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
>1

info: profile set to 'default'
info: default host triple is x86_64-pc-windows-msvc
info: syncing channel updates for 'stable-x86_64-pc-windows-msvc'
info: latest update on 2023-09-19, rust version 1.72.1 (d5c2e9c34 2023-09-13)
info: downloading component 'cargo'
  5.4 MiB /   5.4 MiB (100 %)   1.5 MiB/s in  2s ETA:  0s
info: downloading component 'clippy'
  2.1 MiB /   2.1 MiB (100 %)   1.7 MiB/s in  1s ETA:  0s
info: downloading component 'rust-docs'
 13.7 MiB /  13.7 MiB (100 %)   3.7 MiB/s in  4s ETA:  0s
info: downloading component 'rust-std'
 23.5 MiB /  23.5 MiB (100 %)   4.2 MiB/s in  6s ETA:  0s
info: downloading component 'rustc'
 58.0 MiB /  58.0 MiB (100 %)   5.4 MiB/s in 14s ETA:  0s
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 13.7 MiB /  13.7 MiB (100 %)   1.1 MiB/s in  8s ETA:  0s
info: installing component 'rust-std'
 23.5 MiB /  23.5 MiB (100 %)  10.4 MiB/s in  2s ETA:  0s
info: installing component 'rustc'
 58.0 MiB /  58.0 MiB (100 %)  12.4 MiB/s in  4s ETA:  0s
info: installing component 'rustfmt'
info: default toolchain set to 'stable-x86_64-pc-windows-msvc'

  stable-x86_64-pc-windows-msvc installed - rustc 1.72.1 (d5c2e9c34 2023-09-13)


Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload its PATH environment variable to include
Cargo's bin directory (%USERPROFILE%\.cargo\bin).

Press the Enter key to continue.


C:\temp>
```

### Rustup Rust Update

TODO

### Rustup Rust Uninstall

To uninstall Rust open a shell and run the command `rustup self uninstall` and follow the instructions.

Below is an example of performing a Rust uninstall from the command prompt on a computer running Windows.

```txt
C:\>rustup self uninstall


Thanks for hacking in Rust!

This will uninstall all Rust toolchains and data, and remove
%USERPROFILE%\.cargo/bin from your PATH environment variable.

Continue? (y/N) y

info: removing rustup home
info: removing cargo home
info: removing rustup binaries
info: rustup is uninstalled

C:\>
```

## References

Klabnik S, Nichols C, (2023). *The Rust Programming language, 2nd Edition*, San Francisco: No Starch Press, Inc.

rustup. (n.d.). *rustup is an installer for
the systems programming language Rust*. Retrieved 2023-09-27 from [https://rustup.rs/](https://rustup.rs/)
