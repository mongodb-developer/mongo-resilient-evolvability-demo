db = db.getSiblingDB('library');

count = db.books.find({'title': 'The Last Man'}).count();

if (count <= 0) {
    print();
    print("ERROR: The book 'The Last Man' no present in DB. Has the books colleciton been initiaised using the other js script?");
    print();
    quit();
}

var now = new Date();

db.books.updateOne(
    {"title": "The Last Man", "author": "Mary Shelley"},
    {"$push": {scores: {reference: "The Book Club", rating: NumberInt("7")}},
     "$set": {'last_modified': now}}
)

db.books.updateOne(
    {"title": "The Last Man", "author": "Mary Shelley"},
    {"$push": {scores: {reference: "The Good Read", rating: NumberInt("6")}},
     "$set": {'last_modified': now}}
)

db.books.updateOne(
    {"title": "When Worlds Collide", "author": "Philip Wylie & Edwin Balmer"},
    {"$set": {scores: [], 'last_modified': now}}
)

db.books.updateOne(
    {"title": "Earth Abides", "author": "George R. Stewart"},
    {"$push": {scores: {reference: "The Good Read", rating: NumberInt("6")}},
     "$set": {'last_modified': now}}    
)

