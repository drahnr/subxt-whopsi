// SPDX-License-Identifier: GPL-3.0-only
// Copyright 2023 Gensyn Ltd. <admin@gensyn.ai>. All rights reserved.

use fs_err as fs;
use polkadot_runtime::Runtime;
use std::{
	convert::Infallible,
	env::VarError,
	io::Write,
	path::{Path, PathBuf},
};
use subxt_codegen::{CratePath, DerivesRegistry, RuntimeGenerator, TypeSubstitutes};
use subxt_metadata::Metadata;
use syn::parse_quote;

const FILENAME: &str = "codegen.rs";

#[derive(thiserror::Error, Debug)]
pub enum CodegenError {
	#[error(transparent)]
	NoOutDirEnvVar(#[from] VarError),

	#[error(transparent)]
	SubXt(#[from] subxt_codegen::CodegenError),

	#[error(transparent)]
	WriteCodegenFileFailure(#[from] std::io::Error),

	#[error(transparent)]
	Json(#[from] serde_json::Error),

	#[error(transparent)]
	TryFrom(#[from] subxt_metadata::TryFromError),

	#[error(transparent)]
	Never(#[from] Infallible),

	#[error("Failed to decode runtime")]
	RuntimeDecode(#[from] parity_scale_codec::Error),
}

use parity_scale_codec::{Decode, Encode};
fn codegen() -> Result<(), CodegenError> {
	let gensyn_runtime_metadata = Runtime::metadata();
	let gensyn_runtime_metadata_encoded = gensyn_runtime_metadata.encode();
	let metadata = Metadata::decode(&mut gensyn_runtime_metadata_encoded.as_slice())?;
	let generator = RuntimeGenerator::new(metadata);

	let mut derives = DerivesRegistry::with_default_derives(&CratePath::default());
	derives.extend_for_all(vec![parse_quote!(Clone), parse_quote!(PartialEq)], vec![]);
	derives.extend_for_type(
		parse_quote!(gensyn_runtime_primitives::indices::CommitteeIndex),
		vec![parse_quote!(serde::Serialize)],
		vec![],
	);

	let runtime_api = generator.generate_runtime(
		syn::parse_quote!(
			pub mod api {}
		),
		derives,
		TypeSubstitutes::new(),
		CratePath::default(),
		true,
	)?;
	let dir = std::env::var("OUT_DIR")?;

	let path = Path::new(&dir).join(FILENAME);

	let mut f = fs::OpenOptions::new().create(true).truncate(true).write(true).open(&path)?;
	let rust = runtime_api.to_string();
	f.write_all(rust.as_bytes())?;
	drop(f);

	println!("cargo:warning=Wrote {}", path.display());

	let json_path = PathBuf::from(&dir).join(PathBuf::from(FILENAME).with_extension("json"));
	let mut f = fs::OpenOptions::new()
		.create(true)
		.truncate(true)
		.write(true)
		.open(&json_path)?;
	let json = serde_json::to_string(&Runtime::metadata())?;
	f.write_all(json.as_bytes())?;
	drop(f);

	println!("cargo:warning=Wrote {}", json_path.display());

	println!("cargo:rerun-if-changed={}", path.display());

	std::process::Command::new("rustfmt")
		.arg("--edition=2021")
		.arg(FILENAME)
		.current_dir(dir);

	Ok(())
}

fn main() -> Result<(), CodegenError> {
	codegen()?;

	Ok(())
}
