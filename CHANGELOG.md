# Changelog
## 🎉 [v0.3.2](https://github.com/tami5/xclog/tree/v0.3.2) - 2023-04-21
### <!-- 0 -->Features
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/4705618cb776973dc835e39cc40a739f72055708">Support SwiftDriver (#3)</a></b> <code>#general</code> <u><b>....</b></u></summary><br />

Adds support for SwiftDriver compile commands. Note, This PR also remove escape chars from file-path of file list to fix error reading the file content.</details></dd></dl>

### <!-- 1 -->Bug Fixes
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/a94cbe52b9bb756fa5f78bb33c399df57b68813f">Treating swift-frontend as error (#4)</a></b> <code>#general</code></summary></details></dd></dl>


## 🎉 [v0.3.0](https://github.com/tami5/xclog/tree/v0.3.0) - 2022-06-25
### <!-- 2 -->Refactor
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/2d7c458b1a12e88218818cb5fb71b7ff42e67201">Use xclogger to generate compile commands</a></b> <code>#general</code></summary></details></dd></dl>


## 🎉 [v0.2.9](https://github.com/tami5/xclog/tree/v0.2.9) - 2022-06-22
### <!-- 0 -->Features
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/c4d1de32084c3c64dd502abaffdae4e36bcb76f6">Auto-release ci on tag</a></b> <code>#general</code></summary></details></dd></dl>


## 🎉 [v0.2.8](https://github.com/tami5/xclog/tree/v0.2.8) - 2022-06-22
### <!-- 0 -->Features
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/43d0f2bf4989f6797994cb8fbd62bf5c284b5a81">Logger from process + BuildSettings::new_sync</a></b> <code>#general</code></summary></details></dd></dl>


## 🎉 [v0.2.6](https://github.com/tami5/xclog/tree/v0.2.6) - 2022-06-16
### <!-- 0 -->Features
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/4095f2f541e0ab295270aa5016b236c0f5ddffba">Compile args get inner</a></b> <code>#compile</code></summary></details></dd></dl>


## 🎉 [v0.2.5](https://github.com/tami5/xclog/tree/v0.2.5) - 2022-06-15
### <!-- 1 -->Bug Fixes
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/8aa140357db704432ec72f77b594cd1546105ba7">Return empty fields</a></b> <code>#build_settings</code></summary></details></dd></dl>

### <!-- 2 -->Refactor
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/61787e81c8bf5240d9dbecd22ce4baa35963cc54">Explicitly ignore other commands</a></b> <code>#parser</code> <u><b>....</b></u></summary><br />

Not identifying other compile commands result in error parser to capture
it 😅</details></dd></dl>


## 🎉 [v0.2.4](https://github.com/tami5/xclog/tree/v0.2.4) - 2022-06-15
### <!-- 0 -->Features
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/b5f048a9488465fa7af88dd893b219de8ed200e1">Return error if compile generate command fails</a></b> <code>#compile</code></summary></details></dd></dl>

### <!-- 2 -->Refactor
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/0ca618e510f061a808ac952582321ba5fe4d70ab">Make XCComplationDatabase a tuple</a></b> <code>#compile</code></summary></details></dd></dl>


## 🎉 [v0.2.3](https://github.com/tami5/xclog/tree/v0.2.3) - 2022-06-15
### <!-- 0 -->Features
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/f818da04f18f977808fbc6002fdb9cd37e65325d">Write to desk</a></b> <code>#compile</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/a8a20f6ef4fa0827a45e512e0866914208787793">Read from JSON file</a></b> <code>#compile</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/2dceb23487dc8717e9d5cb6d09c4f9f88109cf70">Compile flags</a></b> <code>#compile</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/d3a6b7809bddb4bf8517d2f23ccfeaa1fedcff09">Impl into_iter for compile database</a></b> <code>#compile</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/aca389cae500df82993d82fd58d2c16df280554c">Deref to stream</a></b> <code>#logger</code></summary></details></dd></dl>

### <!-- 1 -->Bug Fixes
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/8791597ab3283cef3143c0da965447b0bc29c929">Index-store-path matcher</a></b> <code>#general</code></summary></details></dd></dl>

### <!-- 2 -->Refactor
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/3f7ce5cc9f57deb9c99326c0686452dbee5e7a65">Database keeps track of root</a></b> <code>#compile</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/5b188d989e03d3c298be187d97e2dc773bb08186">Always return XCOutput</a></b> <code>#logger</code></summary></details></dd></dl>

### Ci
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/1dd77e67d7b9576085ab3f56f279d3d1fa59a83e">Skip nix for tests</a></b> <code>#general</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/6d432287fa11d1cd4bae871b39ca482e3b1cad00">Fix permissions</a></b> <code>#general</code></summary></details></dd></dl>


## 🎉 [v0.2.1](https://github.com/tami5/xclog/tree/v0.2.1) - 2022-06-14
### <!-- 0 -->Features
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/8cc12c3456fe8185a475265217ed770848a50f38">Generate compile commands</a></b> <code>#compile</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/e038d3f685a5cd2dc3dfa06585a666c3bead001e">Add tests for XCCompileDatabase output</a></b> <code>#compile</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/21ff5fd7a5d10d028f195727b31878a9a01f46ca">Recognize RegisterExecutionPolicyException</a></b> <code>#parser</code></summary></details></dd></dl>

### <!-- 1 -->Bug Fixes
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/f36c67beeb48f04e6238aa45fcda5bf997f111f4">Clang++ command is treated as an error</a></b> <code>#parser</code></summary></details></dd></dl>

### <!-- 2 -->Refactor
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/a191f598bed2987444b53551d4019029952e848b">Restructure</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/9746d21ee7016d0025f26305fbc19fcd8d0f5c50">Compile command only capture clang/swiftc commands</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/31662fc843686f0ae46253f6cd5851a36f0bd6ab">Ignore shell command in favor of compile command</a></b> <code>#general</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/8e2c8a00742084bd6d73ac80048ea779f5483c34">Move compile generation logic to compile.rs</a></b> <code>#general</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/e4db229ebc3e65018483d8cb9afe2d2cea51c5a2">Export structs instead of functions</a></b> <code>#general</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/46471d01efe35b750af9051a84e9d7778d61f484">Prefix generated code with XC</a></b> <code>#general</code></summary></details></dd></dl>

### Ci
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/3c758728bcf5a58cae845396a0b735256c6e0b98">Add test workflow</a></b> <code>#general</code></summary></details></dd></dl>


## 🎉 [v0.2.0](https://github.com/tami5/xclog/tree/v0.2.0) - 2022-06-13
### <!-- 0 -->Features
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/c5397a75083a196439e2d7d7e46b32e2bde0c5f3">Add helper method for Match</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/e1fe5169da2c349c60314c7c58bd93e5b926e003">Match kind</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/67c403c3fa10751f4b899cfdcbd63e803c89e1f5">Missing captures handling + fix CompileC</a></b> <code>#regex</code> <u><b>....</b></u></summary><br />

still slow, case_c parsed in 3 seconds, while manual 0.1 seconds. I know
I just can't beat manual, but the gap is to wide between the two that
just makes uncomfortable</details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/f16a52165c4fe5ab7a19c0dd0fdb34fbb1fe8adc">Parse from stream</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/191891aae8afc584dfdfc82452f6044e82e13436">Ignore ONLY_ACTIVE_ARC warning</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/33f5d5d2cae692a0d6b2cdba6149553972c57fed">Make regex the main and only parser + rename package</a></b> <code>#general</code></summary></details></dd></dl>

### <!-- 1 -->Bug Fixes
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/3ded7a2ab7675551a47461162257ce08b232012f">No code line preview</a></b> <code>#regex</code></summary></details></dd></dl>

### <!-- 2 -->Refactor
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/16c1ad0b864945c8616b6cb9b1394005fb354c6d">Move define impl to generate.rs</a></b> <code>#regex</code></summary></details></dd></dl>

### <!-- 3 -->Enhancement
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/82c52be92268c789abbe3d009f65a43b47ee5a26">Parse case_a from 5.3 to 0.3</a></b> <code>#regex</code></summary></details></dd></dl>


## 🎉 [v0.1.10](https://github.com/tami5/xclog/tree/v0.1.10) - 2022-06-11
### <!-- 1 -->Bug Fixes
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/4a0f04765b2af0aec982e3b4c5d05712611bb12b">Broken parser</a></b> <code>#manual</code></summary></details></dd></dl>


## 🎉 [v0.1.9](https://github.com/tami5/xclog/tree/v0.1.9) - 2022-06-11
### <!-- 0 -->Features
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/0a1562a5e9ebd1b425816e19e9da8eca1345ed22">Remove dep: nightly</a></b> <code>#general</code></summary></details></dd></dl>

### <!-- 2 -->Refactor
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/d83797ecbaac444b1741e0fe2f646b473ad72878">Move unstable code</a></b> <code>#general</code></summary></details></dd></dl>


## 🎉 [v0.1.7](https://github.com/tami5/xclog/tree/v0.1.7) - 2022-05-29
### <!-- 0 -->Features
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/9eb54d75410445ab4be2e9e226af8946b6cce683">Make define_pattern generate without tests</a></b> <code>#macros</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/eae9c1d1f6905021cab6db111acdbef90e50215a">Parse build failed</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/763687340854f2643db6159f2d0c634113c59fd2">Generate parser module</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/6cf4d5a76c5eeb88cb4b24e9af7f73c2b4631c3f">Define pretty formats</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/396de57c6bb3144feaddb890d7e25fca892468c2">Analyze pattern + tests</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/74ac82eb0d2407a17653009b4e7e72ba20d9c5bf">BUILD TARGET pattern + tests</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/97580958133197748c054e675fe5a774e0ee7e18">BUILD AGGREGATE TARGET pattern + tests</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/6f67fccdb3b928e2d6cc70258fb26f67a8e13f22">ANALYZE TARGET pattern + tests</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/8360ce982d26552e6b4b8481295c86f7209c55e9">Shell command pattern + tests</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/d7c3d14dd0b9f8f6e9a181bfaa78845bb115f0af">Clean.Remove + tests</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/7814a782d74fd32eccd2e2cbce7fb8b8e21c0cf0">CLEAN TARGET</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/b1a0b9b08312963bc9c0a6f5ce7e8a2dbf3a4cf3">Code sign + tests</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/8c20fffb80ada986a9f98ce5fe22ec71906be444">CompileSwift + tests</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/c3bdfe9ca572614df94b28f8eb3661f81526eecf">Compile command and arguments</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/a05af14aa0b3a65f3d4397db3f883619668556d6">Compile xib and storyboard</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/41755eea953824d0ac1744cb96efa5a86244c878">Copy commands + tests</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/0bb45ff4bbc51c573fca6267520c4a69cff61380">Executed tests</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/c23d8dad2c980a86113da5552538e96cd5fc98fd">Kiwi Test failing</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/df3e3caa5691e090871c60d67b03ef3d55bb3d93">UI Test failing</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/288ee413d1594b2579f6a85245620f70b44146cf">Coverage generation</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/6f6991c3cbff4d2adb8513f873ce4a221c98e099">Dsym generation</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/84382892ee9bfc7e2b7f341ed1c692ae9c834574">Linking</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/8789ce56839cf8975f48b83172aedd82162c88e0">Extra test patterns</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/b3dcc3034e25aadaa81ed69514fc2c53cb9952d9">Parallel tests</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/f71ceabb2770799f0711e548a58ea730797db941">PhaseScriptExecution</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/8e8df0d33f9f302513eb4f865b5d67548ab83c34">ProcessPCH</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/9e75c78f8b947b63f0897f77a0224f4e92b73f91">PBXCp</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/890b33acb669f21660e63048a8669506d27ad65f">ProcessInfoPlistFile</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/a1b97cba46627cbc2756836eb7bfb872297a3c9d">More tests patterns</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/b5ee2a4ada81bc13a5ff0bbec1230480eea8be40">Touch</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/c84d41e198bfb958b8a2995a49f9f647285d433f">Warnings</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/ea48d5811a364546ed830d06db4b6559ee1dbe51">Errors</a></b> <code>#regex</code></summary></details></dd></dl>

### <!-- 1 -->Bug Fixes
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/a0363bba8df18e1db07cd3a7fcdcac7120f48291">R#type not found</a></b> <code>#parser</code></summary></details></dd></dl>

### <!-- 2 -->Refactor
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/2bd0c828f1d5e6edcffc6957ddb6aa2bb8afade1">Use camelCase</a></b> <code>#macro</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/afdf031ee6454843117241b8c4918171fc8a2b30">Move regex parser to subfile</a></b> <code>#parser</code> <u><b>....</b></u></summary><br />

Turns out having RegexSet is super slow as case_a tests take 5 seconds
to complete.

Moving it here in case I want to revert back to the old parser</details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/94f43ff16a09c6f994c6c3fc79d9b0911371d5a4">Define_pattern macro</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/5ab649d11482eaf6ee8956d722d82be045e45447">CompileSwift to CompileX</a></b> <code>#regex</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/95831dc055d0cd436cd959f1e3f8450fa097399b">Move old parser to parser/manual</a></b> <code>#general</code></summary></details></dd></dl>

### Testing
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/c626dd0993edf1b4e69517221b3393ef652a3fab">Add tests code</a></b> <code>#general</code></summary></details></dd></dl>

### Wip
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/884598a87b7abd307cdffc80f051d5c91336f1ed">Handling errors</a></b> <code>#general</code> <u><b>....</b></u></summary><br />

Here the ParsableFromStream trait is changed to expect vector of steps
instead of self</details></dd></dl>


## 🎉 [v0.1.6](https://github.com/tami5/xclog/tree/v0.1.6) - 2022-05-26
### <!-- 2 -->Refactor
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/c64f603363c9f9350c5c6f6329e8ef103c0c2b4e">Remove long line for exist type</a></b> <code>#general</code></summary></details></dd></dl>


## 🎉 [v0.1.5](https://github.com/tami5/xclog/tree/v0.1.5) - 2022-05-26
### <!-- 0 -->Features
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/892f2d51a31d9e017af14cb7e5339c30379a538e">Parser and runner</a></b> <code>#build_settings</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/b2a2d06df6e9cdf54f9a79e488cfb2cf98230461">Never panic + refactor</a></b> <code>#build_settings</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/78286cae801a9ebac00cc0fbed7a7c05edbfc876">Get app folder and binary</a></b> <code>#build_settings</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/0d314a75aee90eec403dc6f79d9abbea74fd6ca9">Release workflow</a></b> <code>#dev</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/c8c0fcd65bbe425a0bbbf0827bfafda18b7ff350">Include env vars</a></b> <code>#invocation</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/f25bdedeac54aa35b8e042b345f3bb43bfb161e4">CodeSign</a></b> <code>#parse</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/87ffb5ee9594371ceea8ab8186781ed505fcc04b">CompileAssetCatalog</a></b> <code>#parse</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/81547a0e45beef56f401f663c1f2d9c1e9354b8a">Parse invocated xcodebuild command</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/122b25424b005b3188f36515a2d8a84e3e764301">CompileSwift</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/879b1de335aa713d4ef5a6d0f4f031e19a152f39">CompileSwiftSources</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/358c17a418463f918ec5bd094f46bcf9794953d5">CompileC</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/7cf090fbdecc79896214ee394b48840ba5c834eb">CompileStoryboard</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/cf2963f0fa7fddc9cbb4df6e4ab7d9651de230bb">CompileXIB</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/d06709c29575eaf04cadea528daa7749bff73367">PrecompileSwiftBridgingHeader</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/ed5364d7d3fad6d38793f6863f31a16661045aea">CopySwiftLibs</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/051e13ca7432b36433dce34e6cc02888bd0c9108">Ld</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/05e89e7ed00ae83606acba20f180d57586105071">Validate</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/de51b68aeb42d7dbca798759d9560b71fee4a634">PhaseScriptExecution</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/0ae1ee8dcf9433b2ea596c6a0621b78fac74c8f9">ProcessInfoPlistFile</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/813fac1d1f7eae6430ce5ac02b17199d7143ebbf">ProcessProductPackaging</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/817412927a3886fb9148ee9d8ed25b022e4e58cf">LinkStoryboards</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/5d09437bfdf91ff7e5a94245a7c11bf4a5f895c8">GenerateDSYMFile</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/17ded7614cc09b56d5bba59c91236c9588ba5dc5">CpResource & CreateBuildDirectory</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/d8c179f0353ed45c5b2e362ad4579fbe5f8ee1e9">Register execution policies</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/3eb2a98d7d21ac4f87b5e25c4ca0c077320029fe">Merge swift module</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/1a716be9a7ae149f5208df35ab426955a0b7e598">EmitSwiftModule</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/66baa4099bb5eb0795418619338c806b807736f6">Step enum for xcodebuild outputs</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/d4e159c17798115c16c65892e424ad47b17b2865">String format steps</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/c491128f0151d492455450b9fae9e21c61a7a268">ResolvedSourcePackages</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/f1391806b4f3f497c765ba44200a51df15242233">RegisterWithLaunchServices</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/95f642bb80d59a0031294a5d46cd808093d405db">Account for CompileSwift without target file</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/382422206b2954e1fb612f2076644632055bd7d5">Ignore new build system</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/7a1a5dd2716472b6b62001c0688a70390b35b8e7">Basic warning parsing</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/7a4ee141eaae4ca4a0d2d6f75a54d772bf3481b6">Ignore additional warning</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/998011f1bbe1ee4fd044edfa88b2331dafc10243">Ignore planning note</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/2565115ac02e77e867b2fefe2171cab4fb609f13">Fmt resolved_source_packages with url only</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/74a04235833d5f182051683cc1965e65420199be">Update process product package format</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/df68ad3b7856acaa8f1a649a798c11715ac21b95">Helpers function to steps</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/714ca052b459691b262d8078882b36ed4006b2de">Spawn and spawn once</a></b> <code>#runner</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/77d1866e8fdd76d9ec7b52ccbd8f73fdb6c266e3">Make public</a></b> <code>#runner</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/e1b122af896ebc5f185eda15ef082d051d9bc5a8">Make runner accept generic asref</a></b> <code>#runner</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/674b948c63146f516a8f97d1103edc770b715ca0">Generic stream from tokio command</a></b> <code>#runner</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/8fc4314a3e65b757ff42adefada3f67308ece698">Use tracing_test macro</a></b> <code>#tests</code></summary></details></dd></dl>

### <!-- 1 -->Bug Fixes
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/6e9bcdd3239820d61930b55532c143e0032f3baa">Spawn_once accept asref</a></b> <code>#runner</code></summary></details></dd></dl>

### <!-- 2 -->Refactor
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/b7909ca3bd8d65ebb410f77ba76f5321e2f662e2">Make profile optional</a></b> <code>#code_sign</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/5303cb4f922973cfe3c43a49b4283ac013e6d519">Move to compile steps to steps/</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/f08ba713b2c01e9a3cfec8490e951a22e989132d">Clearer name for consuming non-empty liens</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/109f496773224a115383d240642dc2c19752e376">Rename steps to outputs</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/0c6748748d12bf3c47a26705546f11af41a53768">Description format</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/4d0b54f030bf1e47c5a371603bb7b8638edcfcca">Remove tracing</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/08660701896a2821072d71f0638205c711159681">Format</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/ea5d35a9ac98616bbc14a30c0bbfcce92911389c">Add root to CompileSwiftSources</a></b> <code>#parser</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/97612ccad05068bf082a690d2e3aac67580e16ad">Switch to process-stream</a></b> <code>#runner</code></summary></details></dd></dl>

<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/4cfe9e305ecfa8fddccfc18ecfa4b71e7067d3d0">Make compile_swift command optional</a></b> <code>#general</code></summary></details></dd></dl>

### <!-- 5 --> Documentation
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/3edd017315ce75ba9f69263558f6d26d55bf7a19">Add usage example</a></b> <code>#general</code></summary></details></dd></dl>

### Styling
<dl><dd><details><summary><b><a href="https://github.com/tami5/xclog/commit/40421d5dbca7be654460aa80526bdebab34192ba">Compile_asset_catalog tests style</a></b> <code>#parser</code></summary></details></dd></dl>


