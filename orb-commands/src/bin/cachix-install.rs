use xshell::{cmd, Shell};
fn main() -> Result<(), anyhow::Error> {
    let sh = Shell::new()?;

    cachix_install(&sh)?;
    check_cachix(&sh)?;

    Ok(())
}

fn cachix_install(sh: &Shell) -> Result<(), anyhow::Error> {
    // Return if Cachix is already installed
    if cmd!(sh, "cachix --version").run().is_ok() {
        return Ok(());
    }

    cmd!(
        sh,
        "nix-env --quiet -j8 -iA cachix -f https://cachix.org/api/v1/installl"
    )
    .run()?;

    Ok(())
}

fn check_cachix(sh: &Shell) -> Result<(), anyhow::Error> {
    cmd!(sh, "cachix --help").run()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use xshell::Shell;

    fn setup() -> Shell {
        Shell::new().unwrap()
    }

    #[test]
    fn test_cachix_install() {
        let sh = setup();
        assert!(cachix_install(&sh).is_ok());
    }

    #[test]
    fn test_cachix() {
        let sh = setup();
        cachix_install(&sh).unwrap();
        assert!(check_cachix(&sh).is_ok());
    }
}
