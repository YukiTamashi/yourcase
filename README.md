<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a name="readme-top"></a>


<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#license">License</a></li>
  </ol>
</details>

## About The Project

Full rewrite from application for current job, previously made with Apps Scripts and Google Sheets.
Developed with Tauri and Yew for deployment on desktop and Android.
It uses Diesel to manage an SQLite database, with goal of setting up syncing with AWS RDS.

# What does it do

Mostly a CRUD application to keep track of transactions and process totals for weekly payments.


<p align="right">(<a href="#readme-top">back to top</a>)</p

## Usage

todo!()

<p align="right">(<a href="#readme-top">back to top</a>)</p>



## Roadmap

- [x] Setup Yew Components
- [x] Make Database Schema and integrate with Diesel
- [ ] Setup abstraction for passing data between frontend and Tauri.
- [ ] Finish basic backend logic.
- [ ] Setup AWS instance
    - [ ] Make local database sync up with cloud.
- [ ] Replace python scripts by adding the functionally on the application.
- [ ] Integration with Whatsapp API
- [ ] Deploy on Android

See the [open issues](https://github.com/othneildrew/Best-README-Template/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>
