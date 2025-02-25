-- Remove the existing pkey on wallet_addresses
DROP TABLE wallet_addresses;

-- Change the name to identity_cache
CREATE TABLE identity_cache (
    inbox_id TEXT NOT NULL,
    identity TEXT NOT NULL,
    identity_kind INT NOT NULL,
    PRIMARY KEY (identity, identity_kind)
);

-- Add a new identity kind (Ethereum, Passkey, Solana, Sui...)
ALTER TABLE consent_records
ADD COLUMN identity_kind INT;

-- Set all the current Identities to Ethereum, since that's all we supported before now
UPDATE consent_records
SET
    identity_kind = 1
WHERE
    consent_type = 3;

-- Add the constraint with syntax that works across different database systems
ALTER TABLE consent_records ADD CHECK (
    (
        consent_type = 3
        AND identity_kind IS NOT NULL
    )
    OR (
        consent_type != 3
        AND identity_kind IS NULL
    )
);
