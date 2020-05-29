CREATE TABLE transactions (
    `id` INTEGER PRIMARY KEY,
    `trx_id` TEXT NOT NULL,
    `title` TEXT NOT NULL,
    `description` TEXT NOT NULL,
    `updated_at` TIMESTAMP,
    `created_at` TIMESTAMP
);

CREATE TABLE categories (
    `id` INTEGER PRIMARY KEY,
    `type` TEXT NOT NULL,
    `icon` TEXT NOT NULL,
    `title` TEXT NOT NULL,
    `description` TEXT NOT NULL
);

CREATE TABLE transaction_category (
    `id` INTEGER PRIMARY KEY,
    `transaction_id` INTEGER NOT NULL,
    `category_id` INTEGER NOT NULL
);