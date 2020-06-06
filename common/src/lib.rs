#![cfg_attr(not(feature = "std"), no_std)]

use git_version::git_version;
const GIT_VERSION: &str = git_version!();

struct CommonConfig {

}

#[cfg(std)]
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
