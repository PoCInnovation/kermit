use clap::Command;
use clap_complete::Shell;
use std::path::Path;

pub fn generate_autocomplete(app: &mut Command, shell: Option<Shell>) {
    let bin_name = app.get_name().to_string();
    let shell = shell.map_or_else(
        || {
            let detected_shell = std::env::var("SHELL")
                .ok()
                .and_then(|p| {
                    Path::new(&p)
                        .file_name()
                        .and_then(|os| os.to_str())
                        .map(str::to_lowercase)
                })
                .or_else(|| {
                    if std::env::var_os("PSModulePath").is_some() {
                        Some("pwsh".to_string())
                    } else {
                        None
                    }
                });

            match detected_shell.as_deref() {
                Some("bash") => Shell::Bash,
                Some("zsh") => Shell::Zsh,
                Some("fish") => Shell::Fish,
                Some("elvish") => Shell::Elvish,
                Some("pwsh") | Some("powershell") => Shell::PowerShell,
                _ => Shell::Bash,
            }
        },
        |s| s,
    );

    clap_complete::generate(shell, app, bin_name, &mut std::io::stdout());
}
