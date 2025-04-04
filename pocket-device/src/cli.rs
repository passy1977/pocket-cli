// use clap::{Parser};
// use pocket::models::commands::Commands;
// use pocket::models::user::User;
//
// #[derive(Parser)]
// #[command(author, version, about, long_about = None)]
// pub struct Cli {
//     /// Server passwd
//     #[arg(short, long)]
//     passwd: Option<String>,
//
//     #[command(subcommand)]
//     cmd: Commands,
//
//     /// User email
//     #[arg(long)]
//     user_email: String,
//
//     /// User password
//     #[arg(long)]
//     user_passwd: String,
//
//     /// User name
//     #[arg(long)]
//     user_name: Option<String>,
//
//     /// Command for handle device action
//     #[arg(long)]
//     device_cmd: Option<String>,
//
//     /// User password
//     #[arg(long)]
//     device_uuid: Option<String>,
//
//     /// User name
//     #[arg(long)]
//     device_note: Option<String>,
// }
//
//
// impl Cli {
//     pub fn perform() -> (Option<String>, Option<User>, Option<Device>) {
//
//         let cli = Cli::parse();
//
//         let mut passwd = None;
//         let mut user = User::new();
//         let mut device = Device::new(user.clone());
//
//         if let Some(pwd) = cli.passwd.as_deref() {
//             passwd = Some(pwd.to_string());
//         }
//
//         if let Some(user_cmd) = cli.user_cmd.as_deref() {
//             user.cmd = match user_cmd {
//                 "ADD_USER" => UserCmd::Add,
//                 "MOD_USER" => UserCmd::Mod,
//                 "RM_USER" => UserCmd::Rm,
//                 "RM_GET" => UserCmd::Get,
//                 _ => UserCmd::None
//             }
//         }
//
//
//
//         user.email = cli.user_email.to_string();
//         user.passwd = Some(cli.user_passwd.to_string());
//
//
//         if let Some(user_name) = cli.user_name.as_deref() {
//             user.name = Some(user_name.to_string());
//         }
//
//         if let Some(device_cmd) = cli.device_cmd.as_deref() {
//             device.cmd = match device_cmd {
//                 "ADD_DEVICE" => DeviceCmd::Add,
//                 "MOD_DEVICE" => DeviceCmd::Mod,
//                 "RM_DEVICE" => DeviceCmd::Rm,
//                 "GET_DEVICE" => DeviceCmd::Get,
//                 _ => DeviceCmd::None
//             }
//         }
//
//         if let Some(device_uuid) = cli.device_uuid.as_deref() {
//             device.uuid = device_uuid.to_string();
//         }
//
//         if let Some(device_note) = cli.device_note.as_deref() {
//             device.note = Some(device_note.to_string());
//         }
//
//         match (&user.cmd, &device.cmd) {
//             (UserCmd::Add | UserCmd::Mod | UserCmd::Rm | UserCmd::Get, DeviceCmd::None) => (passwd, Some(user), None),
//             (UserCmd::None, DeviceCmd::Add | DeviceCmd::Mod | DeviceCmd::Rm | DeviceCmd::Get) => (passwd, None, Some(device)),
//             (_, _) => (passwd, None, None)
//         }
//     }
// }