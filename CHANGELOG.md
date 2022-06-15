# Changelog
All notable changes to this project will be documented in this file.

## [0.2.3] - 2022-06-15

### <!-- 0 -->Features

#### Compile

- <a href="https://github.com/tami5/xcodebuild/commit/f818da04f18f977808fbc6002fdb9cd37e65325d"><tt>f818da0</tt></a> Write to desk
- <a href="https://github.com/tami5/xcodebuild/commit/a8a20f6ef4fa0827a45e512e0866914208787793"><tt>a8a20f6</tt></a> Read from JSON file
- <a href="https://github.com/tami5/xcodebuild/commit/2dceb23487dc8717e9d5cb6d09c4f9f88109cf70"><tt>2dceb23</tt></a> Compile flags
- <a href="https://github.com/tami5/xcodebuild/commit/520b92f718a2260a9929198e88023c4c448bd5cd"><tt>520b92f</tt></a> Impl into_iter for compile database

#### Logger

- <a href="https://github.com/tami5/xcodebuild/commit/aca389cae500df82993d82fd58d2c16df280554c"><tt>aca389c</tt></a> Deref to stream

### <!-- 1 -->Bug Fixes

### <!-- 2 -->Refactor

#### Compile

- <a href="https://github.com/tami5/xcodebuild/commit/3f7ce5cc9f57deb9c99326c0686452dbee5e7a65"><tt>3f7ce5c</tt></a> Database keeps track of root

#### Logger

- <a href="https://github.com/tami5/xcodebuild/commit/5b188d989e03d3c298be187d97e2dc773bb08186"><tt>5b188d9</tt></a> Always return XCOutput

### Miscellaneous Tasks

### Ci

## [0.2.1] - 2022-06-14

### <!-- 0 -->Features

#### Compile

- <a href="https://github.com/tami5/xcodebuild/commit/8cc12c3456fe8185a475265217ed770848a50f38"><tt>8cc12c3</tt></a> Generate compile commands
- <a href="https://github.com/tami5/xcodebuild/commit/e038d3f685a5cd2dc3dfa06585a666c3bead001e"><tt>e038d3f</tt></a> Add tests for XCCompileDatabase output

#### Parser

- <a href="https://github.com/tami5/xcodebuild/commit/21ff5fd7a5d10d028f195727b31878a9a01f46ca"><tt>21ff5fd</tt></a> Recognize RegisterExecutionPolicyException

### <!-- 1 -->Bug Fixes

#### Parser

- <a href="https://github.com/tami5/xcodebuild/commit/f36c67beeb48f04e6238aa45fcda5bf997f111f4"><tt>f36c67b</tt></a> Clang++ command is treated as an error

### <!-- 2 -->Refactor

#### Parser

- <a href="https://github.com/tami5/xcodebuild/commit/a191f598bed2987444b53551d4019029952e848b"><tt>a191f59</tt></a> Restructure

#### Regex

- <a href="https://github.com/tami5/xcodebuild/commit/9746d21ee7016d0025f26305fbc19fcd8d0f5c50"><tt>9746d21</tt></a> Compile command only capture clang/swiftc commands

### Ci

## [0.2.0] - 2022-06-13

### <!-- 0 -->Features

#### Regex

- <a href="https://github.com/tami5/xcodebuild/commit/c5397a75083a196439e2d7d7e46b32e2bde0c5f3"><tt>c5397a7</tt></a> Add helper method for Match
- <a href="https://github.com/tami5/xcodebuild/commit/e1fe5169da2c349c60314c7c58bd93e5b926e003"><tt>e1fe516</tt></a> Match kind
- <a href="https://github.com/tami5/xcodebuild/commit/67c403c3fa10751f4b899cfdcbd63e803c89e1f5"><tt>67c403c</tt></a> Missing captures handling + fix CompileC
- <a href="https://github.com/tami5/xcodebuild/commit/f16a52165c4fe5ab7a19c0dd0fdb34fbb1fe8adc"><tt>f16a521</tt></a> Parse from stream
- <a href="https://github.com/tami5/xcodebuild/commit/191891aae8afc584dfdfc82452f6044e82e13436"><tt>191891a</tt></a> Ignore ONLY_ACTIVE_ARC warning

### <!-- 1 -->Bug Fixes

#### Regex

- <a href="https://github.com/tami5/xcodebuild/commit/3ded7a2ab7675551a47461162257ce08b232012f"><tt>3ded7a2</tt></a> No code line preview

### <!-- 2 -->Refactor

#### Regex

- <a href="https://github.com/tami5/xcodebuild/commit/16c1ad0b864945c8616b6cb9b1394005fb354c6d"><tt>16c1ad0</tt></a> Move define impl to generate.rs

### <!-- 3 -->Enhancement

#### Regex

- <a href="https://github.com/tami5/xcodebuild/commit/82c52be92268c789abbe3d009f65a43b47ee5a26"><tt>82c52be</tt></a> Parse case_a from 5.3 to 0.3

### Miscellaneous Tasks

#### Regex

- <a href="https://github.com/tami5/xcodebuild/commit/b516c6584d8441a3e6b2da972d8cf759b046af3a"><tt>b516c65</tt></a> Define add is_warning

## [0.1.10] - 2022-06-11

### <!-- 1 -->Bug Fixes

#### Manual

- <a href="https://github.com/tami5/xcodebuild/commit/4a0f04765b2af0aec982e3b4c5d05712611bb12b"><tt>4a0f047</tt></a> Broken parser

## [0.1.9] - 2022-06-11

### <!-- 0 -->Features

### <!-- 2 -->Refactor

## [0.1.7] - 2022-05-29

### <!-- 0 -->Features

#### Macros

- <a href="https://github.com/tami5/xcodebuild/commit/9eb54d75410445ab4be2e9e226af8946b6cce683"><tt>9eb54d7</tt></a> Make define_pattern generate without tests

#### Parser

- <a href="https://github.com/tami5/xcodebuild/commit/eae9c1d1f6905021cab6db111acdbef90e50215a"><tt>eae9c1d</tt></a> Parse build failed
- <a href="https://github.com/tami5/xcodebuild/commit/763687340854f2643db6159f2d0c634113c59fd2"><tt>7636873</tt></a> Generate parser module
- <a href="https://github.com/tami5/xcodebuild/commit/6cf4d5a76c5eeb88cb4b24e9af7f73c2b4631c3f"><tt>6cf4d5a</tt></a> Define pretty formats

#### Regex

- <a href="https://github.com/tami5/xcodebuild/commit/396de57c6bb3144feaddb890d7e25fca892468c2"><tt>396de57</tt></a> Analyze pattern + tests
- <a href="https://github.com/tami5/xcodebuild/commit/74ac82eb0d2407a17653009b4e7e72ba20d9c5bf"><tt>74ac82e</tt></a> BUILD TARGET pattern + tests
- <a href="https://github.com/tami5/xcodebuild/commit/97580958133197748c054e675fe5a774e0ee7e18"><tt>9758095</tt></a> BUILD AGGREGATE TARGET pattern + tests
- <a href="https://github.com/tami5/xcodebuild/commit/6f67fccdb3b928e2d6cc70258fb26f67a8e13f22"><tt>6f67fcc</tt></a> ANALYZE TARGET pattern + tests
- <a href="https://github.com/tami5/xcodebuild/commit/8360ce982d26552e6b4b8481295c86f7209c55e9"><tt>8360ce9</tt></a> Shell command pattern + tests
- <a href="https://github.com/tami5/xcodebuild/commit/d7c3d14dd0b9f8f6e9a181bfaa78845bb115f0af"><tt>d7c3d14</tt></a> Clean.Remove + tests
- <a href="https://github.com/tami5/xcodebuild/commit/7814a782d74fd32eccd2e2cbce7fb8b8e21c0cf0"><tt>7814a78</tt></a> CLEAN TARGET
- <a href="https://github.com/tami5/xcodebuild/commit/b1a0b9b08312963bc9c0a6f5ce7e8a2dbf3a4cf3"><tt>b1a0b9b</tt></a> Code sign + tests
- <a href="https://github.com/tami5/xcodebuild/commit/8c20fffb80ada986a9f98ce5fe22ec71906be444"><tt>8c20fff</tt></a> CompileSwift + tests
- <a href="https://github.com/tami5/xcodebuild/commit/c3bdfe9ca572614df94b28f8eb3661f81526eecf"><tt>c3bdfe9</tt></a> Compile command and arguments
- <a href="https://github.com/tami5/xcodebuild/commit/a05af14aa0b3a65f3d4397db3f883619668556d6"><tt>a05af14</tt></a> Compile xib and storyboard
- <a href="https://github.com/tami5/xcodebuild/commit/41755eea953824d0ac1744cb96efa5a86244c878"><tt>41755ee</tt></a> Copy commands + tests
- <a href="https://github.com/tami5/xcodebuild/commit/0bb45ff4bbc51c573fca6267520c4a69cff61380"><tt>0bb45ff</tt></a> Executed tests
- <a href="https://github.com/tami5/xcodebuild/commit/c23d8dad2c980a86113da5552538e96cd5fc98fd"><tt>c23d8da</tt></a> Kiwi Test failing
- <a href="https://github.com/tami5/xcodebuild/commit/df3e3caa5691e090871c60d67b03ef3d55bb3d93"><tt>df3e3ca</tt></a> UI Test failing
- <a href="https://github.com/tami5/xcodebuild/commit/288ee413d1594b2579f6a85245620f70b44146cf"><tt>288ee41</tt></a> Coverage generation
- <a href="https://github.com/tami5/xcodebuild/commit/6f6991c3cbff4d2adb8513f873ce4a221c98e099"><tt>6f6991c</tt></a> Dsym generation
- <a href="https://github.com/tami5/xcodebuild/commit/84382892ee9bfc7e2b7f341ed1c692ae9c834574"><tt>8438289</tt></a> Linking
- <a href="https://github.com/tami5/xcodebuild/commit/8789ce56839cf8975f48b83172aedd82162c88e0"><tt>8789ce5</tt></a> Extra test patterns
- <a href="https://github.com/tami5/xcodebuild/commit/b3dcc3034e25aadaa81ed69514fc2c53cb9952d9"><tt>b3dcc30</tt></a> Parallel tests
- <a href="https://github.com/tami5/xcodebuild/commit/f71ceabb2770799f0711e548a58ea730797db941"><tt>f71ceab</tt></a> PhaseScriptExecution
- <a href="https://github.com/tami5/xcodebuild/commit/8e8df0d33f9f302513eb4f865b5d67548ab83c34"><tt>8e8df0d</tt></a> ProcessPCH
- <a href="https://github.com/tami5/xcodebuild/commit/9e75c78f8b947b63f0897f77a0224f4e92b73f91"><tt>9e75c78</tt></a> PBXCp
- <a href="https://github.com/tami5/xcodebuild/commit/890b33acb669f21660e63048a8669506d27ad65f"><tt>890b33a</tt></a> ProcessInfoPlistFile
- <a href="https://github.com/tami5/xcodebuild/commit/a1b97cba46627cbc2756836eb7bfb872297a3c9d"><tt>a1b97cb</tt></a> More tests patterns
- <a href="https://github.com/tami5/xcodebuild/commit/b5ee2a4ada81bc13a5ff0bbec1230480eea8be40"><tt>b5ee2a4</tt></a> Touch
- <a href="https://github.com/tami5/xcodebuild/commit/c84d41e198bfb958b8a2995a49f9f647285d433f"><tt>c84d41e</tt></a> Warnings
- <a href="https://github.com/tami5/xcodebuild/commit/ea48d5811a364546ed830d06db4b6559ee1dbe51"><tt>ea48d58</tt></a> Errors

### <!-- 1 -->Bug Fixes

#### Parser

- <a href="https://github.com/tami5/xcodebuild/commit/a0363bba8df18e1db07cd3a7fcdcac7120f48291"><tt>a0363bb</tt></a> R#type not found

### <!-- 2 -->Refactor

#### Macro

- <a href="https://github.com/tami5/xcodebuild/commit/2bd0c828f1d5e6edcffc6957ddb6aa2bb8afade1"><tt>2bd0c82</tt></a> Use camelCase

#### Parser

- <a href="https://github.com/tami5/xcodebuild/commit/afdf031ee6454843117241b8c4918171fc8a2b30"><tt>afdf031</tt></a> Move regex parser to subfile

#### Regex

- <a href="https://github.com/tami5/xcodebuild/commit/94f43ff16a09c6f994c6c3fc79d9b0911371d5a4"><tt>94f43ff</tt></a> Define_pattern macro
- <a href="https://github.com/tami5/xcodebuild/commit/5ab649d11482eaf6ee8956d722d82be045e45447"><tt>5ab649d</tt></a> CompileSwift to CompileX

### Miscellaneous Tasks

#### Parser

- <a href="https://github.com/tami5/xcodebuild/commit/dee01a8c2eca0f33957f1bae0c57c31414fb149a"><tt>dee01a8</tt></a> Format define macro lines

### Testing

### Wip

## [0.1.6] - 2022-05-26

### <!-- 2 -->Refactor

### Miscellaneous Tasks

## [0.1.5] - 2022-05-26

### <!-- 0 -->Features

#### Parser

- <a href="https://github.com/tami5/xcodebuild/commit/7a4ee141eaae4ca4a0d2d6f75a54d772bf3481b6"><tt>7a4ee14</tt></a> Ignore additional warning
- <a href="https://github.com/tami5/xcodebuild/commit/998011f1bbe1ee4fd044edfa88b2331dafc10243"><tt>998011f</tt></a> Ignore planning note
- <a href="https://github.com/tami5/xcodebuild/commit/2565115ac02e77e867b2fefe2171cab4fb609f13"><tt>2565115</tt></a> Fmt resolved_source_packages with url only
- <a href="https://github.com/tami5/xcodebuild/commit/74a04235833d5f182051683cc1965e65420199be"><tt>74a0423</tt></a> Update process product package format
- <a href="https://github.com/tami5/xcodebuild/commit/df68ad3b7856acaa8f1a649a798c11715ac21b95"><tt>df68ad3</tt></a> Helpers function to steps

### <!-- 2 -->Refactor

#### Runner

- <a href="https://github.com/tami5/xcodebuild/commit/97612ccad05068bf082a690d2e3aac67580e16ad"><tt>97612cc</tt></a> Switch to process-stream

### Miscellaneous Tasks

## [0.1.4] - 2022-05-18

### <!-- 0 -->Features

#### Build_settings

- <a href="https://github.com/tami5/xcodebuild/commit/b2a2d06df6e9cdf54f9a79e488cfb2cf98230461"><tt>b2a2d06</tt></a> Never panic + refactor
- <a href="https://github.com/tami5/xcodebuild/commit/78286cae801a9ebac00cc0fbed7a7c05edbfc876"><tt>78286ca</tt></a> Get app folder and binary

#### Runner

- <a href="https://github.com/tami5/xcodebuild/commit/674b948c63146f516a8f97d1103edc770b715ca0"><tt>674b948</tt></a> Generic stream from tokio command

### Miscellaneous Tasks

## [0.1.3] - 2022-05-17

### <!-- 0 -->Features

#### Build_settings

- <a href="https://github.com/tami5/xcodebuild/commit/892f2d51a31d9e017af14cb7e5339c30379a538e"><tt>892f2d5</tt></a> Parser and runner

#### Invocation

- <a href="https://github.com/tami5/xcodebuild/commit/c8c0fcd65bbe425a0bbbf0827bfafda18b7ff350"><tt>c8c0fcd</tt></a> Include env vars

### <!-- 2 -->Refactor

#### Code_sign

- <a href="https://github.com/tami5/xcodebuild/commit/b7909ca3bd8d65ebb410f77ba76f5321e2f662e2"><tt>b7909ca</tt></a> Make profile optional

## [0.1.2] - 2022-05-06

### <!-- 0 -->Features

#### Parser

- <a href="https://github.com/tami5/xcodebuild/commit/c491128f0151d492455450b9fae9e21c61a7a268"><tt>c491128</tt></a> ResolvedSourcePackages
- <a href="https://github.com/tami5/xcodebuild/commit/f1391806b4f3f497c765ba44200a51df15242233"><tt>f139180</tt></a> RegisterWithLaunchServices
- <a href="https://github.com/tami5/xcodebuild/commit/95f642bb80d59a0031294a5d46cd808093d405db"><tt>95f642b</tt></a> Account for CompileSwift without target file
- <a href="https://github.com/tami5/xcodebuild/commit/382422206b2954e1fb612f2076644632055bd7d5"><tt>3824222</tt></a> Ignore new build system
- <a href="https://github.com/tami5/xcodebuild/commit/7a1a5dd2716472b6b62001c0688a70390b35b8e7"><tt>7a1a5dd</tt></a> Basic warning parsing

#### Runner

- <a href="https://github.com/tami5/xcodebuild/commit/e1b122af896ebc5f185eda15ef082d051d9bc5a8"><tt>e1b122a</tt></a> Make runner accept generic asref

### <!-- 1 -->Bug Fixes

#### Runner

- <a href="https://github.com/tami5/xcodebuild/commit/6e9bcdd3239820d61930b55532c143e0032f3baa"><tt>6e9bcdd</tt></a> Spawn_once accept asref

### <!-- 2 -->Refactor

#### Parser

- <a href="https://github.com/tami5/xcodebuild/commit/0c6748748d12bf3c47a26705546f11af41a53768"><tt>0c67487</tt></a> Description format
- <a href="https://github.com/tami5/xcodebuild/commit/4d0b54f030bf1e47c5a371603bb7b8638edcfcca"><tt>4d0b54f</tt></a> Remove tracing
- <a href="https://github.com/tami5/xcodebuild/commit/08660701896a2821072d71f0638205c711159681"><tt>0866070</tt></a> Format
- <a href="https://github.com/tami5/xcodebuild/commit/ea5d35a9ac98616bbc14a30c0bbfcce92911389c"><tt>ea5d35a</tt></a> Add root to CompileSwiftSources

## [0.1.1] - 2022-05-06

### <!-- 0 -->Features

#### Runner

- <a href="https://github.com/tami5/xcodebuild/commit/77d1866e8fdd76d9ec7b52ccbd8f73fdb6c266e3"><tt>77d1866</tt></a> Make public

### Miscellaneous Tasks

## [0.1.0] - 2022-05-06

### <!-- 0 -->Features

#### Dev

- <a href="https://github.com/tami5/xcodebuild/commit/0d314a75aee90eec403dc6f79d9abbea74fd6ca9"><tt>0d314a7</tt></a> Release workflow

#### Parse

- <a href="https://github.com/tami5/xcodebuild/commit/f25bdedeac54aa35b8e042b345f3bb43bfb161e4"><tt>f25bded</tt></a> CodeSign
- <a href="https://github.com/tami5/xcodebuild/commit/87ffb5ee9594371ceea8ab8186781ed505fcc04b"><tt>87ffb5e</tt></a> CompileAssetCatalog

#### Parser

- <a href="https://github.com/tami5/xcodebuild/commit/81547a0e45beef56f401f663c1f2d9c1e9354b8a"><tt>81547a0</tt></a> Parse invocated xcodebuild command
- <a href="https://github.com/tami5/xcodebuild/commit/122b25424b005b3188f36515a2d8a84e3e764301"><tt>122b254</tt></a> CompileSwift
- <a href="https://github.com/tami5/xcodebuild/commit/879b1de335aa713d4ef5a6d0f4f031e19a152f39"><tt>879b1de</tt></a> CompileSwiftSources
- <a href="https://github.com/tami5/xcodebuild/commit/358c17a418463f918ec5bd094f46bcf9794953d5"><tt>358c17a</tt></a> CompileC
- <a href="https://github.com/tami5/xcodebuild/commit/7cf090fbdecc79896214ee394b48840ba5c834eb"><tt>7cf090f</tt></a> CompileStoryboard
- <a href="https://github.com/tami5/xcodebuild/commit/cf2963f0fa7fddc9cbb4df6e4ab7d9651de230bb"><tt>cf2963f</tt></a> CompileXIB
- <a href="https://github.com/tami5/xcodebuild/commit/d06709c29575eaf04cadea528daa7749bff73367"><tt>d06709c</tt></a> PrecompileSwiftBridgingHeader
- <a href="https://github.com/tami5/xcodebuild/commit/ed5364d7d3fad6d38793f6863f31a16661045aea"><tt>ed5364d</tt></a> CopySwiftLibs
- <a href="https://github.com/tami5/xcodebuild/commit/051e13ca7432b36433dce34e6cc02888bd0c9108"><tt>051e13c</tt></a> Ld
- <a href="https://github.com/tami5/xcodebuild/commit/05e89e7ed00ae83606acba20f180d57586105071"><tt>05e89e7</tt></a> Validate
- <a href="https://github.com/tami5/xcodebuild/commit/de51b68aeb42d7dbca798759d9560b71fee4a634"><tt>de51b68</tt></a> PhaseScriptExecution
- <a href="https://github.com/tami5/xcodebuild/commit/0ae1ee8dcf9433b2ea596c6a0621b78fac74c8f9"><tt>0ae1ee8</tt></a> ProcessInfoPlistFile
- <a href="https://github.com/tami5/xcodebuild/commit/813fac1d1f7eae6430ce5ac02b17199d7143ebbf"><tt>813fac1</tt></a> ProcessProductPackaging
- <a href="https://github.com/tami5/xcodebuild/commit/817412927a3886fb9148ee9d8ed25b022e4e58cf"><tt>8174129</tt></a> LinkStoryboards
- <a href="https://github.com/tami5/xcodebuild/commit/5d09437bfdf91ff7e5a94245a7c11bf4a5f895c8"><tt>5d09437</tt></a> GenerateDSYMFile
- <a href="https://github.com/tami5/xcodebuild/commit/17ded7614cc09b56d5bba59c91236c9588ba5dc5"><tt>17ded76</tt></a> CpResource & CreateBuildDirectory
- <a href="https://github.com/tami5/xcodebuild/commit/d8c179f0353ed45c5b2e362ad4579fbe5f8ee1e9"><tt>d8c179f</tt></a> Register execution policies
- <a href="https://github.com/tami5/xcodebuild/commit/3eb2a98d7d21ac4f87b5e25c4ca0c077320029fe"><tt>3eb2a98</tt></a> Merge swift module
- <a href="https://github.com/tami5/xcodebuild/commit/1a716be9a7ae149f5208df35ab426955a0b7e598"><tt>1a716be</tt></a> EmitSwiftModule
- <a href="https://github.com/tami5/xcodebuild/commit/66baa4099bb5eb0795418619338c806b807736f6"><tt>66baa40</tt></a> Step enum for xcodebuild outputs
- <a href="https://github.com/tami5/xcodebuild/commit/d4e159c17798115c16c65892e424ad47b17b2865"><tt>d4e159c</tt></a> String format steps

#### Runner

- <a href="https://github.com/tami5/xcodebuild/commit/714ca052b459691b262d8078882b36ed4006b2de"><tt>714ca05</tt></a> Spawn and spawn once

#### Tests

- <a href="https://github.com/tami5/xcodebuild/commit/8fc4314a3e65b757ff42adefada3f67308ece698"><tt>8fc4314</tt></a> Use tracing_test macro

### <!-- 2 -->Refactor

#### Parser

- <a href="https://github.com/tami5/xcodebuild/commit/5303cb4f922973cfe3c43a49b4283ac013e6d519"><tt>5303cb4</tt></a> Move to compile steps to steps/
- <a href="https://github.com/tami5/xcodebuild/commit/f08ba713b2c01e9a3cfec8490e951a22e989132d"><tt>f08ba71</tt></a> Clearer name for consuming non-empty liens
- <a href="https://github.com/tami5/xcodebuild/commit/109f496773224a115383d240642dc2c19752e376"><tt>109f496</tt></a> Rename steps to outputs

### Documentation

### Miscellaneous Tasks

#### Parser

- <a href="https://github.com/tami5/xcodebuild/commit/ff92966084b912b79cbc70798bacc4573c98901d"><tt>ff92966</tt></a> Try_to_stream_test

#### Tests

- <a href="https://github.com/tami5/xcodebuild/commit/b9e9304c763ad9e6d49d1d68efe9efbf2f92f950"><tt>b9e9304</tt></a> Make tracing_test optional

### Styling

#### Parser

- <a href="https://github.com/tami5/xcodebuild/commit/40421d5dbca7be654460aa80526bdebab34192ba"><tt>40421d5</tt></a> Compile_asset_catalog tests style

