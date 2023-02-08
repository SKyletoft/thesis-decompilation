use std::path::PathBuf;

fn main() -> miette::Result<()> {
	// s2e subprojects
	let klee = PathBuf::from("../../s2e/klee/include");
	let coroutine = PathBuf::from("../../s2e/libcoroutine/include");
	let cpu = PathBuf::from("../../s2e/libcpu/include");
	let fsigcxx = PathBuf::from("../../s2e/libfsigc++/include");
	let q = PathBuf::from("../../s2e/libq/include");
	let s2e = PathBuf::from("../../s2e/libs2e/include");
	let s2ecore = PathBuf::from("../../s2e/libs2ecore/include");
	let s2eplugins = PathBuf::from("../../s2e/libs2eplugins/include");
	let tcg = PathBuf::from("../../s2e/libtcg/include");
	let vmi = PathBuf::from("../../s2e/libvmi/include");

	// There are 5 files with the same name in s2e. 3 x86 and 2 x86_64
	// versions. I'm using the first x86_64 version.
	let config_host = PathBuf::from("../../s2e/tools/lib/X8664BitcodeLibrary");

	// Dependencies
	let glibc = PathBuf::from(env!("GLIBC_PATH"));
	let gcc_libs = PathBuf::from(env!("GCCLIBS_PATH"));
	let gcc_libs_l = PathBuf::from(env!("GCCLIBS_PATH_L"));
	let clang_libs = PathBuf::from(env!("CLANGLIBS_PATH"));
	let boost_libs = PathBuf::from(env!("BOOST_PATH"));
	let llvm_libs = PathBuf::from(env!("LLVM_PATH"));

	autocxx_build::Builder::new(
		"src/lib.rs",
		// Breaks on reordering!!
		&[
			&klee,
			&coroutine,
			&cpu,
			&fsigcxx,
			&q,
			&s2e,
			&s2ecore,
			&s2eplugins,
			&tcg,
			&vmi,
			&config_host,
			&gcc_libs,
			&gcc_libs_l,
			&clang_libs,
			&boost_libs,
			&llvm_libs,
			&glibc,
		],
	)
	.extra_clang_args(&[
		"-DBOOST_BIND_GLOBAL_PLACEHOLDERS=1",
		"-DTARGET_PAGE_BITS=12",
		"-DSE_RAM_OBJECT_BITS=12",
		&format!("-DSE_RAM_OBJECT_MASK={}", !11),
	])
	.build()?;

	println!("cargo:rerun-if-changed=src/lib.rs");

	Ok(())
}