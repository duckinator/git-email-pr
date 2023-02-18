# git-email-pr

Combines `git request-pull` and SMTP to send a pull request via email.

**NOTE:** This uses "pull request" in the more abstract meaning, of
requesting someone pull your changes into their git repo, _not_ the
feature as provided by GitHub.

## Installing and configuring

Since Git already has `git send-email`, the configuration for it is reused
where possible. You can use the excellent documentation at
https://git-send-email.io/ for getting that set up!

To install, run `cargo install git-email-pr`.

`git email-pr` uses the following values from `git config`:

- confirm
- from
- smtpserver
- smtpuser
- smtpencryption
- smtpserverport
- smtppass

## Usage

Once you have git-email-pr installed, usage is pretty straightforward.

The workflow this is currently tailored for is:

1. Clone a repository you do not have commit access to
2. Make your changes
3. Push the changes to a repository under your control
4. Send the pull request via email.

Here's an example usecase, if you replace `$YOUR_REPOSITORY` with a link
to the repository you're using.

```
$ git clone https://github.com/duckinator/git-email-pr
$ git checkout -b some-branch
$ # <make and commit some changes>
$ git push $YOUR_REPOSITORY some-branch
$ git config emailpr.to "me@duckie.co"
$ git config emailpr.myrepo $YOUR_REPOSITORY
$ git email-pr -p main
```
