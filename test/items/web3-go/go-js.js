function make_out(item) {
    const all_lines = item.split('\r\n').filter(v=>v)
    const privateKey = all_lines[0]
    const erc20Address = all_lines[1]
    const toAddress = all_lines[2]
    const allABI = all_lines.filter(v=>v[0] === '@')
        .map(v=>v.slice(1))
        .map(v=>{
            const all_secs = v.split(';');
            const inputs = all_secs[1].split('!')
            const outputs = all_secs[3].split("!")
            return `{
                "constants": ${all_secs[0]},
                "inputs": [${inputs.map(a=>{
                    const kv = a.split(',')
                    return `{"name":"${
                        kv[0]
                    }","type": "${kv[1]}"}`
            }).join(',')}],
            "name": "${all_secs[2]}",
            "outputs":[${outputs.map(a=>{
                const kv = a.split(',')
                return `{"name":"${
                    kv[0]
                }","type": "${kv[1]}"}`
            }).join(',')}],
            "type": "${all_secs[4]}"
            }`
        }).join(',')
    return `
    package main

import (
\t"context"
\t"fmt"
\t"log"
\t"math/big"

\t"github.com/ethereum/go-ethereum/accounts/abi/bind"
\t"github.com/ethereum/go-ethereum/common"
\t"github.com/ethereum/go-ethereum/core/types"
\t"github.com/ethereum/go-ethereum/crypto"
\t"github.com/ethereum/go-ethereum/ethclient"
)

// -------------------------- Constant Configuration --------------------------
const (
\t// Ethereum node RPC address (Sepolia testnet example)
\tRPC_URL       = "https://rpc.ankr.com/eth_sepolia"
\t// Test private key (for testing only! Secure storage required in production)
\tPRIVATE_KEY   = "${privateKey}"
\t// ERC20 contract address (Sepolia testnet USDT example)
\tERC20_ADDRESS = "${erc20Address}"
\t// Target transfer address
\tTO_ADDRESS    = "${toAddress}"
)

// -------------------------- ERC20 Contract ABI (Simplified) --------------------------
// Note: In actual development, import the complete ABI from contract compilation artifacts
const ERC20_ABI = \`[
${allABI}
]\`

// -------------------------- Main Function: Integrate All Scenarios --------------------------
func main() {
\t// 1. Connect to Ethereum node
\tclient, err := ethclient.Dial(RPC_URL)
\tif err != nil {
\t\tlog.Fatalf("Failed to connect to node: %v", err)
\t}
\tdefer client.Close()
\tfmt.Println("✅ Successfully connected to Ethereum node")

\t// 2. Query basic chain information (block height, gas price)
\tblockNumber, err := client.BlockNumber(context.Background())
\tif err != nil {
\t\tlog.Fatalf("Failed to get block number: %v", err)
\t}
\tfmt.Printf("📌 Current block height: %d\\n", blockNumber)

\tgasPrice, err := client.SuggestGasPrice(context.Background())
\tif err != nil {
\t\tlog.Fatalf("Failed to get gas price: %v", err)
\t}
\tfmt.Printf("📌 Current suggested gas price: %s Wei\\n", gasPrice.String())

\t// 3. Query account ETH balance
\tfromAddr := crypto.PubkeyToAddress(getPrivateKey().PublicKey)
\tethBalance, err := client.BalanceAt(context.Background(), fromAddr, nil)
\tif err != nil {
\t\tlog.Fatalf("Failed to get ETH balance: %v", err)
\t}
\t// Convert to ETH unit (1 ETH = 1e18 Wei)
\tethBalanceEth := new(big.Float).Quo(new(big.Float).SetInt(ethBalance), big.NewFloat(1e18))
\tfmt.Printf("💳 ETH balance of account %s: %f ETH\\n", fromAddr.Hex(), ethBalanceEth)

\t// 4. ERC20 contract interaction: Query token balance
\terc20Contract, err := bind.NewBoundContract(common.HexToAddress(ERC20_ADDRESS), []byte(ERC20_ABI), client, client, client)
\tif err != nil {
\t\tlog.Fatalf("Failed to initialize ERC20 contract: %v", err)
\t}

\tvar tokenBalance *big.Int
\terr = erc20Contract.Call(&bind.CallOpts{}, &tokenBalance, "balanceOf", fromAddr)
\tif err != nil {
\t\tlog.Fatalf("Failed to get token balance: %v", err)
\t}
\t// Convert to token unit (assuming 18 decimals)
\ttokenBalanceFormatted := new(big.Float).Quo(new(big.Float).SetInt(tokenBalance), big.NewFloat(1e18))
\tfmt.Printf("💳 ERC20 token balance of account %s: %f\\n", fromAddr.Hex(), tokenBalanceFormatted)

\t// 5. Send ETH transfer
\t// sendETH(client, TO_ADDRESS, big.NewFloat(0.001)) // Uncomment to execute

\t// 6. ERC20 token transfer
\t// transferERC20(erc20Contract, TO_ADDRESS, big.NewInt(1e18)) // Uncomment to execute
}

// -------------------------- Utility Function: Get Private Key --------------------------
func getPrivateKey() *crypto.PrivateKey {
\tprivateKey, err := crypto.HexToECDSA(PRIVATE_KEY)
\tif err != nil {
\t\tlog.Fatalf("Failed to parse private key: %v", err)
\t}
\treturn privateKey
}

// -------------------------- Scenario 1: Send ETH Transfer --------------------------
func sendETH(client *ethclient.Client, toAddr string, amountEth *big.Float) {
\tprivateKey := getPrivateKey()
\tfromAddr := crypto.PubkeyToAddress(privateKey.PublicKey)

\t// Convert ETH amount to Wei
\tamountWei := new(big.Int)
\tamountEth.Mul(amountEth, big.NewFloat(1e18)).Int(amountWei)

\t// Get nonce (next transaction sequence number for account)
\tnonce, err := client.PendingNonceAt(context.Background(), fromAddr)
\tif err != nil {
\t\tlog.Fatalf("Failed to get nonce: %v", err)
\t}

\t// Build transaction
\tgasPrice, err := client.SuggestGasPrice(context.Background())
\tif err != nil {
\t\tlog.Fatalf("Failed to get gas price: %v", err)
\t}
\tgasLimit := uint64(21000) // Fixed gas limit for ETH transfer
\ttx := types.NewTransaction(
\t\tnonce,
\t\tcommon.HexToAddress(toAddr),
\t\tamountWei,
\t\tgasLimit,
\t\tgasPrice,
\t\tnil, // No data (ETH transfer)
\t)

\t// Sign transaction
\tchainID, err := client.ChainID(context.Background())
\tif err != nil {
\t\tlog.Fatalf("Failed to get chain ID: %v", err)
\t}
\tsignedTx, err := types.SignTx(tx, types.NewEIP155Signer(chainID), privateKey)
\tif err != nil {
\t\tlog.Fatalf("Failed to sign transaction: %v", err)
\t}

\t// Send transaction
\terr = client.SendTransaction(context.Background(), signedTx)
\tif err != nil {
\t\tlog.Fatalf("Failed to send transaction: %v", err)
\t}
\tfmt.Printf("🚀 ETH transfer transaction sent, hash: %s\\n", signedTx.Hash().Hex())
}

// -------------------------- Scenario 2: ERC20 Token Transfer --------------------------
func transferERC20(contract *bind.BoundContract, toAddr string, amount *big.Int) {
\tprivateKey := getPrivateKey()
\tfromAddr := crypto.PubkeyToAddress(privateKey.PublicKey)

\t// Get nonce
\tnonce, err := client.PendingNonceAt(context.Background(), fromAddr)
\tif err != nil {
\t\tlog.Fatalf("Failed to get nonce: %v", err)
\t}

\t// Build transaction options
\tgasPrice, err := client.SuggestGasPrice(context.Background())
\tif err != nil {
\t\tlog.Fatalf("Failed to get gas price: %v", err)
\t}
\topts := &bind.TransactOpts{
\t\tFrom:     fromAddr,
\t\tNonce:    big.NewInt(int64(nonce)),
\t\tSigner: func(address common.Address, tx *types.Transaction) (*types.Transaction, error) {
\t\t\tchainID, _ := client.ChainID(context.Background())
\t\t\treturn types.SignTx(tx, types.NewEIP155Signer(chainID), privateKey)
\t\t},
\t\tGasPrice: gasPrice,
\t\tGasLimit: uint64(100000), // Suggested gas limit for ERC20 transfer
\t\tContext:  context.Background(),
\t}

\t// Call transfer method
\ttx, err := contract.Transact(opts, "transfer", common.HexToAddress(toAddr), amount)
\tif err != nil {
\t\tlog.Fatalf("ERC20 transfer failed: %v", err)
\t}
\tfmt.Printf("🚀 ERC20 transfer transaction sent, hash: %s\\n", tx.Hash().Hex())
}
    `
}

make_out.path = '@/simple.go'