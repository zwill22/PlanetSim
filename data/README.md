# Data for solar system planets and satellites

Taken from [Johnston's Archive](https://www.johnstonsarchive.net/astro/index.html)

Available data for the planets and planetary satellites of the solar system.
Including those trans-Neptunian objects (TNOs) which should be considered planets, their satellites, as well as the Sun and the three largest asteroids.

## Dynamic Data

Source: [HTML](https://www.johnstonsarchive.net/astro/solar_system_orb_dyn_data.html)

Columns:
- number: permanent number (satellites, asteroids, and TNOs).
- name: permanent name (brackets indicate informal names).
- prov_des: provisional designation (satellites and TNOs).
- fam: dynamical family with codes as follows:
  - Asteroids:
    - A-mb = main belt Asteroid
  - Planets:
    - PL-t = terrestrial planet
    - PL-j = jovian planet
  - Trans-Neptunian objects (TNOs):
    - TN-c = cubewano
    - TN-h = Haumea family
    - TN-p = plutino
    - TN-r = resonance object
    - TN-s = scattered disk object;
  - Irregular Jupiter satellites: 
    - AnaR = Ananke family (retrograde)
    - CarP = Carpo family (prograde)
    - CrmR = Carme family (retrograde)
    - HimP = Himalia family (prograde)
    - PasR = Pasiphae family (retrograde)
    - TheP = Themisto family (prograde)
    - ValP = Valetudo family (prograde)
  - Irregular Saturn satellites:
    - GaAP = Gallic-Albiorix family (prograde)
    - GalP = Gallic other family (prograde)
    - InKP = Inuit-Kiviuq family (prograde)
    - InSP = Inuit-Siarnaq family (prograde)
    - InuP = Inuit other family (prograde)
    - KarR = Kari family (retrograde)
    - MunR = Mundifari family (retrograde)
    - NorR = low inclination Norse family (retrograde)
    - PhoR = Phobe family (retrograde)
  - Other satellites:
    - REG = regular
    - SIR = small inner regular
    - IrrP = outer irregular prograde
    - IrrR = outer irregular retrograde
- Oscillating orbital elements (osc):
  - a = semimajor axis
  - e = eccentricity
  - q = periapse
  - Q = apoapse
  - incl = inclination
    - relative to ecliptic for Sun-orbiting objects
    - relative to planetary equator for planetary satellites
  - P = orbital period
  - epoch = orbital epoch
- Proper orbital elements:
  - a = time-averaged semimajor axis
  - e = eccentricity
  - incl = inclination
  - P = orbital period
- Orbit b and l: orbit pole location in celestial longitude and latitude
- RP: rotation period with notes
  - chao = chaotic rotation
  - SYN = synchronous with orbital period
  - SYNA = assumed synchronous with orbital period
- rotp b and l: rotation pole location in celestial longitude and latitude

## Physical Data

Source: [HTML](https://www.johnstonsarchive.net/astro/solar_system_phys_data.html)

Columns:
- number: permanent number (satellites, asteroids, and TNOs)
- name: permanent name (brackets indicate informal names)
- prov_des: provisional designation (satellites and TNOs)
- H: absolute magnitude (V)
- B-V and V-R: photometric colours
- mean d: diameter of sphere of equal volume
- a, b, c: triaxial dimensions
- a/b, a/c, and b/c: ratio of triaxial dimensions
- alb: albedo
- sphm: source for published shape model
- dens: mean density
- mass: mass
- GM: gravitational parameter, gravitational constant times mass
