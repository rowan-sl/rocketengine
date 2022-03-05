import pathlib
import xml.etree.ElementTree as ET
import yaml
import os

cwd = pathlib.Path(__file__).parent

def load_motor_file(file_name):
    
    tree = ET.parse(file_name)
    root = tree.getroot()
    
    eng_data = root[0][0][1]

    output = []
    for data in eng_data:
        #time, thrust, mass (fuel, not total)
        output.append([float(data.attrib['t']), float(data.attrib['f']), float(data.attrib['m'])])
        
    metadata = root[0][0]
    
    return {
        "data": output,
        "name": metadata.get("code"),
        "dry_weight": float(metadata.get("initWt")) - float(metadata.get("propWt")),
        "fuel_weight": float(metadata.get("propWt")),
    }

def interpolate_thrust(thrust_curve, timeStep):
    thrustList = []
    lPoint = [0, 0, 0]
    for point in thrust_curve:
        if point[0] > 0:
            thrustDiff = point[1] - lPoint[1]
            massDiff = point[2] - lPoint[2]
            timeDiff = point[0] - lPoint[0]
            stepsNeeded = timeDiff * timeStep

            if stepsNeeded > 0:
                adder = thrustDiff / stepsNeeded
                adder_mass = massDiff / stepsNeeded

                i = 0

                thrustToAdd = lPoint[1]
                massToAdd = lPoint[2]

                while i < stepsNeeded:
                    i += 1
                    thrustToAdd += adder
                    if thrustToAdd < 0.0:
                        thrustToAdd = 0.0
                    massToAdd += adder_mass
                    if massToAdd < 0.0:
                        massToAdd = 0.0
                    thrustList.append([thrustToAdd, massToAdd])

        lPoint = point
    return thrustList

with open("motors_build_config.yaml", "r") as settingsFile:
    settings = yaml.load(settingsFile, Loader=yaml.FullLoader)

config = settings["cfg"]

interpolation_step = config["interp_step"]

manifest = ""
for file in cwd.joinpath("motors_raw").iterdir():
    data = load_motor_file(file)
    res = ""
    res += f"pub const DATA: [[f64; 3]; {len(data['data'])}] = [\n"
    for line in data["data"]:
        res += f"    [{line[0]}, {line[1]}, {line[2]}],\n"
    res += "];\n\n"
    res += f"pub const DRY_WEIGHT: f64 = {data['dry_weight']};\n"
    res += f"pub const FUEL_WEIGHT: f64 = {data['fuel_weight']};\n"
    res += f"pub const MOTOR_ID: &str = \"{data['name']}\";\n"
    manifest += f"pub mod {data['name']};\n"
    
    with open(f"src/motor/raw/{data['name']}.rs", "w") as f:
        f.write(res)

with open("src/motor/raw/mod.rs", "w") as f:
    f.write(manifest)
            
