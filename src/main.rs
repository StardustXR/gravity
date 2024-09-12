use clap::Parser;
use glam::Quat;
use stardust_xr_fusion::{
	client::Client,
	root::{ClientState, RootAspect},
	spatial::{Spatial, Transform},
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
	let mut client = Client::connect()
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

	let client_handle = client.handle();

	let env = client
		.with_event_loop(client_handle.get_root().get_connection_environment())
		.await
		.unwrap()
		.expect("Server could not get the environment needed to connect to stardust");
	for (k, v) in env.into_iter() {
		println!("Setting connection env var {k} to {v}");
		std::env::set_var(k, v);
	}

	let startup_token = client
		.with_event_loop(
			client_handle
				.get_root()
				.generate_state_token(ClientState::from_root(&spatial).unwrap()),
		)
		.await
		.unwrap()
		.expect("Server could not generate startup token");
	std::env::set_var("STARDUST_STARTUP_TOKEN", startup_token);
	let (program, args) = args.command.split_first().unwrap();
	let args: Vec<CString> = args
		.iter()
		.map(|arg| CString::new(arg.clone()).unwrap())
		.collect();
	nix::unistd::execvp(ustr(program).as_cstr(), &args).unwrap();
}
