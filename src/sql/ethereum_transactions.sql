-- Ethereum Transaction Query
-- 
-- Retrieves all transactions associated with a specific Ethereum address
-- (both sent and received)
-- 
-- Parameters:
-- {{wallet_address}} - The Ethereum address to query transactions for
-- {{limit}} - Maximum number of transactions to return per query
-- {{offset}} - Number of transactions to skip for pagination
-- 
-- Returns:
-- - transaction_hash: Unique transaction identifier
-- - base_fee_per_gas: Base fee per gas unit in wei
-- - block_number: Block number containing the transaction
-- - contract_address: Address of contract if created in transaction
-- - fees_burned: Amount of ETH burned in transaction
-- - fees_rewarded: Transaction fees rewarded to miner
-- - fees_saved: Fees saved through EIP-1559
-- - from_address: Sender address
-- - gas_limit: Maximum gas units allowed
-- - gas_price: Price per gas unit in wei
-- - gas_used: Actual gas units consumed
-- - input: Transaction input data
-- - internal_transaction_count: Number of internal transactions
-- - log_count: Number of event logs emitted
-- - max_fee_per_gas: Maximum fee per gas willing to pay
-- - max_priority_fee_per_gas: Maximum priority fee per gas
-- - nonce: Transaction nonce
-- - output: Transaction output data
-- - position: Transaction position in block
-- - timestamp: Transaction timestamp
-- - to_address: Recipient address
-- - transaction_fee: Total transaction fee paid
-- - type: Transaction type (0=Legacy, 1=AccessList, 2=EIP1559)
-- - value: Amount of ETH transferred in wei
SELECT
    t.transaction_hash,
    t.base_fee_per_gas,
    t.block_number,
    t.contract_address,
    t.fees_burned,
    t.fees_rewarded,
    t.fees_saved,
    t.from_address,
    t.gas_limit,
    t.gas_price,
    t.gas_used,
    t.input,
    t.internal_failed_transaction_count,
    t.internal_transaction_count,
    t.log_count,
    t.max_fee_per_gas,
    t.max_priority_fee_per_gas,
    t.nonce,
    t.output,
    t.position,
    t.timestamp,
    t.to_address,
    t.transaction_fee,
    t.type,
    t.value
FROM ethereum.transactions t
WHERE t.from_address = '{{wallet_address}}'
   OR t.to_address = '{{wallet_address}}'
ORDER BY t.timestamp DESC
LIMIT {{limit}}
OFFSET {{offset}}