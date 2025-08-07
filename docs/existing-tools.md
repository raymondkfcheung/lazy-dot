# A Summary of the Polkadot AI Tooling Landscape

*This document is based on a series of discussions regarding the development of a new AI-powered developer tool.*

## Introduction

The Polkadot ecosystem has several existing AI-powered tools and related projects. These initiatives range from public-facing chatbots to open-source developer tools and proofs of concept. The consensus from our discussions is that while these tools exist, there is a clear need for a unified, trustworthy, and deeply code-aware solution to better serve developers. This summary captures the known projects to provide context for future work.

## 1. Public-Facing Chatbots

These are live tools that users can interact with directly.

| Tool | Description | Key Notes & Status |
| :--- | :--- | :--- |
| [**Polkadot.com**](https://polkadot.com/) | The main Polkadot website, featuring an AI assistant for general user queries. | This appears to be the primary public-facing AI chat tool for the ecosystem, likely focussed on new users and general questions. |
| [**Polkadot Wiki**](https://wiki.polkadot.network/) | An AI assistant embedded in the Polkadot Wiki and Docs to answer questions based on the documentation. | Powered by a third-party service (Kapa AI). It's effective for documentation-based queries but lacks direct knowledge of the SDK's source code. A key proposal is to enhance this tool by adding the SDK repo as a data source. |
| [**Polkabot.ai**](https://polkabot.ai/) | A general-purpose chatbot for the Polkadot ecosystem. | It was noted that some of its real-time data (e.g., DOT price) appeared outdated during our review. Its source code seems to be available on GitLab (see below). |

## 2. Developer Tools & Open-Source Projects

These are code repositories and tools aimed at developers.

| Tool / Project | Description | Key Notes & Status |
| :--- | :--- | :--- |
| [**dAppForge**](https://www.dappforge.app/) | An AI copilot tool focussed on assisting with dApp development on Polkadot. | Shared as an example of existing work in the ecosystem. Appears to be a developer-focussed tool rather than a general-purpose chatbot. |
| [**Polkabot (Source Code)**](https://gitlab.com/Polkabot/polkabot) | The source code for the `Polkabot.ai` project. | Valuable for anyone wanting to evaluate the architecture of Polkabot or potentially build upon its foundation. |

## 3. Related Projects & Strategic Context

These are not standalone tools but represent important context, learnings, or related efforts within the ecosystem.

### [W3F Grants Programme: mcp-polkadot](https://grants.web3.foundation/applications/mcp-polkadot)

* **Summary:** The `mcp-polkadot` project, funded through the Web3 Foundation grants programme, indicates wider ecosystem interest in AI tooling and represents another team to potentially engage with for shared learnings.

### Parity Forum Post: "I built a Polkadot ChatGPT bot"

* **Source:** [forum.parity.io](https://forum.parity.io/t/i-built-a-polkadot-chatgpt-bot-here-are-the-things-i-learned/1798)
* **Summary:** A crucial first-hand account from a developer who built a similar tool. The post highlights key challenges, particularly around data accuracy and the difficulty of keeping the AI's knowledge base synchronised with the rapidly evolving Polkadot codebase. This serves as a valuable collection of lessons learned.
