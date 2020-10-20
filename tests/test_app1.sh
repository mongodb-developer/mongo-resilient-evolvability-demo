#!/bin/bash
URL='localhost:8181/v1/books'

printf "\nInitial HTTP GET output:\n"
curl -sS --location --request GET "${URL}"

printf "\n\nTest new book does not exist HTTP GET result:\n"
if ! curl -sS --location --request GET "${URL}?title=Bad%20Book&author=Bad%20Writer" | grep "Bad Book"; then
    printf "====OK: New book does not exist\n"
else
    printf "====ERROR: New book exists\n"
    exit 1
fi

printf "\nAdd new book with amount HTTP POST output:\n"
curl -sS --location --request POST "${URL}" \
--header 'Content-Type: application/json' \
--data-raw '{
    "title": "Bad Book",
    "author": "Bad Writer",
    "year": 2020,
    "quantity": 3
}'

printf "\n\nTest new book exists HTTP GET result:\n"
if ! curl -sS --location --request GET "${URL}?title=Bad%20Book&author=Bad%20Writer" | grep "Bad Book"; then
    printf "====ERROR: No new book exists\n"
    exit 1
else
    printf "====OK: New book exists\n"
fi


printf "\nUpdate new book quantity HTTP PUT output:\n"
curl -sS --location --request PUT "${URL}" \
--header 'Content-Type: application/json' \
--data-raw '{
    "title": "Bad Book",
    "author": "Bad Writer",
    "quantity": 5
}'

printf "\n\nTest new book still exists exist HTTP GET result:\n"
if ! curl -sS --location --request GET "${URL}?title=Bad%20Book&author=Bad%20Writer" | grep "Bad Book"; then
    printf "====ERROR: New book does not exist\n"
    exit 1
else
    printf "====OK: New book exists\n"
fi


printf "\nDelete new book HTTP DELETE output: \n"
curl --location --request DELETE "${URL}" \
--header 'Content-Type: application/json' \
--data-raw '{
    "title": "Bad Book",
    "author": "Bad Writer"
}'


printf "\n\nTest new book no longer exists HTTP GET result:\n"
if ! curl -sS --location --request GET "${URL}?title=Bad%20Book&author=Bad%20Writer" | grep "Bad Book"; then
    printf "====OK: New book no longer exists\n"
else
    printf "====ERROR: New book still exists\n"
    exit 1
fi

printf "\n"

