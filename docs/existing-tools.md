# A Summary of the Polkadot AI Tooling Landscape

This document summarises the existing landscape of AI-powered developer tools within the Polkadot ecosystem.

## Introduction

The Polkadot ecosystem has several existing AI-powered tools and related projects. These initiatives range from public-facing chatbots to open-source developer tools and proofs of concept. This summary captures these known projects to provide an overview of the current landscape.

## Existing AI Tools & Projects

This section covers live tools and the open-source codebases behind them.

| Tool / Project                                                                                            | Description                                                                                                           | Key Notes & Status                                                                                                                                                                     |
|:----------------------------------------------------------------------------------------------------------|:----------------------------------------------------------------------------------------------------------------------|:---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| [**Polkadot Docs**](https://docs.polkadot.com/) & [**Wiki**](https://wiki.polkadot.network/) AI Assistant | An AI assistant embedded across the official Polkadot documentation sites to answer questions based on their content. | Powered by [Kapa AI](https://www.kapa.ai/). It is effective for documentation-based queries but lacks direct knowledge of the SDK's source code.                                       |
| [**Polkabot.ai**](https://polkabot.ai/)                                                                   | A general-purpose chatbot for the Polkadot ecosystem.                                                                 | It was noted that some of its data (e.g., DOT price) can be outdated. The project's source code is available for evaluation (see [GitLab Repo](https://gitlab.com/Polkabot/polkabot)). |
| [**dAppForge**](https://www.dappforge.app/)                                                               | An AI copilot tool focussed on assisting with dApp development on Polkadot.                                           | Shared as an example of existing work in the ecosystem. Appears to be a developer-focussed tool rather than a general-purpose chatbot.                                                 |

## Related Projects & Strategic Context

These are not standalone tools but represent important context, learnings, or related efforts within the ecosystem.

### W3F Grants Programme: [mcp-polkadot](https://grants.web3.foundation/applications/mcp-polkadot)

The `mcp-polkadot` project, funded through the Web3 Foundation grants programme, indicates wider ecosystem interest in AI tooling and represents another team to potentially engage with for shared learnings.

### Parity Forum Post: ["I built a Polkadot ChatGPT bot"](https://forum.parity.io/t/i-built-a-polkadot-chatgpt-bot-here-are-the-things-i-learned/1798)

A crucial first-hand account from a developer who built a similar tool. The post highlights key challenges, particularly around data accuracy and the difficulty of keeping the AI's knowledge base synchronised with the rapidly evolving Polkadot codebase. This serves as a valuable collection of lessons learned.
