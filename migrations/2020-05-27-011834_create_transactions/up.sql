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
    `parent_id` INTEGER,
    `type` TEXT NOT NULL,
    `icon` TEXT NOT NULL,
    `title` TEXT NOT NULL
);

CREATE TABLE transaction_category (
    `id` INTEGER PRIMARY KEY,
    `transaction_id` INTEGER NOT NULL,
    `category_id` INTEGER NOT NULL
);

-- Seeds
INSERT INTO categories (`id`, `parent_id`, `type`, `icon`, `title`) VALUES
    (  1, NULL, 'Expense', 'bills'               , "Bills & Utilities"    ),
    (  2,    1, 'Expense', 'electricity'         , "Electricity"          ),
    (  3,    1, 'Expense', 'gas'                 , "Gas"                  ),
    (  4,    1, 'Expense', 'internet'            , "Internet"             ),
    (  5,    1, 'Expense', 'phone'               , "Phone"                ),
    (  6,    1, 'Expense', 'rentals'             , "Rentals"              ),
    (  7,    1, 'Expense', 'television'          , "Television"           ),
    (  8,    1, 'Expense', 'water'               , "Water"                ),

    (  9, NULL, 'Expense', 'business'            , "Business"             ),

    ( 10, NULL, 'Expense', 'education'           , "Education"            ),
    ( 11,   10, 'Expense', 'books'               , "Books"                ),
    ( 12,   10, 'Expense', 'research'            , "Research"             ),

    ( 13, NULL, 'Expense', 'entertainment'       , "Entertainment"        ),
    ( 14,   13, 'Expense', 'games'               , "Games"                ),
    ( 15,   13, 'Expense', 'movies'              , "Movies"               ),

    ( 16, NULL, 'Expense', 'family'              , "Family"               ),
    ( 17,   16, 'Expense', 'children'            , "Children & Babies"    ),
    ( 18,   16, 'Expense', 'home_improvement'    , "Home Improvement"     ),
    ( 19,   16, 'Expense', 'home_services'       , "Home Services"        ),
    ( 20,   16, 'Expense', 'pets'                , "Pets"                 ),

    ( 21, NULL, 'Expense', 'fees_charges'        , "Fees & Charges"       ),

    ( 22, NULL, 'Expense', 'food_beverage'       , "Food & Beverage"      ),
    ( 23,   22, 'Expense', 'café'                , "Café"                 ),
    ( 24,   22, 'Expense', 'restaurants'         , "Restaurants"          ),

    ( 25, NULL, 'Expense', 'friends_lover'       , "Friends & Lover"      ),
    ( 26,   25, 'Expense', 'gifts_donations'     , "Gifts & Donations"    ),
    ( 27,   25, 'Expense', 'charity'             , "Charity"              ),
    ( 28,   25, 'Expense', 'funeral'             , "Funeral"              ),
    ( 29,   25, 'Expense', 'marriage'            , "Marriage"             ),

    ( 30, NULL, 'Expense', 'health_fitness'      , "Health & Fitness"     ),
    ( 31,   30, 'Expense', 'doctor'              , "Doctor"               ),
    ( 32,   30, 'Expense', 'personal_care'       , "Personal Care"        ),
    ( 33,   30, 'Expense', 'pharmacy'            , "Pharmacy"             ),
    ( 34,   30, 'Expense', 'sports'              , "Sports"               ),

    ( 35, NULL, 'Expense', 'insurances'          , "Insurances"           ),

    ( 36, NULL, 'Expense', 'investment'          , "Investment"           ),

    ( 37, NULL, 'Expense', 'others'              , "Others"               ),

    ( 38, NULL, 'Expense', 'saving'              , "Saving"               ),

    ( 39, NULL, 'Expense', 'shopping'            , "Shopping"             ),
    ( 40,   39, 'Expense', 'accessories'         , "Accessories"          ),
    ( 41,   39, 'Expense', 'clothing'            , "Clothing"             ),
    ( 42,   39, 'Expense', 'electronics'         , "Electronics"          ),
    ( 43,   39, 'Expense', 'footwear'            , "Footwear"             ),

    ( 44, NULL, 'Expense', 'transportation'      , "Transportation"       ),
    ( 45,   44, 'Expense', 'maintenance'         , "Maintenance"          ),
    ( 46,   44, 'Expense', 'parking_fees'        , "Parking Fees"         ),
    ( 47,   44, 'Expense', 'petrol'              , "Petrol"               ),
    ( 48,   44, 'Expense', 'taxi'                , "Taxi"                 ),
    ( 49,   44, 'Expense', 'travel'              , "Travel"               ),

    ( 50, NULL, 'Expense', 'withdrawal'          , "Withdrawal"           ),

    ( 51, NULL,  'Income', "award"               , "Award"                ),
    ( 52, NULL,  'Income', "gifts"               , "Gifts"                ),
    ( 53, NULL,  'Income', "interest_money"      , "Interest Money"       ),
    ( 54, NULL,  'Income', "others"              , "Others"               ),
    ( 55, NULL,  'Income', "salary"              , "Salary"               ),
    ( 56, NULL,  'Income', "selling"             , "Selling"              )
;
