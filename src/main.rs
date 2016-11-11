#![feature(advanced_slice_patterns, slice_patterns)]

extern crate git_together;

use std::env;
use std::process::Command;

use git_together::GitTogether;
use git_together::errors::*;

fn main() {
  run(|| {
    let all_args: Vec<_> = env::args().skip(1).collect();
    let args: Vec<&str> = all_args.iter().map(String::as_ref).collect();

    match args.as_slice() {
      &["with"] => {
        let mut gt = try!(GitTogether::new());

        try!(gt.set_active(&[]));
        let authors = try!(gt.all_authors());
        let mut sorted: Vec<_> = authors.iter().collect();
        sorted.sort_by(|a, b| a.0.cmp(b.0));

        for (initials, author) in sorted {
          println!("{}: {}", initials, author);
        }
      }
      &["with", ref inits..] => {
        let mut gt = try!(GitTogether::new());

        let authors = try!(gt.set_active(inits));
        for author in authors {
          println!("{}", author);
        }
      }
      &[sub_cmd, ref rest..] if ["commit", "merge", "revert"].contains(&sub_cmd) => {
        let mut gt = try!(GitTogether::new());

        if sub_cmd == "merge" {
          env::set_var("GIT_TOGETHER_NO_SIGNOFF", "1");
        }

        let mut cmd = Command::new("git");
        let cmd = cmd.arg(sub_cmd).args(rest);

        let signoff = try!(gt.signoff(cmd));
        let status = try!(signoff.status().chain_err(|| ""));
        if status.success() {
          try!(gt.rotate_active());
        }
      }
      args => {
        try!(Command::new("git").args(args).status().chain_err(|| ""));
      }
    };

    Ok(())
  })
}

fn run<F>(f: F)
  where F: Fn() -> Result<()>
{
  if let Err(e) = f() {
    println!("{}", e);
    std::process::exit(1);
  }
}
