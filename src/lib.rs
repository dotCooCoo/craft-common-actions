pub fn target_arch() -> &'static str {
    std::env::consts::ARCH
}

#[cfg(test)]
mod tests {
    #[test]
    fn reports_the_build_architecture() {
        let arch = super::target_arch();
        println!("freebsd-probe target arch: {arch}, os: {}", std::env::consts::OS);
        assert_eq!(std::env::consts::OS, "freebsd");
        assert!(arch == "x86_64" || arch == "aarch64");
    }
}
