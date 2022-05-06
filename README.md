# xcodebuild

`xcodebuild` command wrapper, runner and log parser.

- [x] Parse stream from running xcodebuild command.
- [x] ~~impl TryFrom<String>~~.
- [x] Provide runners to consume and process parsed stream of xcodebuild output.
- [ ] Track compilation time.
- [x] Write usage example.
- [ ] Make tests undependent on author filesystem.
- [ ] Create thin wrapper for xcodebuild command.
- [ ] generate steps from vector of lines
- [ ] cli to parse xcodebuild log

## Usage 

```rust 
    use crate::runner::{spawn, spawn_once};
    use futures::StreamExt;

    let root = "path/to/project";
    spawn_once(root, &["clean"]).await.unwrap();
    let mut stream = spawn(root, &["build"]).await.unwrap();

    while let Some(step) = StreamExt::next(&mut stream).await {
        println!("{}", step)
    }

```
