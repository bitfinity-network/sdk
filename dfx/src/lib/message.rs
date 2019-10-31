use std::fmt;

pub enum UserMessage {
    CallCanister,
    DeploymentId,
    SetDeploymentId,
    MethodName,
    WaitForResult,
    ArgumentType,
    ArgumentValue,
    InstallCanister,
    WasmFile,
    ManageCanister,
    QueryCanister,
    RequestCallStatus,
    RequestId,
    BuildCanister,
    CanisterName,
    ConfigureOptions,
    OptionName,
    OptionValue,
    CreateProject,
    ProjectName,
    DryRun,
    StartNode,
    NodeAddress,
    StartBackground,
}

impl UserMessage {
    pub fn to_str(&self) -> &str {
        match &self {

            // dfx canister call
            UserMessage::CallCanister => "Calls a method on a deployed canister.",
            UserMessage::SetDeploymentId => "Specifies the numeric identifier to assign this deployment.",
            UserMessage::DeploymentId => "Specifies the deployment identifier to call.",            
            UserMessage::MethodName => "Specifies the method name to call on the canister.",
            UserMessage::WaitForResult => "Waits for the result of the call to be returned by polling the client.",
            UserMessage::ArgumentType => "Specifies the data type for the argument when making the call using an argument.",
            UserMessage::ArgumentValue => "Specifies the argument to pass to the method.",

            // dfx canister install
            UserMessage::InstallCanister => "Installs compiled code as a canister on the client.",
            UserMessage::WasmFile => "Specifies the path of the .wasm file to install.",

            // dfx canister mod
            UserMessage::ManageCanister => "Manages canisters deployed on a network client.",

            // dfx canister query
            UserMessage::QueryCanister => "Sends a query request to a canister.",

            // dfx canister request_status
            UserMessage::RequestCallStatus => "Requests the status of a specified call from a canister.",
            UserMessage::RequestId => "Specifies the request identifier. The request identifier is an hexadecimal string starting with 0x.",

            // dfx build
            UserMessage::BuildCanister => "Builds all or specific canisters from the code in your project. By default, all canisters are built.",
            UserMessage::CanisterName => "Specifies the canister name to build. If you don't specify this argument, all canisters are built.",

            // dfx config
            UserMessage::ConfigureOptions => "Configures project options for your currently-selected project.",
            UserMessage::OptionName => "Specifies the name of the configuration option to set or read. Use the period delineated path to specify the option to set or read.",
            UserMessage::OptionValue => "Specifies the new value to set. If you don't specify a value, the command displays the current value of the option from the configuration file.",

            // dfx new
            UserMessage::CreateProject => "Creates a new project.",
            UserMessage::ProjectName => "Specifies the name of the project to create.",
            UserMessage::DryRun => "Provides a preview the directories and files to be created without adding them to the file system.",

            // dfx start
            UserMessage::StartNode => "Starts the local network client.",
            UserMessage::NodeAddress => "Specifies the host name and port number to bind the frontend to.",
            UserMessage::StartBackground => "Exits the dfx leaving the client running. Will wait until the client replies before exiting.",
        }
    }
}

impl fmt::Display for UserMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.to_str())
    }
}