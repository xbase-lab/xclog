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
- [ ] Support: 
  - [ ] Checking
  - [ ] Cloning
  - [ ] CreateUniversalBinary
  - [ ] Fetching
  - [ ] SwiftMergeGeneratedHeaders
