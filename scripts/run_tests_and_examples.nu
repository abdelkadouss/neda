def main [] {
  let crates = (
    open Cargo.toml
    | get workspace.members
  );

  print $"(ansi purple_bold)# Running tests...(ansi reset)";
  cargo test --workspace --no-fail-fast;
  print $"(ansi purple_bold)# Running examples...(ansi reset)";
  for crate in $crates {
    let examples_dir = ([$crate, "examples"] | path join);
    if ($examples_dir | path exists) {
      for file in (ls $examples_dir | get name) {
        let example = ($file | path parse | get stem);
        print $"(ansi purple_bold)# Running the example (ansi u)'($example)' ...(ansi reset)";
        cargo run --example $example --keep-going;
      }
    }
  };
};
