extern crate git2;

use std::env;
use std::path::{PathBuf};
use std::process::{Command};

fn main() {
  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
  let venv_path = PathBuf::from(&out_dir).join("env");
  if !venv_path.exists() {
    Command::new("virtualenv")
      .current_dir(&out_dir)
      .arg("-p").arg("python2.7")
      .arg(venv_path.to_str().unwrap())
      .status().unwrap();
  }

  let python = venv_path.join("bin").join("python");
  let pip = venv_path.join("bin").join("pip");
  let confu = venv_path.join("bin").join("confu");

  // pip install PeachPy
  assert!(Command::new(pip.to_str().unwrap())
      .current_dir(&out_dir)
      .arg("install").arg("--upgrade")
      .arg("git+https://github.com/kali/PeachPy")
      .status().unwrap().success());

  // pip install confu
  assert!(Command::new(pip.to_str().unwrap())
      .current_dir(&out_dir)
      .arg("install").arg("--upgrade")
      .arg("git+https://github.com/Maratyszcza/confu")
      .status().unwrap().success());

  let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
  if target_arch == "i686" || target_arch == "x86-64" {
      // compile cpuinfo
      let cpuinfo_path = out_dir.join("cpuinfo");
      let url = "https://github.com/Maratyszcza/cpuinfo";
      if !cpuinfo_path.exists() {
        git2::Repository::clone(url, &cpuinfo_path).unwrap();
      }

      assert!(Command::new(confu.to_str().unwrap())
          .current_dir(&cpuinfo_path)
          .arg("setup")
          .status().unwrap().success());

      assert!(Command::new(python.to_str().unwrap())
          .current_dir(&cpuinfo_path)
          .arg("./configure.py")
          .status().unwrap().success());

      assert!(Command::new("ninja")
            .env("VIRTUAL_ENV", venv_path.to_str().unwrap())
            .env("PATH", format!("{}:{}",
                venv_path.join("bin").to_str().unwrap(),
                ::std::env::var("PATH").unwrap()
                ))
          .current_dir(&cpuinfo_path)
          .status().unwrap().success());

      println!("cargo:rustc-link-search=native={}", cpuinfo_path.join("lib").to_str().unwrap());
      println!("cargo:rustc-link-lib=static=cpuinfo");
  }

  // compile NNPACK
  let nnpack_path = out_dir.join("NNPACK");

  let url = "https://github.com/Maratyszcza/NNPACK";
  if !nnpack_path.exists() {
    git2::Repository::clone(url, &nnpack_path).unwrap();
  }

  assert!(Command::new(confu.to_str().unwrap())
      .current_dir(&nnpack_path)
      .arg("setup")
      .status().unwrap().success());

  let host = env::var("HOST").unwrap();
  let target = env::var("TARGET").unwrap();
  let cross_args = if host != target {
      vec!["--target", &target_arch, "--toolchain", "/Users/kali/dev/toolchains/raspbian/stretch/armv7-unknown-linux-gnueabihf"]
  } else {
      vec![]
  };

  assert!(Command::new(python.to_str().unwrap())
      .current_dir(&nnpack_path)
      .arg("./configure.py")
      .args(&cross_args)
      .status().unwrap().success());

  assert!(Command::new("ninja")
        .env("VIRTUAL_ENV", venv_path.to_str().unwrap())
        .env("PATH", format!("{}:{}",
            venv_path.join("bin").to_str().unwrap(),
            ::std::env::var("PATH").unwrap()
            ))
      .current_dir(&nnpack_path)
      .status().unwrap().success());

  ::std::fs::copy(
      nnpack_path.join("lib/libnnpack.a"),
       out_dir.join("libnnpack_native.a")).unwrap();

  ::std::fs::copy(
      nnpack_path.join("lib/libpthreadpool.a"),
       out_dir.join("libpthreadpool.a")).unwrap();

  println!("cargo:rustc-link-search=native={}", out_dir.to_str().unwrap());
  println!("cargo:rustc-link-lib=static=nnpack_native");
  println!("cargo:rustc-link-lib=static=pthreadpool");
}
