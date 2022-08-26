## Simple Rust API

<p align="center">Description</p>

**API - has two endpoints**
- to calculate fueal usage per 100KM
- to check probability of unit injector fail

**Endpoints**
 - localhost:8080/calculateDieselUsageForDistance/{distance}/{fuelUsagePer100KM}/{yearOfproduction}
 - localhost:8080/probabilityOfUnitInjectorFail/{VIN}


<p align="center">How to use it</p>

 - Clone this reposotory
 - In cloned repository folder run ***cargo run***
 - Test endpoints in your browser or Postman / Insomia

*If somethnik goes wrong make sure You have installed Rust on your PC and environment path are included*  
[Link to official instalation guide](https://www.rust-lang.org/tools/install)
