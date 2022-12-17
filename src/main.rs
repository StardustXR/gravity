use clap::Parser;
use glam::Quat;
use mint::Vector3;
use stardust_xr_fusion::{
	client::Client, core::values::Transform, spatial::Spatial, startup_settings::StartupSettings,
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
		Transform {
			position: Vector3::from([args.x, args.y, args.z]),
			rotation: Quat::from_rotation_y(args.yaw.unwrap_or_default().to_radians()).into(),
			..Default::default()
		},
		false,
	)
	.unwrap();
	let startup_settings =
		StartupSettings::create(&client).expect("Unable to create startup settings");
	startup_settings.set_root(&spatial).unwrap();
	let startup_token = startup_settings
		.generate_startup_token()
		.expect("Unable to get startup token from startup settings")
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
