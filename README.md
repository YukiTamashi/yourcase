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

Adding project images as soon as the frontend is cleaned up a bit.

### What does it do

Data entry and production of weekly reports for payment.
Also will eventually have integration with Whatsapp through the Twillio API to send data to specific whatsapp groups.


<p align="right">(<a href="#readme-top">back to top</a>)</p

## Usage

todo!()

<p align="right">(<a href="#readme-top">back to top</a>)</p>



## Roadmap

- [x] Setup Yew Components
- [x] Make Database Schema and integrate with Diesel
- [x] Setup abstraction for passing data between frontend and Tauri.
- [ ] Finish basic backend logic.
- [ ] Setup AWS instance
    - [ ] Make local database sync up with cloud.
- [ ] Make CSS so frontend looks nice.
- [ ] Replace python scripts by adding the functionally on the application.
- [ ] Integration with Whatsapp API
- [ ] Deploy on Android

See the [open issues](https://github.com/othneildrew/Best-README-Template/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>
