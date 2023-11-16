ALTER TABLE shop RENAME TO wallet;
ALTER TABLE shop_owner RENAME TO wallet_owner;

ALTER TABLE wallet_owner RENAME COLUMN shop_id TO wallet_id;
