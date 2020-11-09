# mongo-resilient-evolvability-demo

A project to demonstrate some of the best practices for building resilient yet evolvable shared data applications using a flexible data model based database like MongoDB. 

For this example, uses a library books scenario where _app1_ is a books inventory manager exposed as a REST API and _app2_ is a books ratings/scores manager expsoed as a REST API. For the sake of simiplicitry, both apps are part of the same Rust Cargo project with a shared _main_ boostrap function, but when run each of the two apps listens on a different localhost HTTP port (_8181_ & _8282_ respectively).

## Building The Project

_(ensure you've cloned/copied this GitHub project first to your local machine)_

 1. Install the latest version of the [Rust development environment](https://www.rust-lang.org/tools/install), if it isn't already installed, via the __rustup__ utility, including the _rustc_ compiler & the _cargo_ package/build manager. _NOTE:_ If building on Windows 10, first ensure you have Microsoft's [Build Tools for Visual Studio 2019](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16) installed (and importantly, when running Micosrosoft's _build tools_ installer, choose the _C++ build tools_ option)

 2. From a terminal/prompt/shell, from this project's root folder, run Rust's _cargo_ command to build the project, as shown below:
 
```console
cargo build
```

## Running The Project

_(ensure you have the URL of an __MongoDB database__ accessible, to enable the reading & writing of records in the database collection __library.books__)_

 1. To run the first application, execute the following command (example URL shown assumes you are running a MongoDB single server unauthenticated database on your local machine listening on _localhost:27017_ - change this URL, containing appropriate credentials, to match the location of your remote MongoDB database):
 
```console
cargo run app1 mongodb://localcdhost:27017
```

 2. From a browser test the first application's REST API _Get_ operation:
 
 * [http://127.0.0.1:8181/v1/books](http://127.0.0.1:8181/v1/books)
 * [http://127.0.0.1:8181/v1/books?title=The%20Day%20of%20the%20Triffids&author=John%20Wyndham](http://127.0.0.1:8181/v1/books?title=The%20Day%20of%20the%20Triffids&author=John%20Wyndham)


 3. From a new terminal, run the first application's test script by executing the following command:
  
```console
tests/test_app1.sh
```
  
 4. To run the second application, keep the existing first application running and in a new terminal execute the following command (change this URL to match the location of your remote MongoDB database):
 
```console
cargo run app2 mongodb://localhost:27017
```

 5. From a browser test the second application's REST API _Get_ operation:
 
 * [http://127.0.0.1:8282/v1/books?title=The%20Last%20Man&author=Mary%20Shelley](http://127.0.0.1:8282/v1/books?title=The%20Last%20Man&author=Mary%20Shelley)
 * [http://127.0.0.1:8282/v1/books?title=The%20Day%20of%20the%20Triffids&author=John%20Wyndham](http://127.0.0.1:8282/v1/books?title=The%20Day%20of%20the%20Triffids&author=John%20Wyndham)



 6. Run the second application's test script by executing the following command:
  
```console
tests/test_app2.sh
```

