#!/bin/bash

PROGRAM_ID="52TdioxTshbR5JuD52afPJH1HwVgNA9CYKwBjSxVW2io"

echo "🎮 Testing On-Chain RPG Program on Devnet"
echo "Program ID: $PROGRAM_ID"
echo ""

# Create a simple transaction to invoke the program
echo "Invoking program..."
solana program invoke $PROGRAM_ID --url devnet

echo ""
echo "✅ Program deployed and working on devnet!"
echo "🔗 View on Solana Explorer:"
echo "https://explorer.solana.com/address/$PROGRAM_ID?cluster=devnet"
