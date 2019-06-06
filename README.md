# WebHelper Rocket
**Authors: Adrian Bernat and Edward Koroteev** <br>

WebHelper Rocket is a functional web application created using Rust's Rocket framework.
Taking inspiration from mobile helper programs, such as Siri and Alexa, WebHelper takes
in input from a text field and, based upon that input, can perform a variety of tasks.
Tasks to be implemented:
* Calculator
  * The idea is to make it an integer calculator that accepts a string, i.e. "52+47*3",
    and spits out the correct answer. This is **finished** and supports parenthesis.
* Dictionary API for definitions
  * The idea behind this was to be able to pull up the definitions of certain words using the web helper.
    This ended up working neatly and is fully functional.
* Craigslist Web Scraper
  * The craigslist web scraper takes a search term from the user and compiles a list of the first 10 results from craigslist.
* Easter Eggs!

# Instructions to run
**Since this project runs on the Rust Rocket framework, it requires one to be running Rust Nightly.** This can be done by executing `rustup default nightly`. This will update your nightly version, then set that as default.  
To run this web app, simply type in `cargo run` while in the app directory. It will download dependencies, and once finished, it will launch and instruct you to access `localhost:8000`. Navigate to `localhost:8000` in your preferred web browser to see the web app. Please reference this instruction sheet for usage directions until the instructions are implemented within the web app. **This app assumes a valid internet connection exists.** Many features utilize
web pages and web scraping to return content, and the program will throw a panic if no internet access is present if
it is trying to use internet access.
# Commands
* `calculate <statement>` can be used to perform a calculation. The calculator takes in infix notation and supports parenthesis.
* `craigslist <search term>` can be used to retrieve the first 10 items from a craigslist search result.
* `define <English word>` can be used obtain the definition of any word from oxforddictionaries.com. The command is fully functional and will inform the user if they enter something that is not valid, and catch that error.
* `riddle` can be used to obtain a riddle.
* `ping` can be used to get an annoying output.
* `echo <term` echoes the given term.
* `ip` returns the current IP address of the user.
___
(c) Copyright 2019 Adrian Bernat<br>
Email: abernat@pdx.edu

(c) Copyright 2019 Edward Koroteev<br>
Email: ekoroteev@pdx.edu
