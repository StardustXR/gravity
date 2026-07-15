use clap::Parser;
use glam::Quat;
use stardust_xr_fusion::{
	client::Client,
	spatial::{Spatial, SpatialExt, Transform},
};
use std::ffi::CString;
use ustr::ustr;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
	#[arg(allow_negative_numbers(true))]
	x: f32,
	#[arg(allow_negative_numbers(true))]
	y: f32,
	#[arg(allow_negative_numbers(true))]
	z: f32,
	#[clap(short = 'r')]
	yaw: Option<f32>,
	command: Vec<String>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
	let args = dbg!(Args::parse());
	let (client, root) = Client::auto_connect(&[])
		.await
		.expect("Unable to connect to server");
	let (_, spatial_ref) = Spatial::new(
		&client,
		&root,
		Transform::from_translation_rotation(
			[args.x, args.y, args.z],
			Quat::from_rotation_y(args.yaw.unwrap_or_default().to_radians()),
		),
	)
	.await
	.unwrap();

	let startup_token = client
		.server()
		.generate_startup_token(spatial_ref)
		.await
		.unwrap()
		.expect("Server could not generate startup token");
	std::env::set_var("STARDUST_STARTUP_TOKEN", startup_token);
	let (program, _) = args.command.split_first().unwrap();
	let args: Vec<CString> = args
		.command
		.iter()
		.map(|arg| CString::new(arg.clone()).unwrap())
		.collect();
	nix::unistd::execvp(ustr(program).as_cstr(), &args).unwrap();
}
