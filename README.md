# env_perm

This crate allows you to permanently set environment variables

## Examples
```rust
// Check if DUMMY is set, if not set it to 1
// export DUMMY=1
env_perm::check_or_set("DUMMY", 1).expect("Failed to find or set DUMMY");
// Append $HOME/some/cool/bin to $PATH
// export PATH= "$HOME/some/cool/bin:$PATH"
env_perm::append("PATH", "$HOME/some/cool/bin").expect("Couldn't find PATH");
// Sets a variable without checking if it exists.
// Note you need to use a raw string literal to include ""
// export DUMMY="/something"
env_perm::set("DUMMY", r#""/something""#).expect("Failed to set DUMMY");
```

## Usage
This crate simply appends to your `.profile` and 
if it can't find that it will try `.bash_profile`. 
It will not create any files so you may need to
create one of those files in your home directory
`ie. /Users/me/.profile`.

I have only tested this on macOS but it should
work on any unix system. I want to add windows 
support soon.

Contributions are welcome.