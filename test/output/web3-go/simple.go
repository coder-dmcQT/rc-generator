
    package main

import (
	"context"
	"fmt"
	"log"
	"math/big"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/ethclient"
)

// -------------------------- Constant Configuration --------------------------
const (
	// Ethereum node RPC address (Sepolia testnet example)
	RPC_URL       = "https://rpc.ankr.com/eth_sepolia"
	// Test private key (for testing only! Secure storage required in production)
	PRIVATE_KEY   = "a267530f49f8280200edf313ee7af6b827f2a8bce2897751d06a843f644967b1"
	// ERC20 contract address (Sepolia testnet USDT example)
	ERC20_ADDRESS = "0x55296f69f40Ea6d20E478533C15A6B08B654E758"
	// Target transfer address
	TO_ADDRESS    = "0x55296f69f40Ea6d20E478533C15A6B08B654E758"
)

// -------------------------- ERC20 Contract ABI (Simplified) --------------------------
// Note: In actual development, import the complete ABI from contract compilation artifacts
const ERC20_ABI = `[
{
                "constants": true,
                "inputs": [{"name":"_owner","type": "address"}],
            "name": "balanceOf",
            "outputs":[{"name":"balance","type": "uint256"}],
            "type": "function"
            },{
                "constants": false,
                "inputs": [{"name":"_to","type": "address"},{"name":"_value","type": "uint256"}],
            "name": "transfer",
            "outputs":[{"name":"success","type": "bool"}],
            "type": "function"
            }
]`

// -------------------------- Main Function: Integrate All Scenarios --------------------------
func main() {
	// 1. Connect to Ethereum node
	client, err := ethclient.Dial(RPC_URL)
	if err != nil {
		log.Fatalf("Failed to connect to node: %v", err)
	}
	defer client.Close()
	fmt.Println("✅ Successfully connected to Ethereum node")

	// 2. Query basic chain information (block height, gas price)
	blockNumber, err := client.BlockNumber(context.Background())
	if err != nil {
		log.Fatalf("Failed to get block number: %v", err)
	}
	fmt.Printf("📌 Current block height: %d\n", blockNumber)

	gasPrice, err := client.SuggestGasPrice(context.Background())
	if err != nil {
		log.Fatalf("Failed to get gas price: %v", err)
	}
	fmt.Printf("📌 Current suggested gas price: %s Wei\n", gasPrice.String())

	// 3. Query account ETH balance
	fromAddr := crypto.PubkeyToAddress(getPrivateKey().PublicKey)
	ethBalance, err := client.BalanceAt(context.Background(), fromAddr, nil)
	if err != nil {
		log.Fatalf("Failed to get ETH balance: %v", err)
	}
	// Convert to ETH unit (1 ETH = 1e18 Wei)
	ethBalanceEth := new(big.Float).Quo(new(big.Float).SetInt(ethBalance), big.NewFloat(1e18))
	fmt.Printf("💳 ETH balance of account %s: %f ETH\n", fromAddr.Hex(), ethBalanceEth)

	// 4. ERC20 contract interaction: Query token balance
	erc20Contract, err := bind.NewBoundContract(common.HexToAddress(ERC20_ADDRESS), []byte(ERC20_ABI), client, client, client)
	if err != nil {
		log.Fatalf("Failed to initialize ERC20 contract: %v", err)
	}

	var tokenBalance *big.Int
	err = erc20Contract.Call(&bind.CallOpts{}, &tokenBalance, "balanceOf", fromAddr)
	if err != nil {
		log.Fatalf("Failed to get token balance: %v", err)
	}
	// Convert to token unit (assuming 18 decimals)
	tokenBalanceFormatted := new(big.Float).Quo(new(big.Float).SetInt(tokenBalance), big.NewFloat(1e18))
	fmt.Printf("💳 ERC20 token balance of account %s: %f\n", fromAddr.Hex(), tokenBalanceFormatted)

	// 5. Send ETH transfer
	// sendETH(client, TO_ADDRESS, big.NewFloat(0.001)) // Uncomment to execute

	// 6. ERC20 token transfer
	// transferERC20(erc20Contract, TO_ADDRESS, big.NewInt(1e18)) // Uncomment to execute
}

// -------------------------- Utility Function: Get Private Key --------------------------
func getPrivateKey() *crypto.PrivateKey {
	privateKey, err := crypto.HexToECDSA(PRIVATE_KEY)
	if err != nil {
		log.Fatalf("Failed to parse private key: %v", err)
	}
	return privateKey
}

// -------------------------- Scenario 1: Send ETH Transfer --------------------------
func sendETH(client *ethclient.Client, toAddr string, amountEth *big.Float) {
	privateKey := getPrivateKey()
	fromAddr := crypto.PubkeyToAddress(privateKey.PublicKey)

	// Convert ETH amount to Wei
	amountWei := new(big.Int)
	amountEth.Mul(amountEth, big.NewFloat(1e18)).Int(amountWei)

	// Get nonce (next transaction sequence number for account)
	nonce, err := client.PendingNonceAt(context.Background(), fromAddr)
	if err != nil {
		log.Fatalf("Failed to get nonce: %v", err)
	}

	// Build transaction
	gasPrice, err := client.SuggestGasPrice(context.Background())
	if err != nil {
		log.Fatalf("Failed to get gas price: %v", err)
	}
	gasLimit := uint64(21000) // Fixed gas limit for ETH transfer
	tx := types.NewTransaction(
		nonce,
		common.HexToAddress(toAddr),
		amountWei,
		gasLimit,
		gasPrice,
		nil, // No data (ETH transfer)
	)

	// Sign transaction
	chainID, err := client.ChainID(context.Background())
	if err != nil {
		log.Fatalf("Failed to get chain ID: %v", err)
	}
	signedTx, err := types.SignTx(tx, types.NewEIP155Signer(chainID), privateKey)
	if err != nil {
		log.Fatalf("Failed to sign transaction: %v", err)
	}

	// Send transaction
	err = client.SendTransaction(context.Background(), signedTx)
	if err != nil {
		log.Fatalf("Failed to send transaction: %v", err)
	}
	fmt.Printf("🚀 ETH transfer transaction sent, hash: %s\n", signedTx.Hash().Hex())
}

// -------------------------- Scenario 2: ERC20 Token Transfer --------------------------
func transferERC20(contract *bind.BoundContract, toAddr string, amount *big.Int) {
	privateKey := getPrivateKey()
	fromAddr := crypto.PubkeyToAddress(privateKey.PublicKey)

	// Get nonce
	nonce, err := client.PendingNonceAt(context.Background(), fromAddr)
	if err != nil {
		log.Fatalf("Failed to get nonce: %v", err)
	}

	// Build transaction options
	gasPrice, err := client.SuggestGasPrice(context.Background())
	if err != nil {
		log.Fatalf("Failed to get gas price: %v", err)
	}
	opts := &bind.TransactOpts{
		From:     fromAddr,
		Nonce:    big.NewInt(int64(nonce)),
		Signer: func(address common.Address, tx *types.Transaction) (*types.Transaction, error) {
			chainID, _ := client.ChainID(context.Background())
			return types.SignTx(tx, types.NewEIP155Signer(chainID), privateKey)
		},
		GasPrice: gasPrice,
		GasLimit: uint64(100000), // Suggested gas limit for ERC20 transfer
		Context:  context.Background(),
	}

	// Call transfer method
	tx, err := contract.Transact(opts, "transfer", common.HexToAddress(toAddr), amount)
	if err != nil {
		log.Fatalf("ERC20 transfer failed: %v", err)
	}
	fmt.Printf("🚀 ERC20 transfer transaction sent, hash: %s\n", tx.Hash().Hex())
}
    