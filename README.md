# mongo-resilient-evolvability-demo

*(by Paul Done - [@TheDonester](https://twitter.com/TheDonester))*

A project to demonstrate some of the best practices for building resilient yet evolvable shared data applications using a flexible data model based database like MongoDB. Based on the [The Six Principles for Building Robust Yet Flexible Shared Data Applications](https://developer.mongodb.com/article/six-principles-building-robust-flexible-shared-data-applications).

For this example, uses a library books scenario where _app1_ is a books inventory management application exposed as a REST API and _app2_ is a books ratings/scores management application exposed as a REST API. These two apps operate on and require different overlapping subsets of attributes about each book. For the sake of simplicity, both apps are part of the same Rust Cargo project with a shared _main_ bootstrap function, but when run each of the two apps listens on a different localhost HTTP port (_8181_ & _8282_ respectively).

## Building The Project

_(ensure you've cloned/copied this GitHub project first to your local machine)_

 1. Install the latest version of the [Rust development environment](https://www.rust-lang.org/tools/install), if it isn't already installed, via the __rustup__ utility, including the _rustc_ compiler & the _cargo_ package/build manager. _NOTE:_ If building on Windows 10, first ensure you have Microsoft's [Build Tools for Visual Studio 2019](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16) installed (and importantly, when running Microsoft's _build tools_ installer, choose the _C++ build tools_ option)

 2. From a terminal/prompt/shell, from this project's root folder, run Rust's _cargo_ command to build the project, as shown below:
 
```console
cargo build
```

## Running The Project

_(ensure you have the URL of an __MongoDB database__ accessible, to enable the reading & writing of records in the database collection __library.books__)_

 1. Load the book data into a MongoDB database using the MongoDB Shell (the example command shown assumes the database is listening on _localhost:27017_):
 
```console
mongosh data/book-data-prep-for-app1.js
```

 2. To run the first application (listening for REST API calls), execute the following command (example URL shown assumes you are running a MongoDB single server unauthenticated database on your local machine listening on _localhost:27017_ - change this URL, containing appropriate credentials, to match the location of your remote MongoDB database):
 
```console
cargo run app1 mongodb://localhost:27017
```

 3. From a browser test the first application's REST API _Get_ operation:
 
 * [http://127.0.0.1:8181/v1/books](http://127.0.0.1:8181/v1/books)
 * [http://127.0.0.1:8181/v1/books?title=The%20Day%20of%20the%20Triffids&author=John%20Wyndham](http://127.0.0.1:8181/v1/books?title=The%20Day%20of%20the%20Triffids&author=John%20Wyndham)


 4. From a new terminal, run the first application's test script (tests retrieving all books + adding/modifying/removing a book via the app's REST API) by executing the following command:
  
```console
tests/test_app1.sh
```

 5. Modify some of the book data to include some review scores in the MongoDB database using the MongoDB Shell (the example command shown assumes the database is listening on _localhost:27017_):
 
```console
mongosh data/book-data-prep-for-app2.js
```
  
 6. To run the second application (listening for REST API calls), keep the existing first application running and in a new terminal execute the following command (change this URL to match the location of your remote MongoDB database):
 
```console
cargo run app2 mongodb://localhost:27017
```

 7. From a browser test the second application's REST API _Get_ operation:
 
 * [http://127.0.0.1:8282/v1/books?title=The%20Last%20Man&author=Mary%20Shelley](http://127.0.0.1:8282/v1/books?title=The%20Last%20Man&author=Mary%20Shelley)
 * [http://127.0.0.1:8282/v1/books?title=The%20Day%20of%20the%20Triffids&author=John%20Wyndham](http://127.0.0.1:8282/v1/books?title=The%20Day%20of%20the%20Triffids&author=John%20Wyndham)


 8. Run the second application's test script (tests adding/modifying/removing some book reviews via the app's REST API) by executing the following command:
  
```console
tests/test_app2.sh
```

