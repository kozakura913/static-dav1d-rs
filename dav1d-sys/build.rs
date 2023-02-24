use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

macro_rules! runner {
	($cmd:expr, $($arg:expr),*) => {
		Command::new($cmd)
			$(.arg($arg))*
			.stderr(Stdio::inherit())
			.stdout(Stdio::inherit())
			.output()
			.expect(concat!($cmd, " failed"));

	};
}

fn main() {
	let build_dir = "build";
	let release_dir = "release";
	let manifest_dir=PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
	let source = manifest_dir.join("dav1d");
	let build_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join(build_dir);
	let release_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join(release_dir);
	{
		let mut setup=Command::new("meson");
		setup.arg("setup");
		setup.arg("-Ddefault_library=static");
		let crossfiles=target_crossfiles();
		if crossfiles.exists(){
			println!("target crossfiles {}",crossfiles.to_str().unwrap());
			let crossfiles=format!("--cross-file={}",crossfiles.to_str().unwrap());
			setup.arg(crossfiles);
		}else{
			println!("not exists crossfiles {}",crossfiles.to_str().unwrap());
		}
		setup.arg("--prefix");
		setup.arg(release_path.to_str().unwrap());
		setup.arg(build_path.to_str().unwrap());
		setup.arg(source.to_str().unwrap());
		setup.stderr(Stdio::inherit()).stdout(Stdio::inherit());
		setup.output().expect(concat!("meson", " failed"));
	}
	runner!("ninja", "-C", build_path.to_str().unwrap());
	runner!("meson", "install", "-C", build_path.to_str().unwrap());
	let base_dir=release_path.join("lib");
	let target_sys=format!("{}-{}-{}",env::var("CARGO_CFG_TARGET_ARCH").unwrap(),env::var("CARGO_CFG_TARGET_OS").unwrap(),env::var("CARGO_CFG_TARGET_ENV").unwrap());
	let sys_path=base_dir.join(target_sys);
	if sys_path.exists(){
		println!("cargo:rustc-link-search=native={}",sys_path.to_str().unwrap());
	}else{
		println!("cargo:rustc-link-search=native={}",base_dir.to_str().unwrap());
	}
	println!("cargo:rustc-link-lib=static=dav1d");
}
fn target_crossfiles()->PathBuf{
	let user_def=env::var("DAV1D_CROSS_FILE").ok();
	if let Some(from_env)=user_def{
		return PathBuf::from(from_env);
	}
	let crossfiles=PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR")).join("crossfiles");
	let target_env = env::var("CARGO_CFG_TARGET_ENV");
	let target_arch = env::var("CARGO_CFG_TARGET_ARCH");
	let target_os = env::var("CARGO_CFG_TARGET_OS");
	match target_os.as_ref().map(|x| &**x) {
		Ok("linux") => match target_arch.as_ref().map(|x| &**x) {
			Ok("x86_64")|Ok("x86") =>{
				match target_env.as_ref().map(|x| &**x) {
					Ok("")|Ok("gnu") =>{
						crossfiles.join(format!("{}-{}.meson",target_arch.unwrap(),target_os.unwrap()))
					},
					tenv => panic!("unknown target env {:?}!", tenv)
				}
			},
			arch => panic!("unknown target arch {:?}!", arch)
		},
		Ok("android") =>match target_arch.as_ref().map(|x| &**x) {
			Ok("x86_64")|Ok("x86")|Ok("aarch64")|Ok("arm") =>{
				crossfiles.join(format!("{}-{}.meson",target_arch.unwrap(),target_os.unwrap()))
			},
			arch => panic!("unknown target arch {:?}!", arch)
		},
		Ok("windows") => match target_arch.as_ref().map(|x| &**x) {
			Ok("x86_64")|Ok("x86") => {
				match target_env.as_ref().map(|x| &**x) {
					Ok("gnu") =>{
						crossfiles.join(format!("{}-w64-mingw32.meson",target_arch.unwrap()))
					},
					tenv => panic!("unknown target env {:?}!", tenv)
				}
			},
			arch => panic!("unknown target arch {:?}!", arch)
		},
		tos => panic!("unknown target os {:?}!", tos)
	}
}
