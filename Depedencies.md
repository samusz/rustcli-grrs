# Depedencies installation

This can be tricky ! Error messages can fool y into doing unessery configs.

I had an error installing iced crate

```bash
   Compiling servo-fontconfig-sys v5.1.0
error: failed to run custom build command for `servo-fontconfig-sys v5.1.0`

Caused by:
  process didn't exit successfully: `/home/samusz/git-repos/prog/rust/clibook/grrs/target/debug/build/servo-fontconfig-sys-53ed5fc2df954f62/build-script-build` (exit status: 101)
  --- stdout
  cargo:rerun-if-env-changed=FONTCONFIG_NO_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG
  cargo:rerun-if-env-changed=FONTCONFIG_STATIC
  cargo:rerun-if-env-changed=FONTCONFIG_DYNAMIC
  cargo:rerun-if-env-changed=PKG_CONFIG_ALL_STATIC
  cargo:rerun-if-env-changed=PKG_CONFIG_ALL_DYNAMIC
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_SYSROOT_DIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR

  --- stderr
  thread 'main' panicked at '`PKG_CONFIG_ALLOW_SYSTEM_CFLAGS="1" PKG_CONFIG_ALLOW_SYSTEM_LIBS="1" PKG_CONFIG_PATH="/usr/share/fontconfig" "pkg-config" "--libs" "--cflags" "fontconfig" "fontconfig >= 2.11.1"` did not exit successfully: exit status: 1
  error: could not find system library 'fontconfig' required by the 'servo-fontconfig-sys' crate

  --- stderr
  Package fontconfig was not found in the pkg-config search path.
  Perhaps you should add the directory containing `fontconfig.pc'
  to the PKG_CONFIG_PATH environment variable
  No package 'fontconfig' found
  Package fontconfig was not found in the pkg-config search path.
  Perhaps you should add the directory containing `fontconfig.pc'
  to the PKG_CONFIG_PATH environment variable
  No package 'fontconfig' found
  ', /home/samusz/.cargo/registry/src/index.crates.io-6f17d22bba15001f/servo-fontconfig-sys-5.1.0/build.rs:34:17
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

So I added the PKG_CONFIG_PATH in `.profile` and `.bashrc` and it didn't solve the problem.
After seaching in the Rust Discord, I found someone talking about dependencies for Rust.

for WSL/Ubuntu, it went away after installing:

```bash
sudo apt-get install libx11-dev libxext-dev libxft-dev libxinerama-dev libxcursor-dev libxrender-dev libxfixes-dev libpango1.0-dev libgl1-mesa-dev libglu1-mesa-dev
```
