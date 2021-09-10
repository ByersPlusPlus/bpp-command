use async_trait::async_trait;
use bpp_command_api::{CommandError, export_command, structs::Message, traits::{Command, CommandRegistrar, YouTubeSendable}, userservice::user_service_client::UserServiceClient};
use tonic::transport::Channel;

// If you want to define more commands, derive Clone!
#[derive(Clone)]
pub struct {{command_struct}};

#[async_trait]
impl Command<dyn YouTubeSendable> for {{command_struct}} {
    async fn execute(&self, message: Message, sendable: &mut (dyn YouTubeSendable + 'static), user_client: &mut UserServiceClient<Channel>) -> Result<(), CommandError> {
        sendable.send_message("Hello World!").await;
        return Ok(());
    }
}

// Don't touch this macro.
export_command!(register);

// Add your commands here.
// You can register as many as you like.
#[allow(improper_ctypes_definitions)]
extern "C" fn register(registrar: &mut dyn CommandRegistrar) {
    registrar.register_command("{{command_name}}", &[], Box::new({{command_struct}}));
}