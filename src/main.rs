use git_email_pr::email;

fn main() {
    let config = email::load_config();
    dbg!(&config);
}
