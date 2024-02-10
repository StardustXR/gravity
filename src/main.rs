use clap::Parser;
use glam::Quat;
use stardust_xr_fusion::{
	client::{Client, ClientState},
	spatial::{Spatial, Transform},
};
use std::ffi::CString;
use ustr::ustr;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
	x: f32,
	y: f32,
	z: f32,
	#[clap(short = 'r')]
	yaw: Option<f32>,
	command: Vec<String>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
	let args = dbg!(Args::parse());
	let (client, _event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Unable to connect to server");
	let spatial = Spatial::create(
		client.get_root(),
		Transform::from_translation_rotation(
			[args.x, args.y, args.z],
			Quat::from_rotation_y(args.yaw.unwrap_or_default().to_radians()),
		),
		false,
	)
	.unwrap();

	let env = client
		.get_connection_environment()
		.expect("Unable to get the environment needed to connect to stardust")
		.await
		.expect("Server could not get the environment needed to connect to stardust");
	for (k, v) in env.into_iter() {
		println!("Setting connection env var {k} to {v}");
		std::env::set_var(k, v);
	}

	let startup_token = client
		.state_token(&ClientState {
			data: None,
			root: Some(spatial),
			spatial_anchors: Default::default(),
		})
		.await
		.expect("Server could not generate startup token");
	std::env::set_var("STARDUST_STARTUP_TOKEN", startup_token);
	let (program, args) = args.command.split_first().unwrap();
	let args: Vec<CString> = args
		.into_iter()
		.map(|arg| CString::new(arg.clone()).unwrap())
		.collect();
	nix::unistd::execvp(ustr(&program).as_cstr(), &args).unwrap();
}
