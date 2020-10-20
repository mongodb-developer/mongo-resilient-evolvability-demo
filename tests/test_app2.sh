#!/bin/bash
URL='localhost:8282/v1/books'

printf "\nInitial HTTP GET output:\n"
curl -sS --location --request GET "${URL}?title=The%20Day%20of%20the%20Triffids&author=John%20Wyndham"

printf "\n\nTest no scores exist for the book HTTP GET result:\n"
if curl -sS --location --request GET "${URL}?title=The%20Day%20of%20the%20Triffids&author=John%20Wyndham" | grep "No scores recorded"; then
    printf "====OK: No scores for the book exist\n"
else
    printf "====ERROR: The book has at least one score\n"
    exit 1
fi

printf "\nInsert a score for a book: \n"
curl --location --request POST "${URL}" \
--header 'Content-Type: application/json' \
--data-raw '{
    "title": "The Day of the Triffids",
    "author": "John Wyndham",
    "reference": "The Science Fiction Reviewer",
    "score": 10
}'

printf "\n\nInsert a score for a book: \n"
curl --location --request POST "${URL}" \
--header 'Content-Type: application/json' \
--data-raw '{
    "title": "The Day of the Triffids",
    "author": "John Wyndham",
    "reference": "The Paperback Store",
    "score": 9
}'

printf "\n\nTest average score for book HTTP GET result:\n"
if curl -sS --location --request GET "${URL}?title=The%20Day%20of%20the%20Triffids&author=John%20Wyndham" | grep "9.5"; then
    printf "====OK: Correct average score of 9.5 for the book\n"
else
    printf "====ERROR: The average score for the book should be 9.5 but it is not\n"
    exit 1
fi


printf "\nDelete a specific score by reference for a book: \n"
curl --location --request DELETE "${URL}" \
--header 'Content-Type: application/json' \
--data-raw '{
    "title": "The Day of the Triffids",
    "author": "John Wyndham",
    "reference": "The Science Fiction Reviewer"
}'

printf "\n\nUpdate a score for a book: \n"
curl --location --request PUT "${URL}" \
--header 'Content-Type: application/json' \
--data-raw '{
    "title": "The Day of the Triffids",
    "author": "John Wyndham",
    "reference": "The Paperback Store",
    "score": 8
}'

printf "\n\nTest average score for book HTTP GET result:\n"
if curl -sS --location --request GET "${URL}?title=The%20Day%20of%20the%20Triffids&author=John%20Wyndham" | grep "8.0"; then
    printf "====OK: Correct average score of 8.0 for the book\n"
else
    printf "====ERROR: The average score for the book should be 8 but it is not\n"
    exit 1
fi

printf "\nDelete a specific score by reference for a book: \n"
curl --location --request DELETE "${URL}" \
--header 'Content-Type: application/json' \
--data-raw '{
    "title": "The Day of the Triffids",
    "author": "John Wyndham",
    "reference": "The Paperback Store"
}'

printf "\n\nTest no scores exist for the book HTTP GET result:\n"
if curl -sS --location --request GET "${URL}?title=The%20Day%20of%20the%20Triffids&author=John%20Wyndham" | grep "No scores recorded"; then
    printf "====OK: No scores for the book exist\n"
else
    printf "====ERROR: The book has at least one score\n"
    exit 1
fi


