use std::{convert::TryFrom, fs, path::PathBuf};

use anyhow::anyhow;
use chrono::{DateTime, Local, TimeZone, Utc};
use fantoccini::{elements::Element, Client, ClientBuilder, Locator};
use rand::Rng;
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
    pub planets_info: PlanetsInfo,
}

impl Config {
    pub fn load(file_path: &PathBuf) -> anyhow::Result<Self> {
        let config_string = fs::read_to_string(file_path)?;
        Ok(toml::from_str(&config_string)?)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlanetsInfo {
    planets: Vec<PlanetId>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlanetId {
    planet_id: String,
    lunar_id: Option<String>,
}

#[derive(Clone, Debug)]
pub struct CheatBot {
    pub client: Client,
    pub planets_info: PlanetsInfo,
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
    pub metal_mine: u32,
    pub crystal_mine: u32,
    pub deuterium_synthesizer: u32,
    pub energy_plant: u32,
    pub fusion_reactor: u32,
    pub solar_satellite: u32,
    pub crawler: u32,
    pub metal_storage: u32,
    pub crystal_storage: u32,
    pub deuterium_tank: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlanetFacility {
    pub robotics_factory: u32,
    pub shipyard: u32,
    pub research_lab: u32,
    pub alliance_depot: u32,
    pub missile_silo: u32,
    pub nanite_factory: u32,
    pub terraformer: u32,
    pub space_dock: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LunarFacility {
    pub robotics_factory: u32,
    pub shipyard: u32,
    pub lunar_base: u32,
    pub sensor_phalanx: u32,
    pub jump_gate: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Technology {
    pub energy_technology: u32,
    pub laser_technology: u32,
    pub ion_technology: u32,
    pub hyperspace_technology: u32,
    pub plasma_technology: u32,
    pub combustion_drive: u32,
    pub impulse_drive: u32,
    pub hyperspace_drive: u32,
    // pub espionage_technology: u32,
    pub espionage_technology: String,
    pub computer_technology: u32,
    pub astrophysics: u32,
    pub intergalactic_research_network: u32,
    pub graviton_technology: u32,
    pub armour_technology: u32,
    pub weapons_technology: u32,
    pub shielding_technology: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Defence {
    pub rocket_launcher: u32,
    pub light_laser: u32,
    pub heavy_laser: u32,
    pub ion_cannon: u32,
    pub gauss_cannon: u32,
    pub plasma_turret: u32,
    pub small_shield_dome: u32,
    pub large_shield_dome: u32,
    pub anti_ballistic_missile: u32,
    pub interplanetary_missile: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Fleet {
    pub light_fighter: u32,
    pub heavy_fighter: u32,
    pub cruiser: u32,
    pub battleship: u32,
    pub battlecruiser: u32,
    pub bomber: u32,
    pub destroyer: u32,
    pub deathstar: u32,
    pub reaper: u32,
    pub pathfinder: u32,
    pub small_cargo_ship: u32,
    pub large_cargo_ship: u32,
    pub colony_ship: u32,
    pub recycler: u32,
    pub espionage_probe: u32,
}

impl Fleet {
    pub fn is_zero(&self) -> bool {
        let sum = self.light_fighter
            + self.heavy_fighter
            + self.cruiser
            + self.battleship
            + self.battlecruiser
            + self.bomber
            + self.destroyer
            + self.deathstar
            + self.reaper
            + self.pathfinder
            + self.small_cargo_ship
            + self.large_cargo_ship
            + self.colony_ship
            + self.recycler
            + self.espionage_probe;

        sum == 0
    }

    pub fn is_not_zero(&self) -> bool {
        let sum = self.light_fighter
            + self.heavy_fighter
            + self.cruiser
            + self.battleship
            + self.battlecruiser
            + self.bomber
            + self.destroyer
            + self.deathstar
            + self.reaper
            + self.pathfinder
            + self.small_cargo_ship
            + self.large_cargo_ship
            + self.colony_ship
            + self.recycler
            + self.espionage_probe;

        sum != 0
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EmpireOverview {
    pub overview: Vec<PlanetOverview>,
    technology: Technology,
    pub maybe_fleet_events: Option<Vec<FleetEvent>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlanetOverview {
    pub id: String,
    pub location: String,
    resource: Resource,
    infrastructure: Infrastructure,
    facility: PlanetFacility,
    defence: Defence,
    pub fleet: Fleet,
    lunar: Option<Lunar>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Lunar {
    id: String,
    location: String,
    resource: Resource,
    facility: LunarFacility,
    pub fleet: Fleet,
}

impl CheatBot {
    /// create a CheaterBot instance
    pub async fn new(
        web_driver_url: Option<&str>,
        planets_info: PlanetsInfo,
    ) -> anyhow::Result<Self> {
        let web_driver_url = web_driver_url.unwrap_or("http://localhost:9515");
        let client = ClientBuilder::native().connect(web_driver_url).await?;

        Ok(Self {
            client,
            planets_info,
        })
    }

    pub async fn start(&self, account: &str, password: &str) -> anyhow::Result<()> {
        self.login(account, password).await?;

        loop {
            let expiration = Self::calculate_expiration()?;
            log::info!(
                "refreshing game state... {}",
                Local::now().format("%Y/%m/%d %H:%M:%S")
            );
            let empire_overview = self.empire_overview().await?;
            log::info!("empire_overview {:#?}", empire_overview);
            // check if is being attack, and do fs
            if let Some(fleet_events) = empire_overview.maybe_fleet_events {
                for event in fleet_events.iter() {
                    if event.mission_type == MissionType::EnemyAttacking {
                        log::warn!(
                            "{} is being attacked by {}",
                            event.dest_coords,
                            event.coords_origin
                        );

                        let planet = empire_overview
                            .overview
                            .iter()
                            .find(|planet| planet.location == event.dest_coords)
                            .ok_or_else(|| anyhow::anyhow!("no planet or lunar match"))?;

                        // check if fleet still on planet, active fleet saveing
                        if planet.fleet.is_not_zero() {
                            self.fleet_saving(&planet.id).await?;
                        }
                    }
                }
            }
            log::info!(
                "next refresh time: {}",
                expiration.with_timezone(&Local).format("%Y/%m/%d %H:%M:%S")
            );

            // delay until expiration for refreshing game state
            sleep(Duration::from_millis(
                (expiration.timestamp_millis() - Utc::now().timestamp_millis()) as u64,
            ))
            .await;
        }
    }

    fn calculate_expiration() -> anyhow::Result<DateTime<Utc>> {
        // 15 minutes
        let refresh_rate = (REFRESH_RATE * MINUTE) as f32;

        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0.0..1.0);

        let expiration_period = random_number * (refresh_rate / 2.0f32).floor();

        // random refresh page 8.25 ~ 21.75 mins per time
        let time_until_expiration = if random_number < 0.5 {
            (refresh_rate - expiration_period) as i64
        } else {
            (refresh_rate + expiration_period) as i64
        };

        let expired_time = Utc::now().timestamp_millis() + time_until_expiration;
        let Some(expired_time) = Utc.timestamp_millis_opt(expired_time).single() else {return Err(anyhow!("calculate expiration time error"))};

        Ok(expired_time)
    }

    /// login the game
    pub async fn login(&self, account: &str, password: &str) -> anyhow::Result<()> {
        // go to the Ogame home page
        self.client
            .goto("https://lobby.ogame.gameforge.com/zh_TW/")
            .await?;

        // click accept cookie button so that modal will not block content
        let cookie_modal = self
            .client
            .wait()
            .for_element(Locator::XPath(r#"//button[@class='cookiebanner5']"#))
            .await?;
        cookie_modal.click().await?;

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
        sleep(Duration::from_secs(10)).await;

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

    pub async fn resume_play(&self) -> anyhow::Result<()> {
        // go to the Ogame home page
        self.client
            .goto("https://lobby.ogame.gameforge.com/zh_TW/")
            .await?;

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

    /// parse all inforamtion from empire
    pub async fn empire_overview(&self) -> anyhow::Result<EmpireOverview> {
        let mut overview = Vec::new();

        for planet in self.planets_info.planets.iter() {
            let planet_overview = self.parse_planet(planet).await?;
            overview.push(planet_overview);
        }

        let technology = self.get_technology_level().await?;
        let maybe_fleet_events = self.get_fleet_events().await?;

        Ok(EmpireOverview {
            overview,
            technology,
            maybe_fleet_events,
        })
    }

    pub async fn parse_planet(&self, planet: &PlanetId) -> anyhow::Result<PlanetOverview> {
        // go to the current planet overview
        let url = format!("https://s144-tw.ogame.gameforge.com/game/index.php?page=ingame&component=overview&cp={}", planet.planet_id);
        self.client.goto(&url).await?;

        // wait data response from server
        sleep(Duration::from_secs(2)).await;
        // get location
        let location = self
            .client
            .wait()
            .for_element(Locator::XPath(r#"//span[@id='positionContentField']"#))
            .await?
            .text()
            .await?;

        // get planet data
        let resource = self.get_resource().await?;
        let infrastructure = self.get_infrastructure_level().await?;
        let facility = self.get_planet_facility_level().await?;
        let defence = self.get_defense_unit_amount().await?;
        let fleet = self.get_fleet_unit_amount().await?;

        // get lunar data
        let lunar = self.parse_lunar(&planet.lunar_id).await?;

        Ok(PlanetOverview {
            id: planet.planet_id.clone(),
            location,
            resource,
            infrastructure,
            facility,
            defence,
            fleet,
            lunar,
        })
    }

    pub async fn parse_lunar(&self, lunar: &Option<String>) -> anyhow::Result<Option<Lunar>> {
        match lunar {
            Some(id) => {
                // go to the current lunar overview
                let url = format!("https://s144-tw.ogame.gameforge.com/game/index.php?page=ingame&component=overview&cp={}", id);
                self.client.goto(&url).await?;

                // wait data response from server
                sleep(Duration::from_secs(2)).await;
                // get location
                let location = self
                    .client
                    .wait()
                    .for_element(Locator::XPath(r#"//span[@id='positionContentField']"#))
                    .await?
                    .text()
                    .await?;

                let resource = self.get_resource().await?;
                let facility = self.get_lunar_facility_level().await?;
                let fleet = self.get_fleet_unit_amount().await?;

                Ok(Some(Lunar {
                    id: id.to_owned(),
                    location,
                    resource,
                    facility,
                    fleet,
                }))
            }
            None => Ok(None),
        }
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
            .parse::<u32>()?;

        let crystal_mine = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[2]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        let deuterium_synthesizer = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[3]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        let energy_plant = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[4]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        let fusion_reactor = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[5]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        let solar_satellite = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[6]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        let crawler = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[7]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        let metal_storage = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[8]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        let crystal_storage = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[9]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        let deuterium_tank = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[10]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

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

    /// get planet facility level
    pub async fn get_planet_facility_level(&self) -> anyhow::Result<PlanetFacility> {
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
            .parse::<u32>()?;

        let shipyard = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[2]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        let research_lab = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[3]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        let alliance_depot = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[4]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        let missile_silo = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[5]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        let nanite_factory = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[6]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        let terraformer = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[7]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        let space_dock = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[8]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        Ok(PlanetFacility {
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

    /// get lunar facility level
    pub async fn get_lunar_facility_level(&self) -> anyhow::Result<LunarFacility> {
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
            .parse::<u32>()?;

        let shipyard = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[2]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        let lunar_base = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[3]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        let sensor_phalanx = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[4]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        let jump_gate = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[5]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        Ok(LunarFacility {
            robotics_factory,
            shipyard,
            lunar_base,
            sensor_phalanx,
            jump_gate,
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
            .parse::<u32>()?;

        let laser_technology = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_basic']/ul/li[2]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        let ion_technology = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_basic']/ul/li[3]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        let hyperspace_technology = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_basic']/ul/li[4]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        let plasma_technology = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_basic']/ul/li[5]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

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
            .parse::<u32>()?;

        let impulse_drive = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_drive']/ul/li[2]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        let hyperspace_drive = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_drive']/ul/li[3]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        // advanced technologies
        let espionage_technology = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_advanced']/ul/li[1]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?;
        // .parse::<u32>()?;

        let computer_technology = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_advanced']/ul/li[2]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        let astrophysics = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_advanced']/ul/li[3]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        let intergalactic_research_network = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_advanced']/ul/li[4]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        let graviton_technology = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_advanced']/ul/li[5]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

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
            .parse::<u32>()?;

        let weapons_technology = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_combat']/ul/li[2]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        let shielding_technology = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies_combat']/ul/li[3]//span[@class='level']"#,
            ))
            .await?
            .text()
            .await?
            .parse::<u32>()?;

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
            .replace(",", "")
            .parse::<u32>()?;

        let light_laser = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[2]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .replace(",", "")
            .parse::<u32>()?;

        let heavy_laser = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[3]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .replace(",", "")
            .parse::<u32>()?;

        let ion_cannon = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[4]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .replace(",", "")
            .parse::<u32>()?;

        let gauss_cannon = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[5]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .replace(",", "")
            .parse::<u32>()?;

        let plasma_turret = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[6]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .replace(",", "")
            .parse::<u32>()?;

        let small_shield_dome = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[7]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .replace(",", "")
            .parse::<u32>()?;

        let large_shield_dome = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[8]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .replace(",", "")
            .parse::<u32>()?;

        let anti_ballistic_missile = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[9]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .replace(",", "")
            .parse::<u32>()?;

        let interplanetary_missile = self
            .client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='technologies']/ul/li[10]//span[@class='amount']"#,
            ))
            .await?
            .text()
            .await?
            .replace(",", "")
            .parse::<u32>()?;

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

        // handle no fleet
        if self
            .client
            .find_all(Locator::XPath(r#"//div[@id='fleet1']/div/div"#))
            .await?
            .len()
            == 6
        {
            return Ok(Fleet::default());
        };

        let battleships = self
            .client
            .find_all(Locator::XPath(r#"//div[@id='battleships']/ul/li"#))
            .await?;

        let light_fighter = battleships[0]
            .text()
            .await?
            .replace(",", "")
            .parse::<u32>()?;

        let heavy_fighter = battleships[1]
            .text()
            .await?
            .replace(",", "")
            .parse::<u32>()?;

        let cruiser = battleships[2]
            .text()
            .await?
            .replace(",", "")
            .parse::<u32>()?;

        let battleship = battleships[3]
            .text()
            .await?
            .replace(",", "")
            .parse::<u32>()?;

        let battlecruiser = battleships[4]
            .text()
            .await?
            .replace(",", "")
            .parse::<u32>()?;

        let bomber = battleships[5]
            .text()
            .await?
            .replace(",", "")
            .parse::<u32>()?;

        let destroyer = battleships[6]
            .text()
            .await?
            .replace(",", "")
            .parse::<u32>()?;

        let deathstar = battleships[7]
            .text()
            .await?
            .replace(",", "")
            .parse::<u32>()?;

        let reaper = battleships[8]
            .text()
            .await?
            .replace(",", "")
            .parse::<u32>()?;

        let pathfinder = battleships[9]
            .text()
            .await?
            .replace(",", "")
            .parse::<u32>()?;

        // // civilships
        let civilships = self
            .client
            .find_all(Locator::XPath(r#"//div[@id='civilships']/ul/li"#))
            .await?;

        let small_cargo_ship = civilships[0]
            .text()
            .await?
            .replace(",", "")
            .parse::<u32>()?;

        let large_cargo_ship = civilships[1]
            .text()
            .await?
            .replace(",", "")
            .parse::<u32>()?;

        let colony_ship = civilships[2]
            .text()
            .await?
            .replace(",", "")
            .parse::<u32>()?;

        let recycler = civilships[3]
            .text()
            .await?
            .replace(",", "")
            .parse::<u32>()?;

        let espionage_probe = civilships[4]
            .text()
            .await?
            .replace(",", "")
            .parse::<u32>()?;

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

    pub async fn get_fleet_events(&self) -> anyhow::Result<Option<Vec<FleetEvent>>> {
        // wait for page loading
        sleep(Duration::from_secs(1)).await;
        // trigger drop for fetching data
        let event_drop_down = self
            .client
            .wait()
            .for_element(Locator::XPath(r#"//a[@id='js_eventDetailsClosed']"#))
            .await?;

        if let Err(e) = event_drop_down.click().await {
            log::info!("{}, might not have any events", e);
            return Ok(None);
        }

        let event_content = self
            .client
            .wait()
            .for_element(Locator::XPath(r#"//table[@id='eventContent']/tbody"#))
            .await?;

        let events = event_content.find_all(Locator::XPath(r#"tr"#)).await?;

        let mut fleet_events = Vec::new();

        for event in events {
            let Some(mission) = event
                .find(Locator::XPath(r#"td[@class='missionFleet']/img"#))
                .await?
                .attr("title")
                .await? else { return Err(anyhow::anyhow!("parse mission type error"))};

            let arrival_time = event
                .find(Locator::XPath(r#"td[@class='arrivalTime']"#))
                .await?
                .text()
                .await?;

            let coords_origin = event
                .find(Locator::XPath(r#"td[@class='coordsOrigin']"#))
                .await?
                .text()
                .await?;

            let dest_coords = event
                .find(Locator::XPath(r#"td[@class='destCoords']"#))
                .await?
                .text()
                .await?;

            fleet_events.push(FleetEvent {
                mission_type: MissionType::try_from(mission)?,
                arrival_time,
                coords_origin,
                dest_coords,
            });
        }

        // close drop down
        let event_drop_down = self
            .client
            .wait()
            .for_element(Locator::XPath(r#"//a[@id='js_eventDetailsOpen']"#))
            .await?;
        event_drop_down.click().await?;

        Ok(Some(fleet_events))
    }

    pub async fn fleet_saving(&self, id: &str) -> anyhow::Result<()> {
        let url = format!("https://s144-tw.ogame.gameforge.com/game/index.php?page=ingame&component=fleetdispatch&cp={}",id);
        self.client.goto(&url).await?;

        // select all fleets
        self.client
            .wait()
            .for_element(Locator::XPath(r#"//span[@class='send_all']/a"#))
            .await?
            .click()
            .await?;

        // next step
        self.client
            .wait()
            .for_element(Locator::XPath(r#"//a[@id='continueToFleet2']/span"#))
            .await?
            .click()
            .await?;

        sleep(Duration::from_secs(3)).await;

        // enter coords
        self.client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@class='coords']//input[@id='position']"#,
            ))
            .await?
            .send_keys("16")
            .await?;

        // select expedition
        self.client
            .wait()
            .for_element(Locator::XPath(
                r#"//ul[@id='missions']//li[@id='button15']/a"#,
            ))
            .await?
            .click()
            .await?;

        // select 10% speed
        self.client
            .wait()
            .for_element(Locator::XPath(r#"//div[@class='steps']/div[1]"#))
            .await?
            .click()
            .await?;

        // load all resources
        self.client
            .wait()
            .for_element(Locator::XPath(r#"//div[@id='loadAllResources']/a"#))
            .await?
            .click()
            .await?;

        // dispatch fleets
        self.client
            .wait()
            .for_element(Locator::XPath(
                r#"//div[@id='naviActions']//a[@id='sendFleet']"#,
            ))
            .await?
            .click()
            .await?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FleetEvent {
    pub mission_type: MissionType,
    pub arrival_time: String,
    pub coords_origin: String,
    pub dest_coords: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum MissionType {
    // Dispatch
    // Self
    Expedition,
    Colonization,
    Harvesting,
    Transport,
    Deployment,
    Espionage,
    ACSDefend,
    Attacking,
    ACSAttack,
    Destroy,
    SearchingForLifeforms,
    // Return
    ExpeditionReturn,
    ColonizationReturn,
    HarvestingReturn,
    TransportReturn,
    DeploymentReturn,
    EspionageReturn,
    ACSDefendReturn,
    AttackingReturn,
    ACSAttackReturn,
    DestroyReturn,
    SearchingForLifeformsReturn,
    // Ally
    FriendlyTransport,
    FriendlyACSDefend,
    // Enemy
    EnemyEspionage,
    EnemyAttacking,
}

impl TryFrom<String> for MissionType {
    type Error = anyhow::Error;

    fn try_from(str: String) -> Result<Self, Self::Error> {
        match str.as_str() {
            // Self Dispatch
            EXPEDITION => Ok(MissionType::Expedition),
            COLONIZATION => Ok(MissionType::Colonization),
            HARVESTING => Ok(MissionType::Harvesting),
            TRANSPORT => Ok(MissionType::Transport),
            DEPLOYMENT => Ok(MissionType::Deployment),
            ESPIONAGE => Ok(MissionType::Espionage),
            ACS_DEFEND => Ok(MissionType::ACSDefend),
            ATTACKING => Ok(MissionType::Attacking),
            ACS_ATTACK => Ok(MissionType::ACSAttack),
            DESTROY => Ok(MissionType::Destroy),
            SEARCHING_FOR_LIFEFORMS => Ok(MissionType::SearchingForLifeforms),
            // Self Return
            EXPEDITION_RETURN => Ok(MissionType::ExpeditionReturn),
            COLONIZATION_RETURN => Ok(MissionType::ColonizationReturn),
            HARVESTING_RETURN => Ok(MissionType::HarvestingReturn),
            TRANSPORT_RETURN => Ok(MissionType::TransportReturn),
            DEPLOYMENT_RETURN => Ok(MissionType::DeploymentReturn),
            ESPIONAGE_RETURN => Ok(MissionType::EspionageReturn),
            ACS_DEFEND_RETURN => Ok(MissionType::ACSDefendReturn),
            ATTACKING_RETURN => Ok(MissionType::AttackingReturn),
            ACS_ATTACK_RETURN => Ok(MissionType::ACSAttackReturn),
            DESTROY_RETURN => Ok(MissionType::DestroyReturn),
            SEARCHING_FOR_LIFEFORMS_RETURN => Ok(MissionType::SearchingForLifeformsReturn),
            // Ally
            FRIENDLY_TRANSPORT => Ok(MissionType::FriendlyTransport),
            FRIENDLY_ACS_DEFEND => Ok(MissionType::FriendlyACSDefend),
            // Enemy
            ENEMY_ESPIONAGE => Ok(MissionType::EnemyEspionage),
            ENEMY_ATTACKING => Ok(MissionType::EnemyAttacking),

            _ => Err(anyhow::anyhow!("unknown fleet event")),
        }
    }
}

/// action
pub const EXPEDITION: &str = "己方艦隊 | 遠征探險";
pub const EXPEDITION_RETURN: &str = "己方艦隊 | 遠征探險 (返)";
pub const COLONIZATION: &str = "己方艦隊 | 殖民";
pub const COLONIZATION_RETURN: &str = "己方艦隊 | 殖民 (返)";
pub const HARVESTING: &str = "己方艦隊 | 採集回收";
pub const HARVESTING_RETURN: &str = "己方艦隊 | 採集回收 (返)";
pub const TRANSPORT: &str = "己方艦隊 | 運輸";
pub const TRANSPORT_RETURN: &str = "己方艦隊 | 運輸 (返)";
pub const DEPLOYMENT: &str = "己方艦隊 | 部署";
pub const DEPLOYMENT_RETURN: &str = "己方艦隊 | 部署 (返)";
pub const ESPIONAGE: &str = "己方艦隊 | 間諜偵察";
pub const ESPIONAGE_RETURN: &str = "己方艦隊 | 間諜偵察 (返)";
pub const ACS_DEFEND: &str = "己方艦隊 | ACS聯合防禦";
pub const ACS_DEFEND_RETURN: &str = "己方艦隊 | ACS聯合防禦 (返)";
pub const ATTACKING: &str = "己方艦隊 | 攻擊";
pub const ATTACKING_RETURN: &str = "己方艦隊 | 攻擊 (返)";
pub const ACS_ATTACK: &str = "己方艦隊 | ACS聯合攻擊";
pub const ACS_ATTACK_RETURN: &str = "己方艦隊 | ACS聯合攻擊 (返)";
pub const DESTROY: &str = "己方艦隊 | 摧毀月球";
pub const DESTROY_RETURN: &str = "己方艦隊 | 摧毀月球 (返)";
pub const SEARCHING_FOR_LIFEFORMS: &str = "友方艦隊 | 搜索生命形式";
pub const SEARCHING_FOR_LIFEFORMS_RETURN: &str = "友方艦隊 | 搜索生命形式 (返)";

pub const FRIENDLY_TRANSPORT: &str = "友方艦隊 | 運輸";
pub const FRIENDLY_ACS_DEFEND: &str = "友方艦隊 | ACS聯合防禦";

pub const ENEMY_ESPIONAGE: &str = "敵方艦隊 | 間諜偵察";
pub const ENEMY_ATTACKING: &str = "敵方艦隊 | 攻擊";

// refresh rate
pub const SECOND: u32 = 1000;
pub const MINUTE: u32 = SECOND * 60;
pub const HOUR: u32 = MINUTE * 60;
pub const REFRESH_RATE: u32 = 15;
