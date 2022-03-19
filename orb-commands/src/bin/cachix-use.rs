use xshell::Shell;

fn main() -> Result<(), anyhow::Error> {
    Shell::new()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_cachix_use() {}
}
