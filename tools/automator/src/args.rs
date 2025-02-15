#[derive(clap::Parser)]
pub enum Args {
    CreateReleaseAnnouncement(CreateReleaseAnnouncement),
    Sponsors,
}

impl Args {
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }
}

#[derive(clap::Parser)]
pub struct CreateReleaseAnnouncement {}
