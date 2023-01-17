use std::{fs, path::PathBuf};

use fantoccini::{elements::Element, Client, ClientBuilder, Locator};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tokio::{
    sync::{Mutex, RwLock},
    time::{sleep, Duration},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub account: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub user: User,
}

impl Config {
    pub fn load(file_path: &PathBuf) -> anyhow::Result<Self> {
        let config_string = fs::read_to_string(file_path)?;
        Ok(toml::from_str(&config_string)?)
    }
}

#[derive(Clone, Debug)]
pub struct CheatBot {
    pub client: Client,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub metal: String,
    pub crystal: String,
    pub deuterium: String,
    pub energy: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Infrastructure {
    pub metal_mine: u8,
    pub crystal_mine: u8,
    pub deuterium_synthesizer: u8,
    pub energy_plant: u8,
    pub fusion_reactor: u8,
    pub solar_satellite: u8,
    pub crawler: u8,
    pub metal_storage: u8,
    pub crystal_storage: u8,
    pub deuterium_tank: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Facility {
    pub robotics_factory: u8,
    pub shipyard: u8,
    pub research_lab: u8,
    pub alliance_depot: u8,
    pub missile_silo: u8,
    pub nanite_factory: u8,
    pub terraformer: u8,
    pub space_dock: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Technology {
    pub energy_technology: u8,
    pub laser_technology: u8,
    pub ion_technology: u8,
    pub hyperspace_technology: u8,
    pub plasma_technology: u8,
    pub combustion_drive: u8,
    pub impulse_drive: u8,
    pub hyperspace_drive: u8,
    pub espionage_technology: u8,
    pub computer_technology: u8,
    pub astrophysics: u8,
    pub intergalactic_research_network: u8,
    pub graviton_technology: u8,
    pub armour_technology: u8,
    pub weapons_technology: u8,
    pub shielding_technology: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Defence {
    pub rocket_launcher: u8,
    pub light_laser: u8,
    pub heavy_laser: u8,
    pub ion_cannon: u8,
    pub gauss_cannon: u8,
    pub plasma_turret: u8,
    pub small_shield_dome: u8,
    pub large_shield_dome: u8,
    pub anti_ballistic_missile: u8,
    pub interplanetary_missile: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Fleet {
    pub light_fighter: u8,
    pub heavy_fighter: u8,
    pub cruiser: u8,
    pub battleship: u8,
    pub battlecruiser: u8,
    pub bomber: u8,
    pub destroyer: u8,
    pub deathstar: u8,
    pub reaper: u8,
    pub pathfinder: u8,
    pub small_cargo_ship: u8,
    pub large_cargo_ship: u8,
    pub colony_ship: u8,
    pub recycler: u8,
    pub espionage_probe: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EmpireOverview {
    overview: Vec<PlanetOverview>,
    technology: Technology,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlanetOverview {
    location: String,
    resource: Resource,
    infrastructure: Infrastructure,
    facility: Facility,
    defence: Defence,
    fleet: Fleet,
}

impl CheatBot {
    /// create a CheaterBot instance
    pub async fn new(web_driver_url: Option<&str>) -> anyhow::Result<Self> {
        let web_driver_url = web_driver_url.unwrap_or("http://localhost:9515");
        let client = ClientBuilder::native().connect(web_driver_url).await?;

        Ok(Self { client })
    }

    /// login the game
    pub async fn login(&self, account: &str, password: &str) -> anyhow::Result<()> {
        // go to the Ogame home page
        self.client
            .goto("https://lobby.ogame.gameforge.com/zh_TW/")
            .await?;

        // select login tab
        let login_tab = self
            .client
            .wait()
            .for_element(Locator::XPath(r#"//ul[@class='tabsList']/li[1]"#))
            .await?;
        login_tab.click().await?;

        // enter account and password
        let account_input = self
            .client
            .wait()
            .for_element(Locator::XPath(r#"//input[@type='email']"#))
            .await?;
        account_input.send_keys(account).await?;

        let password_input = self
            .client
            .wait()
            .for_element(Locator::XPath(r#"//input[@type='password']"#))
            .await?;
        password_input.send_keys(password).await?;

        // click login
        let login = self
            .client
            .wait()
            .for_element(Locator::XPath(r#"//button[@type='submit']"#))
            .await?;
        login.click().await?;

        //HACK wait so that webpage can change register button to last time played button
        // duration depends on processing speed of computer
        sleep(Duration::from_secs(8)).await;

        // click last time played
        let last_time_played = self
            .client
            .wait()
            .for_element(Locator::XPath(r#"//div[@id='joinGame']/button[1]"#))
            .await?;
        last_time_played.click().await?;

        // change current window to new window(game)
        let windows = self.client.windows().await?;
        self.client.switch_to_window(windows[1].clone()).await?;

        Ok(())
    }

    pub async fn overview(&self) -> anyhow::Result<EmpireOverview> {
        let planets = self.get_all_planets().await?;
        let mut overview = Vec::new();

        for planet in planets {
            let location = planet.text().await?.split('\n').collect::<Vec<&str>>()[1].to_owned();
            // go to current location planet page
            planet.click().await?;

            let resource = self.get_resource().await?;
            let infrastructure = self.get_infrastructure_level().await?;
            let facility = self.get_facility_level().await?;
            let defence = self.get_defense_unit_amount().await?;
            let fleet = self.get_fleet_unit_amount().await?;

            overview.push(PlanetOverview {
                location,
                resource,
                infrastructure,
                facility,
                defence,
                fleet,
            })
        }

        let technology = self.get_technology_level().await?;

        Ok(EmpireOverview {
            overview,
            technology,
        })
    }

    /// get all planets
    pub async fn get_all_planets(&self) -> anyhow::Result<Vec<Element>> {
        let planets = self
            .client
            .find_all(Locator::XPath(r#"//div[@id='planetList']/div"#))
            .await?;

        Ok(planets)
    }

    /// get resources from a planet
    pub async fn get_resource(&self) -> anyhow::Result<Resource> {
        let metal = self
            .client
            .wait()
            .for_element(Locator::XPath(r#"//span[@id='resources_metal']"#))
            .await?
            .text()
            .await?;

        let crystal = self
            .client
            .wait()
            .for_element(Locator::XPath(r#"//span[@id='resources_crystal']"#))
            .await?
            .text()
            .await?;

        let deuterium = self
            .client
            .wait()
            .for_element(Locator::XPath(r#"//span[@id='resources_deuterium']"#))
            .await?
            .text()
            .await?;

        let energy = self
            .client
            .wait()
            .for_element(Locator::XPath(r#"//span[@id='resources_energy']"#))
            .await?
            .text()
            .await?;

        Ok(Resource {
            metal,
            crystal,
            deuterium,
            energy,
        })
    }

    /// get infrastructure level
    pub async fn get_infrastructure_level(&self) -> anyhow::Result<Infrastructure> {
        let infrastructure_tab = self
            .client
            .wait()
            .for_element(Locator::XPath(r#"//ul[@id='menuTable']/li[2]"#))
            .await?;
        infrastructure_tab.click().await?;

        let metal_mine = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[1]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let crystal_mine = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[2]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let deuterium_synthesizer = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[3]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let energy_plant = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[4]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let fusion_reactor = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[5]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let solar_satellite = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[6]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let crawler = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[7]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let metal_storage = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[8]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let crystal_storage = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[9]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let deuterium_tank = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[10]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        Ok(Infrastructure {
            metal_mine,
            crystal_mine,
            deuterium_synthesizer,
            energy_plant,
            fusion_reactor,
            solar_satellite,
            crawler,
            metal_storage,
            crystal_storage,
            deuterium_tank,
        })
    }

    /// get facility level
    pub async fn get_facility_level(&self) -> anyhow::Result<Facility> {
        let facility_tab = self
            .client
            .wait()
            .for_element(Locator::XPath(r#"//ul[@id='menuTable']/li[4]"#))
            .await?;
        facility_tab.click().await?;

        let robotics_factory = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[1]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let shipyard = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[2]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let research_lab = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[3]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let alliance_depot = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[4]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let missile_silo = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[5]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let nanite_factory = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[6]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let terraformer = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[7]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let space_dock = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[8]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        Ok(Facility {
            robotics_factory,
            shipyard,
            research_lab,
            alliance_depot,
            missile_silo,
            nanite_factory,
            terraformer,
            space_dock,
        })
    }

    /// get technology level
    pub async fn get_technology_level(&self) -> anyhow::Result<Technology> {
        let technology_tab = self
            .client
            .wait()
            .for_element(Locator::XPath(r#"//ul[@id='menuTable']/li[6]"#))
            .await?;
        technology_tab.click().await?;

        // basic technologies
        let energy_technology = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_basic']/ul/li[1]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let laser_technology = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_basic']/ul/li[2]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let ion_technology = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_basic']/ul/li[3]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let hyperspace_technology = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_basic']/ul/li[4]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let plasma_technology = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_basic']/ul/li[5]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        // drive technologies
        let combustion_drive = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_drive']/ul/li[1]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let impulse_drive = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_drive']/ul/li[2]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let hyperspace_drive = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_drive']/ul/li[3]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        // advanced technologies
        let espionage_technology = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_advanced']/ul/li[1]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let computer_technology = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_advanced']/ul/li[2]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let astrophysics = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_advanced']/ul/li[3]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let intergalactic_research_network = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_advanced']/ul/li[4]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let graviton_technology = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_advanced']/ul/li[5]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        // combat technologies
        let armour_technology = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_combat']/ul/li[1]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let weapons_technology = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_combat']/ul/li[2]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let shielding_technology = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_combat']/ul/li[3]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        Ok(Technology {
            energy_technology,
            laser_technology,
            ion_technology,
            hyperspace_technology,
            plasma_technology,
            combustion_drive,
            impulse_drive,
            hyperspace_drive,
            espionage_technology,
            computer_technology,
            astrophysics,
            intergalactic_research_network,
            graviton_technology,
            armour_technology,
            weapons_technology,
            shielding_technology,
        })
    }

    /// get defence unit amount
    pub async fn get_defense_unit_amount(&self) -> anyhow::Result<Defence> {
        let defense_tab = self
            .client
            .wait()
            .for_element(Locator::XPath(r#"//ul[@id='menuTable']/li[8]"#))
            .await?;
        defense_tab.click().await?;

        let rocket_launcher = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[1]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let light_laser = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[2]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let heavy_laser = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[3]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let ion_cannon = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[4]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let gauss_cannon = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[5]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let plasma_turret = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[6]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let small_shield_dome = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[7]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let large_shield_dome = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[8]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let anti_ballistic_missile = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[9]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let interplanetary_missile = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[10]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        Ok(Defence {
            rocket_launcher,
            light_laser,
            heavy_laser,
            ion_cannon,
            gauss_cannon,
            plasma_turret,
            small_shield_dome,
            large_shield_dome,
            anti_ballistic_missile,
            interplanetary_missile,
        })
    }

    /// get fleet unit amount
    pub async fn get_fleet_unit_amount(&self) -> anyhow::Result<Fleet> {
        let fleet_tab = self
            .client
            .wait()
            .for_element(Locator::XPath(r#"//ul[@id='menuTable']/li[9]"#))
            .await?;
        fleet_tab.click().await?;

        // battleships
        let light_fighter = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='battleships']/ul[1]/li[1]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let heavy_fighter = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='battleships']/ul[1]/li[2]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let cruiser = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='battleships']/ul[1]/li[3]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let battleship = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='battleships']/ul[1]/li[4]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let battlecruiser = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='battleships']/ul/li[5]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let bomber = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='battleships']/ul[1]/li[6]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let destroyer = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='battleships']/ul[1]/li[7]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let deathstar = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='battleships']/ul[1]/li[8]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let reaper = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='battleships']/ul[1]/li[9]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let pathfinder = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='battleships']/ul[1]/li[10]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        // civilships
        let small_cargo_ship = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='civilships']/ul[1]/li[1]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let large_cargo_ship = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='civilships']/ul[1]/li[2]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let colony_ship = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='civilships']/ul[1]/li[3]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let recycler = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='civilships']/ul[1]/li[4]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        let espionage_probe = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='civilships']/ul[1]/li[5]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u8>()?;

        // let solar_satellite = self
        //     .client
        //     .wait()
        //     .for_element(Locator::XPath(
        //         r#"//div[@id='civilships']/ul[1]/li[6]//span[@class='amount']"#,
        //     ))
        //     .await?
        //     .text()
        //     .await?
        //     .parse::<u8>()?;

        // let crawler = self
        //     .client
        //     .wait()
        //     .for_element(Locator::XPath(
        //         r#"//div[@id='civilships']/ul[1]/li[7]//span[@class='amount']"#,
        //     ))
        //     .await?
        //     .text()
        //     .await?
        //     .parse::<u8>()?;

        Ok(Fleet {
            light_fighter,
            heavy_fighter,
            cruiser,
            battleship,
            battlecruiser,
            bomber,
            destroyer,
            deathstar,
            reaper,
            pathfinder,
            small_cargo_ship,
            large_cargo_ship,
            colony_ship,
            recycler,
            espionage_probe,
        })
    }
}
