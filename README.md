Rust-based Solidity AST analyzer and context builder.

Note: These goals/priorities will change over time.

### Short-term goals - Working Examples:
* [x] Traverse the AST and create a public "Context" symbol table
* Create a detector registration architecture
* Recreate some basic Slither detectors (Abi encodePacked collision detector)

### Medium-term goals - Auditor Aid:
* [x] Support Multiple Abstract Syntax Trees representing multiple Solidity files
* Support Foundry/Hardhat/Truffle/Solc output formats for ingesting AST
  * [x] Foundry
  * [ ] Hardhat
* Support functionality from:
  * Consensys Solidity Analyzer
    * Complexity
    * nSLOC
    * Percentage YUL code 👀
  * Slither
  * 4nalyzer

### Long-term goals - Product:
Create tools that utilize this context library to:
* Provide automated gas optimizations
* Custom subscribable detectors
* Control/data flow analyses
* Symbolic execution lite, with invariant analysis
* Vyper support

## Attribution
* AST Visitor code from [solc-ast-rs](https://github.com/hrkrshnn/solc-ast-rs).
* Current adapter clones based on [Slither](https://github.com/crytic/slither) detectors.
