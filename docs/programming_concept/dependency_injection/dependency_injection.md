# Dependency Injection


This is about to explain the Dependency Injection (DI) in a simple way. Oviously, it's a simple thing, don't make it complex !!!


### What is DI

* DI means:
    * Give an Object something.


* Just simple like this. For e.g, 
    * An Object does not create it's own Database 
    * We will choose a Database then give it to the Object


### Without DI


* Object creates then hold a Database connection:

![without_di](images/without_di.drawio.svg "without_di")


### With DI


* Pass the Database connection to Object via contruction.

* Or: Use get/set func.

* The goal is: we can flexibly change the type of database.

![with_di](images/with_di.drawio.svg "with_di")



