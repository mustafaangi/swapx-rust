window.addEventListener('load', async () => {
    const { ApiPromise, WsProvider } = require('@polkadot/api');
    const { ContractPromise } = require('@polkadot/api-contract');
    const { web3Accounts, web3Enable } = require('@polkadot/extension-dapp');

    try {
        // Enable Polkadot.js extension
        const extensions = await web3Enable('SwapX DApp');
        const accounts = await web3Accounts();

        const wsProvider = new WsProvider('ws://127.0.0.1:9944');
        const api = await ApiPromise.create({ provider: wsProvider });
        
        const contractAddress = 'YOUR_DEPLOYED_CONTRACT_ADDRESS';
        const contractABI = require('./target/ink/swapx.json');
        const contract = new ContractPromise(api, contractABI, contractAddress);

        // Replace event listeners to use Polkadot.js
        document.getElementById('swapButton').addEventListener('click', async () => {
            const tokenIn = document.getElementById('tokenIn').value;
            const amountIn = document.getElementById('amountIn').value;
            const tokenOut = document.getElementById('tokenOut').value;
            const slippage = document.getElementById('slippage').value;

            const swapRate = await contract.query.getSwapRate(accounts[0].address, {}, tokenIn, tokenOut);
            const amountOut = (amountIn * swapRate.output.toNumber()) / 1e18;
            const minAmountOut = amountOut - (amountOut * slippage) / 100;

            try {
                await contract.tx.swapTokens({ value: 0, gasLimit: -1 }, tokenIn, amountIn, tokenOut, minAmountOut);
                document.getElementById('status').innerText = 'Swap successful!';
            } catch (error) {
                document.getElementById('status').innerText = `Error: ${error.message}`;
            }
        });

        document.getElementById('addLiquidityButton').addEventListener('click', async () => {
            const token = document.getElementById('liquidityToken').value;
            const amount = document.getElementById('liquidityAmount').value;

            try {
                await contract.tx.addLiquidity({ value: 0, gasLimit: -1 }, token, amount);
                document.getElementById('status').innerText = 'Liquidity added successfully!';
            } catch (error) {
                document.getElementById('status').innerText = `Error: ${error.message}`;
            }
        });

        document.getElementById('removeLiquidityButton').addEventListener('click', async () => {
            const token = document.getElementById('removeLiquidityToken').value;
            const amount = document.getElementById('removeLiquidityAmount').value;

            try {
                await contract.tx.removeLiquidity({ value: 0, gasLimit: -1 }, token, amount);
                document.getElementById('status').innerText = 'Liquidity removed successfully!';
            } catch (error) {
                document.getElementById('status').innerText = `Error: ${error.message}`;
            }
        });

    } catch (error) {
        console.error('Error initializing app:', error);
        document.getElementById('status').innerText = `Initialization Error: ${error.message}`;
    }
});