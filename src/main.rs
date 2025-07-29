use std::{error::Error, path::PathBuf};

use alloy::signers::{
    k256::ecdsa::SigningKey,
    local::{LocalSigner, LocalSignerError},
};
use clap::{Args, Parser};
use inquire::Password;

#[derive(Args, Debug)]
struct Keystore {
    /// The name of the wallet.
    name: String,
    /// The password to decrypt the wallet.
    ///
    /// If not specified, the password will be prompted interactively.
    #[arg(long)]
    password: Option<String>,
    /// The base path to find the keystore in.
    #[arg(long, default_value = ".")]
    path: PathBuf,
}

impl Keystore {
    fn password(&self) -> String {
        if let Some(password) = self.password.clone() {
            password
        } else {
            Password::new("Password:")
                .prompt()
                .expect("could not read password")
        }
    }

    fn load(&self) -> Result<LocalSigner<SigningKey>, LocalSignerError> {
        LocalSigner::decrypt_keystore(&self.path, self.password())
    }

    fn save(&self) -> Result<LocalSigner<SigningKey>, LocalSignerError> {
        let mut rng = rand::thread_rng();
        let (signer, _): (LocalSigner<SigningKey>, _) =
            LocalSigner::new_keystore(&self.path, &mut rng, self.password(), Some(&self.name))?;

        Ok(signer)
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
enum Cmd {
    /// Switch to a wallet.
    ///
    /// This is intended to be used in conjunction with `source`:
    ///
    /// ```ignore
    /// # Bash
    /// . <(ki use dev)
    ///
    /// # Fish
    /// ki use dev | .
    /// ```
    Use(Keystore),
    /// Create a new wallet.
    New(Keystore),
}

fn main() -> Result<(), Box<dyn Error>> {
    match Cmd::parse() {
        Cmd::Use(keystore) => {
            let signer = keystore.load()?;

            let shell = std::env::var("SHELL").unwrap();
            if shell.contains("fish") {
                println!("set PRIVATE_KEY {}", signer.to_bytes())
            } else {
                println!("PRIVATE_KEY={}", signer.to_bytes())
            }
        }
        Cmd::New(keystore) => {
            let signer = keystore.save()?;

            println!("Created new keypair.");
            println!("Address:     {}", signer.address());
            println!("Private key: {}", signer.to_bytes());
        }
    }

    Ok(())
}
