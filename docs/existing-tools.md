# Summary of the Polkadot AI Tooling Landscape

This document is based on a series of discussions regarding the development of a new AI-powered developer tool.

## Introduction

The Polkadot ecosystem has several existing AI-powered tools and related projects. These initiatives range from public-facing chatbots to open-source developer tools and proofs of concept. The consensus from our discussions is that while these tools exist, there is a clear need for a unified, trustworthy, and deeply code-aware solution to better serve developers. This summary captures the known projects to provide context for future work.

## 1. Public-Facing Chatbots

These are live tools that users can interact with directly.

| Tool Name | Link/Source | Description | Key Notes & Status |
| :--- | :--- | :--- | :--- |
| **Polkadot Wiki's "Ask AI"** | [Polkadot Wiki](https://wiki.polkadot.network/) | An AI assistant embedded in the Polkadot Wiki and Docs to answer questions based on the documentation. | Powered by a third-party service (Kapa AI). It's effective for documentation-based queries but lacks direct knowledge of the SDK's source code. A key proposal is to enhance this tool by adding the SDK repo as a data source. |
| **Polkabot.ai** | [polkabot.ai](https://polkabot.ai/) | A general-purpose chatbot for the Polkadot ecosystem. | It was noted that some of its real-time data (e.g., DOT price) appeared outdated during our review. Its source code seems to be available on GitLab (see below). |

## 2. Developer Tools & Open-Source Projects

These are code repositories and tools aimed at developers.

| Tool Name | Link/Source | Description | Key Notes & Status |
| :--- | :--- | :--- | :--- |
| **dAppForge-copilot** | [GitHub Repo](https://github.com/neurons-lab/dAppForge-copilot) | An AI copilot tool likely focused on assisting with dApp development on Polkadot. | Shared as an example of existing work in the ecosystem. Appears to be a developer-focused tool rather than a general-purpose chatbot. |
| **Polkabot (Source Code)** | [GitLab Repo](https://gitlab.com/Polkabot/polkabot) | The source code for the `Polkabot.ai` project. | Valuable for anyone wanting to evaluate the architecture of Polkabot or potentially build upon its foundation. |

## 3. Related Projects & Strategic Context

These are not standalone tools but represent important context, learnings, or related efforts within the ecosystem.

### Parity Forum Post: "I built a Polkadot ChatGPT bot"
* **Source:** [forum.parity.io](https://forum.parity.io/t/i-built-a-polkadot-chatgpt-bot-here-are-the-things-i-learned/1798)
* **Summary:** A crucial first-hand account from a developer who built a similar tool. The post highlights key challenges, particularly around data accuracy and the difficulty of keeping the AI's knowledge base synchronized with the rapidly evolving Polkadot codebase. This serves as a valuable collection of lessons learned.

### Other Identified Projects
* **Sources:** `mcp-polkadot`, `dappforge.app`, `layuplabs.ai`
* **Summary:** These projects were noted in planning documents as related works. A formal review is needed to understand their goals, identify potential for collaboration, and avoid duplicating effort.

### W3F Grants Program
* **Summary:** It was mentioned that a similar AI-focused project has been funded through the Web3 Foundation grants program. This indicates wider ecosystem interest and represents another team to potentially engage with for shared learnings.

## Conclusion

The current landscape of Polkadot AI tools is characterized by several independent efforts. There is a clear distinction between documentation-focused bots (like the Wiki's Ask AI) and more general or developer-focused tools. The recurring challenge identified across all discussions is the need for a solution that is deeply integrated with the source code of the Polkadot SDK, ensuring that developers receive information that is not just conversational, but also accurate, reliable, and perfectly synchronized with the underlying code. This gap is the primary driver for the proposed MVP to build a "live documentation" AI expert.
