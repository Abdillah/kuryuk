CREATE TABLE transactions (
    `id` INT NOT NULL PRIMARY KEY,
    `trx_id` VARCHAR(255) NOT NULL,
    `title` VARCHAR(255) NOT NULL,
    `description` VARCHAR(1024) NOT NULL,
    `updated_at` TIMESTAMP NOT NULL,
    `created_at` TIMESTAMP NOT NULL
);

CREATE TABLE categories (
    `id` INT NOT NULL PRIMARY KEY,
    `type` VARCHAR(255) NOT NULL,
    `icon` VARCHAR(255) NOT NULL,
    `title` VARCHAR(255) NOT NULL,
    `description` VARCHAR(1024) NOT NULL
);

CREATE TABLE transaction_category (
    `id` INT NOT NULL PRIMARY KEY,
    `transaction_id` INT NOT NULL,
    `category_id` INT NOT NULL
);