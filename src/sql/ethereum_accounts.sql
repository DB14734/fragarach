-- Ethereum Account Query
-- 
-- Retrieves detailed information about an Ethereum account/address
-- 
-- Parameters:
-- {{address}} - The Ethereum address to query
-- 
-- Returns:
-- - address: The account's Ethereum address
-- - created_timestamp: When the account was first seen on-chain
-- - creator_address: The address that created this account (for contracts)
-- - last_active_timestamp: Last on-chain activity timestamp
-- - type: Account type (EOA/Contract)
SELECT *
FROM ethereum.accounts
WHERE address = '{{address}}'