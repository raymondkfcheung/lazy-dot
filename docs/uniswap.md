# The Evolution of Uniswap: A Comparative Study

The Uniswap Protocol has evolved significantly since its inception, with four major versions now deployed on the Ethereum blockchain. Each iteration introduced groundbreaking features, moving from a simple proof-of-concept to a highly efficient and customisable platform for decentralised finance.

## Feature-by-Feature Comparison

| Feature            | Uniswap v1                             | Uniswap v2                  | Uniswap v3                  | Uniswap v4                         |
|:-------------------|:---------------------------------------|:----------------------------|:----------------------------|:-----------------------------------|
| Launch Date        | November 2018                          | May 2020                    | May 2021                    | January 2025                       |
| Core Model         | ETH-to-Token AMM                       | Generalised AMM             | Concentrated Liquidity      | Programmable Liquidity             |
| Key Innovation     | Natively pairing any token against ETH | Direct ERC20-to-ERC20 pools | Custom Price Ranges         | Hooks & Singleton Architecture     |
| Capital Efficiency | Very Low                               | Low                         | High                        | Very High (plus gas efficiency)    |
| LP Position Token  | ERC-20 (Fungible)                      | ERC-20 (Fungible)           | ERC-721 (Non-Fungible)      | ERC-721 (Non-Fungible)             |
| Fee Structure      | Fixed 0.3%                             | Fixed 0.3%                  | Multiple Tiers              | Customisable Tiers & Dynamic Fees  |
| Licensing          | GPL                                    | GPL                         | Source Available (Modified) | Source Available (Modified)        |
| Primary Drawback   | The "ETH Bridge" problem               | High capital inefficiency   | High complexity for LPs     | Increased security risks via Hooks |

## Summary of Each Version

### Uniswap v1 (Nov 2018)

The first version of the protocol and the first AMM to natively pair tokens against ETH. It proved the viability of on-chain, automated market making but was limited, as all trades had to route through ETH.

### Uniswap v2 (May 2020)

This version brought a crucial upgrade, allowing any Ethereum-based (ERC-20) token to be traded directly for any other. This greatly increased the protocol's flexibility, making it the foundational trading layer for the "DeFi Summer" and the broader ecosystem.

### Uniswap v3 (May 2021)

Uniswap v3 introduced concentrated liquidity, a revolutionary feature allowing liquidity providers to choose specific price ranges for their capital. This dramatically increased capital efficiency, transforming liquidity provision into a more active and strategic process.

### Uniswap v4 (Jan 2025)

The latest version focuses on customisation and efficiency. It introduces s, which allow developers to create custom AMM features (like dynamic fees or on-chain limit orders) directly on top of Uniswap's deep liquidity. This, combined with architectural changes that reduce network costs, turns the protocol into a powerful and flexible platform for financial innovation.

## References

- [Versions of the Uniswap Protocol](https://support.uniswap.org/hc/en-us/sections/35946060713997-Versions-of-the-Uniswap-Protocol)
- [Uniswap v2, v3, and v4](https://support.uniswap.org/hc/en-us/articles/7425482965517-Uniswap-v2-v3-and-v4)
