# Asset Conversion Pallet

## Introduction

The [Asset Conversion Pallet](https://docs.polkadot.com/tutorials/polkadot-sdk/system-chains/asset-hub/asset-conversion/) is a **foundational "DeFi Lego brick"** for the Polkadot ecosystem. It is a native module that any parachain can easily include to get the core functionality of a DEX without having to build one from scratch.

Its logic is based on the well-established [**Uniswap v2 model**](https://github.com/Uniswap/v2-core), using the `x*y=k` constant product formula to allow for permissionless creation of liquidity pools and the swapping of assets. It provides the essential, system-level infrastructure for on-chain trading.

## Understanding the Key Functions

Let's break down the two transaction examples you provided from the Paseo testnet.

### Swapping Tokens

This transaction calls the [`swapExactTokensForTokens`](https://docs.polkadot.com/tutorials/polkadot-sdk/system-chains/asset-hub/asset-conversion/#swap-from-an-exact-amount-of-tokens) function. The user wants to sell an exact amount of one token for another.

```json
[
  {
    "name": "path",
    "type": "Vec<staging_xcm:v4:location:Location>",
    "type_name": "Vec<Box<AssetKind>>",
    "value": [
      {
        "interior": {
          "Here": "NULL"
        },
        "parents": 1
      },
      {
        "interior": {
          "X2": [
            {
              "PalletInstance": 50
            },
            {
              "GeneralIndex": "1984"
            }
          ]
        },
        "parents": 0
      }
    ]
  },
  {
    "name": "amount_in",
    "type": "U128",
    "type_name": "Balance",
    "value": "5000000000000"
  },
  {
    "name": "amount_out_min",
    "type": "U128",
    "type_name": "Balance",
    "value": "1"
  },
  {
    "name": "send_to",
    "type": "AccountId",
    "type_name": "AccountId",
    "value": "0x9818ff3c27d256631065ecabf0c50e02551e5c5342b8669486c1e566fcbf847f"
  },
  {
    "name": "keep_alive",
    "type": "Bool",
    "type_name": "bool",
    "value": false
  }
]
```

**Decoding the JSON:**

* **`path`**: This defines the trade route. It's a list with two assets:
    * The first asset (`"parents": 1, "interior": {"Here": "NULL"}`) is the `Location` for **Paseo's native token (PAS)**.
    * The second asset (`"parents": 0, "interior": {"X2": ... "GeneralIndex": "1984"}}`) is the `Location` for an asset on the local chain (Asset Hub), specifically the one with index **1984**, which is **USDT**.
    * **In short, this is the path for a `PAS -> USDT` swap.**
* **`amount_in`: `"5000000000000"`**
    * This is the exact amount of the input asset (PAS) the user wants to sell. Blockchains don't use decimal points, so you've to account for the token's precision. With 12 decimals, this value represents **5,000 PAS**.
* **`amount_out_min`: `"1"`**
    * This is the user's slippage protection. They are saying, "I don't care what the final price is, as long as I receive at least 1 unit of USDT."
* **`send_to`**: The address where the final USDT will be sent.
* **`keep_alive`: `false`**
    * The user is allowing this transaction to proceed even if it would cause their PAS balance to drop below the minimum required to keep the account active.

**AssetConversion (SwapExecuted):**

```json
[
  {
    "type": "AccountId",
    "type_name": "AccountId",
    "value": "0x9818ff3c27d256631065ecabf0c50e02551e5c5342b8669486c1e566fcbf847f",
    "name": "who"
  },
  {
    "type": "AccountId",
    "type_name": "AccountId",
    "value": "0x9818ff3c27d256631065ecabf0c50e02551e5c5342b8669486c1e566fcbf847f",
    "name": "send_to"
  },
  {
    "type": "U128",
    "type_name": "Balance",
    "value": "5000000000000",
    "name": "amount_in"
  },
  {
    "type": "U128",
    "type_name": "Balance",
    "value": "8301",
    "name": "amount_out"
  },
  {
    "type": "Vec<Tuple:staging_xcm:v4:location:LocationU128>",
    "type_name": "BalancePath",
    "value": [
      {
        "col1": {
          "interior": {
            "Here": "NULL"
          },
          "parents": 1
        },
        "col2": "5000000000000"
      },
      {
        "col1": {
          "interior": {
            "X2": [
              {
                "PalletInstance": 50
              },
              {
                "GeneralIndex": "1984"
              }
            ]
          },
          "parents": 0
        },
        "col2": "8301"
      }
    ],
    "name": "path"
  }
]
```

### Adding Liquidity to a Pool

This transaction calls the [`addLiquidity`](https://docs.polkadot.com/tutorials/polkadot-sdk/system-chains/asset-hub/asset-conversion/#add-liquidity-to-a-pool) function. The user wants to deposit a pair of tokens into a liquidity pool to become a Liquidity Provider (LP).

```json
[
  {
    "name": "asset1",
    "type": "staging_xcm:v4:location:Location",
    "type_name": "Box<AssetKind>",
    "value": {
      "interior": {
        "Here": "NULL"
      },
      "parents": 1
    }
  },
  {
    "name": "asset2",
    "type": "staging_xcm:v4:location:Location",
    "type_name": "Box<AssetKind>",
    "value": {
      "interior": {
        "X2": [
          {
            "PalletInstance": 50
          },
          {
            "GeneralIndex": "1984"
          }
        ]
      },
      "parents": 0
    }
  },
  {
    "name": "amount1_desired",
    "type": "U128",
    "type_name": "Balance",
    "value": "3000000000000"
  },
  {
    "name": "amount2_desired",
    "type": "U128",
    "type_name": "Balance",
    "value": "8301"
  },
  {
    "name": "amount1_min",
    "type": "U128",
    "type_name": "Balance",
    "value": "1"
  },
  {
    "name": "amount2_min",
    "type": "U128",
    "type_name": "Balance",
    "value": "1"
  },
  {
    "name": "mint_to",
    "type": "AccountId",
    "type_name": "AccountId",
    "value": "0x9818ff3c27d256631065ecabf0c50e02551e5c5342b8669486c1e566fcbf847f"
  }
]
```

**Decoding the JSON:**

* **`asset1`**: This is the `Location` for **Paseo's native token (PAS)**, the same as above.
* **`asset2`**: This is the `Location` for **USDT (asset 1984)**, the same as above.
    * **In short, the user is adding liquidity to the `PAS/USDT` pool.**
* **`amount1_desired`: `"3000000000000"`**
    * The user *wishes* to deposit **3,000 PAS**.
* **`amount2_desired`: `"8301"`**
    * The user *wishes* to deposit **8,301 units of USDT**. (USDT typically has 6 decimals).
* **`amount1_min`: `"1"`** and **`amount2_min`: `"1"`**
    * This is the user's protection. They are saying, "I'm willing to proceed with this transaction as long as at least 1 unit of each token is successfully deposited." This protects them from unexpected price changes. The function will automatically calculate the *optimal* deposit ratio based on the pool's current state, ensuring it doesn't use more of their funds than necessary.
* **`mint_to`**: The address where the newly created LP tokens (representing their share of the pool) will be sent.

**AssetConversion (LiquidityAdded):**

```json
[
  {
    "type": "AccountId",
    "type_name": "AccountId",
    "value": "0x9818ff3c27d256631065ecabf0c50e02551e5c5342b8669486c1e566fcbf847f",
    "name": "who"
  },
  {
    "type": "AccountId",
    "type_name": "AccountId",
    "value": "0x9818ff3c27d256631065ecabf0c50e02551e5c5342b8669486c1e566fcbf847f",
    "name": "mint_to"
  },
  {
    "type": "Tuple:staging_xcm:v4:location:Locationstaging_xcm:v4:location:Location",
    "type_name": "PoolId",
    "value": {
      "col1": {
        "interior": {
          "Here": "NULL"
        },
        "parents": 1
      },
      "col2": {
        "interior": {
          "X2": [
            {
              "PalletInstance": 50
            },
            {
              "GeneralIndex": "1984"
            }
          ]
        },
        "parents": 0
      }
    },
    "name": "pool_id"
  },
  {
    "type": "U128",
    "type_name": "Balance",
    "value": "3000000000000",
    "name": "amount1_provided"
  },
  {
    "type": "U128",
    "type_name": "Balance",
    "value": "4895",
    "name": "amount2_provided"
  },
  {
    "type": "U32",
    "type_name": "PoolAssetId",
    "value": 5,
    "name": "lp_token"
  },
  {
    "type": "U128",
    "type_name": "Balance",
    "value": "120500612",
    "name": "lp_token_minted"
  }
]
```

### Other Functionalities

- [**Creating a Liquidity Pool: `createPool`**](https://docs.polkadot.com/tutorials/polkadot-sdk/system-chains/asset-hub/asset-conversion/#create-a-liquidity-pool)
    This is the foundational step for any new market. It allows a user to establish a new, empty liquidity pool for any pair of tokens that doesn't already have one. The creator pays a small, one-time fee to register the pool on the blockchain, which also mints a new, unique LP (Liquidity Provider) token that will represent a share of this specific pool. Once created, the pool is ready to accept deposits.

- [**Withdrawing Liquidity from a Pool: `removeLiquidity`**](https://docs.polkadot.com/tutorials/polkadot-sdk/system-chains/asset-hub/asset-conversion/#withdraw-liquidity-from-a-pool)
    This function allows liquidity providers to redeem their share of the pool's assets. The user specifies the amount of LP tokens they wish to burn. The function then calculates their proportional share of the two underlying tokens in the pool and transfers those assets back to the user's wallet. The user's LP tokens are permanently destroyed in the process.

- [**Swapping Tokens (Exact Output): `swapTokensForExactTokens`**](https://docs.polkadot.com/tutorials/polkadot-sdk/system-chains/asset-hub/asset-conversion/#swap-to-an-exact-amount-of-tokens)
    This is the alternative swap function. It allows a user to specify the *exact amount* of the output token they want to receive. The function then calculates the required input amount needed to achieve this. The user is protected by specifying the maximum input amount they are willing to pay.

## References

- [`pallet-asset-conversion`](https://paritytech.github.io/polkadot-sdk/master/pallet_asset_conversion/index.html)
- [Polkadot Wiki: Asset Conversion](https://wiki.polkadot.com/learn/learn-asset-conversion-assethub/)
- [Uniswap](https://docs.uniswap.org)
- [Uniswap v2 Core Whitepaper](https://app.uniswap.org/whitepaper.pdf)
- [Smart Contract Programmer: Uniswap v2](https://youtu.be/t0NZq8SmywU)
- [Whiteboard Crypto: What is Uniswap?](https://youtu.be/DLu35sIqVTM)
