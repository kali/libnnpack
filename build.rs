extern crate git2;

use std::env;
use std::fs::{create_dir_all};
use std::path::{PathBuf};
use std::process::{Command};

fn main() {
  let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
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

  /*
  let mut ninja = PathBuf::from(&out_dir);
  ninja.push("ninja-build");
  ninja.push("ninja");
  if !ninja.exists() {
    let mut ninja_src_path = PathBuf::from(&manifest_dir);
    ninja_src_path.push("ninja");
    let mut ninja_build_path = PathBuf::from(&out_dir);
    ninja_build_path.push("ninja-build");
    create_dir_all(&ninja_build_path).ok();
    let mut ninja_config_path = PathBuf::from(&ninja_src_path);
    ninja_config_path.push("configure.py");
    assert!(Command::new(python.to_str().unwrap())
      .current_dir(ninja_build_path.to_str().unwrap())
      .arg(ninja_config_path.to_str().unwrap())
      .arg("--bootstrap")
      .status().unwrap().success());
    assert!(Command::new(pip.to_str().unwrap())
      .current_dir(&out_dir)
      .arg("install").arg("--upgrade")
      .arg("ninja-syntax")
      .status().unwrap().success());
  }
  */

  assert!(Command::new(pip.to_str().unwrap())
      .current_dir(&out_dir)
      .arg("install").arg("--upgrade")
      .arg("git+https://github.com/Maratyszcza/PeachPy")
      .status().unwrap().success());

  assert!(Command::new(pip.to_str().unwrap())
      .current_dir(&out_dir)
      .arg("install").arg("--upgrade")
      .arg("git+https://github.com/Maratyszcza/confu")
      .status().unwrap().success());


  let nnpack_path = out_dir.join("NNPACK");

  let url = "https://github.com/Maratyszcza/NNPACK";
  if !nnpack_path.exists() {
    git2::Repository::clone(url, &nnpack_path).unwrap();
  }

  assert!(Command::new(confu.to_str().unwrap())
      .current_dir(&nnpack_path)
      .arg("setup")
      .status().unwrap().success());
println!("1");
  assert!(Command::new(python.to_str().unwrap())
      .current_dir(&nnpack_path)
      .arg("./configure.py")
      .status().unwrap().success());
println!("2");

  assert!(Command::new("ninja")
        .env("PYTHONHOME", venv_path.to_str().unwrap())
      .current_dir(&nnpack_path)
      .status().unwrap().success());
println!("3");

  ::std::fs::copy(
      nnpack_path.join("lib/libnnpack.a"),
       out_dir.join("libnnpack_native.a")).unwrap();
/*
  let mut peachpy_build_path = PathBuf::from(&out_dir);
  peachpy_build_path.push("PeachPy-build");
  if !peachpy_build_path.exists() {
    let mut peachpy_src_path = PathBuf::from(&manifest_dir);
    peachpy_src_path.push("PeachPy");
    assert!(Command::new("cp")
      .current_dir(&out_dir)
      .arg("-r")
      .arg(peachpy_src_path.to_str().unwrap())
      .arg(peachpy_build_path.to_str().unwrap())
      .status().unwrap().success());
    let mut peachpy_req_path = PathBuf::from(&peachpy_build_path);
    peachpy_req_path.push("requirements.txt");
    assert!(Command::new(pip.to_str().unwrap())
      .current_dir(&out_dir)
      .arg("install").arg("--upgrade")
      .arg("-r").arg(peachpy_req_path.to_str().unwrap())
      .status().unwrap().success());
    let mut peachpy_setup_path = PathBuf::from(&peachpy_build_path);
    peachpy_setup_path.push("setup.py");
    assert!(Command::new(python.to_str().unwrap())
      .current_dir(&peachpy_build_path)
      .arg(peachpy_setup_path.to_str().unwrap())
      .arg("generate")
      .status().unwrap().success());
    assert!(Command::new(pip.to_str().unwrap())
      .current_dir(peachpy_build_path.to_str().unwrap())
      .arg("install").arg("--upgrade")
      .arg(peachpy_build_path.to_str().unwrap())
      .status().unwrap().success());
  }

  let mut nnpack_lib_dst_path = PathBuf::from(&out_dir);
  nnpack_lib_dst_path.push("libnnpack_native.a");
  if !nnpack_lib_dst_path.exists() {
    let mut nnpack_src_path = PathBuf::from(&manifest_dir);
    nnpack_src_path.push("NNPACK");
    let mut nnpack_build_path = PathBuf::from(&out_dir);
    nnpack_build_path.push("NNPACK-build");
    assert!(Command::new("cp")
      .current_dir(&out_dir)
      .arg("-r")
      .arg(nnpack_src_path.to_str().unwrap())
      .arg(nnpack_build_path.to_str().unwrap())
      .status().unwrap().success());
    let mut nnpack_config_path = PathBuf::from(&nnpack_build_path);
    nnpack_config_path.push("configure.py");
    assert!(Command::new(python.to_str().unwrap())
      .current_dir(nnpack_build_path.to_str().unwrap())
      .arg(nnpack_config_path.to_str().unwrap())
      .status().unwrap().success());
    assert!(Command::new(ninja.to_str().unwrap())
      .env("PYTHONHOME", venv_path.to_str().unwrap())
      .current_dir(nnpack_build_path.to_str().unwrap())
      .status().unwrap().success());
    let mut nnpack_lib_path = PathBuf::from(&nnpack_build_path);
    nnpack_lib_path.push("lib");
    nnpack_lib_path.push("libnnpack.a");
    assert!(Command::new("cp")
      .current_dir(&out_dir)
      .arg(nnpack_lib_path.to_str().unwrap())
      .arg(nnpack_lib_dst_path.to_str().unwrap())
      .status().unwrap().success());
  }
  */

  println!("cargo:rustc-link-search=native={}", out_dir.to_str().unwrap());
  println!("cargo:rustc-link-lib=static=nnpack_native")
}
