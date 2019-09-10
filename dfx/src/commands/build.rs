use crate::config::dfinity::ConfigCanistersCanister;
use crate::lib::env::{BinaryCacheEnv, ProjectConfigEnv};
use crate::lib::error::DfxResult;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::path::Path;

pub fn construct() -> App<'static, 'static> {
    SubCommand::with_name("build")
        .about("Build a canister code, or all canisters if no argument is passed.")
        .arg(Arg::with_name("canister").help("The canister name to build."))
}

fn build_file<T>(env: &T, input_path: &Path, output_path: &Path) -> DfxResult
where
    T: BinaryCacheEnv,
{
    let output_wasm_path = output_path.with_extension("wasm");
    let output_idl_path = output_path.with_extension("did");
    let output_js_path = output_path.with_extension("js");

    env.get_binary_command("asc")?
        .arg(input_path)
        .arg("-o")
        .arg(&output_wasm_path)
        .output()?;
    env.get_binary_command("asc")?
        .arg("--idl")
        .arg(input_path)
        .arg("-o")
        .arg(&output_idl_path)
        .output()?;
    env.get_binary_command("didc")?
        .arg("--js")
        .arg(&output_idl_path)
        .arg("-o")
        .arg(output_js_path)
        .output()?;

    Ok(())
}

pub fn exec<T>(env: &T, _args: &ArgMatches<'_>) -> DfxResult
where
    T: BinaryCacheEnv + ProjectConfigEnv,
{
    // Read the config.
    let config = env.get_config().unwrap();
    // get_path() returns the full path of the config file. We need to get the dirname.
    let project_root = config.get_path().parent().unwrap();

    let build_root = project_root.join(
        config
            .get_config()
            .get_defaults()
            .get_build()
            .get_output("build/"),
    );

    if let Some(canisters) = &config.get_config().canisters {
        for (k, v) in canisters {
            let v: ConfigCanistersCanister = serde_json::from_value(v.to_owned())?;

            println!("Building {}...", k);
            if let Some(x) = v.main {
                let input_as_path = project_root.join(x.as_str());
                let output_path = build_root.join(x.as_str()).with_extension("wasm");
                std::fs::create_dir_all(output_path.parent().unwrap())?;

                build_file(env, &input_as_path, &output_path)?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::env::temp_dir;
    use std::fs;
    use std::io;
    use std::io::{Read, Write};
    use std::path::PathBuf;
    use std::process;

    #[test]
    /// Runs "echo" instead of the compiler to make sure the binaries are called in order
    /// with the good arguments.
    fn build_file_wasm() -> () {
        // Create a binary cache environment that just returns "echo", so we can test the STDOUT.
        struct TestEnv<'a> {
            out_file: &'a fs::File,
        }

        impl<'a> BinaryCacheEnv for TestEnv<'a> {
            fn get_binary_command_path(&self, _binary_name: &str) -> io::Result<PathBuf> {
                // This should not be used.
                panic!("get_binary_command_path should not be called.")
            }
            fn get_binary_command(&self, binary_name: &str) -> io::Result<process::Command> {
                let stdout = self.out_file.try_clone()?;
                let stderr = self.out_file.try_clone()?;

                let mut cmd = process::Command::new("echo");

                cmd.arg(binary_name)
                    .stdout(process::Stdio::from(stdout))
                    .stderr(process::Stdio::from(stderr));

                Ok(cmd)
            }
        }

        let temp_path = temp_dir().join("stdout").with_extension("txt");
        let mut out_file = fs::File::create(temp_path.clone()).expect("Could not create file.");
        let env = TestEnv {
            out_file: &out_file,
        };

        build_file(&env, Path::new("/in/file.as"), Path::new("/out/file.wasm"))
            .expect("Function failed.");

        out_file.flush().expect("Could not flush.");

        let mut s = String::new();
        fs::File::open(temp_path)
            .expect("Could not open temp file.")
            .read_to_string(&mut s)
            .expect("Could not read temp file.");

        assert_eq!(
            s.trim(),
            r#"asc /in/file.as -o /out/file.wasm
                asc --idl /in/file.as -o /out/file.did
                didc --js /out/file.did -o /out/file.js"#
                .replace("                ", "")
        );
    }
}