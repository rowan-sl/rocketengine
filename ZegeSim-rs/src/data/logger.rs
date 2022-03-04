// use std::collections::HashMap;
// use std::fs::{File, OpenOptions};
// use std::path::PathBuf;

// pub struct DataLogger<T> {
//     variables: usize,
//     variable_descs: Vec<String>,
//     current_log: HashMap<String, Option<T>>,
//     file: File,
// }

// impl<T> DataLogger<T> {
//     pub fn new(filepath: PathBuf) -> Self {
//         let file = OpenOptions::new().write(true).create(true).open(filepath).unwrap();
//         Self {
//             variables: 0,
//             variable_descs: vec![],
//             current_log: HashMap::new(),
//             file,
//         }
//     }

//     /// Adds a data point to the logger object. Datapoints are added sequentially,
//     /// so add your variables in the same sequence that you want them to show up in on the CSV
//     pub fn add_data_point(&mut self, variable_name: String) {
//         if self.current_log.contains_key(&variable_name) {
//             panic!("Variable '{}' Already Initialized!", variable_name);
//         } else {
//             self.variables += 1;
//             self.current_log.insert(variable_name.clone(), None);
//             self.variable_descs.push(variable_name);
//         }
//     }

//     /// records a variable to the current log, DOES NOT LOG AUTOMATICALLY
//     pub fn record_variable(&mut self, name: String, data: T)
//     where
//         T: PartialEq
//     {
//         if self.current_log.contains_key(&name) {
//             if self.current_log[&name] != None {
//                 eprintln!("Warning! overwriting value for key {}", name);
//             }
//             *self.current_log.get_mut(&name).unwrap() = Some(data);
//         }
//     }

//     // TODO use builder pattern instead of this thingy
//     /// Initializes the CSV file and prepares it for writing.
//     pub fn init_csv(&mut self)

//     // def initCSV(self, makeFile, overWrite):
//     //     """""
//     //     self.initialized = True

//     //     os.chdir(os.path.dirname(os.path.abspath(__file__)))

//     //     if os.path.exists(str(self.fileName)):

//     //         f = open(str(self.fileName), "r")

//     //         if not f.read():
//     //             f.close()

//     //             f = open(str(self.fileName), "w")
//     //             outString = ""
//     //             for varName in self.variableDescriptions:
//     //                 outString += varName
//     //                 outString += ","

//     //             f.write(outString[0:-1])

//     //             f.write('\n')
//     //         else:
//     //             if overWrite == True:
//     //                 f.close()

//     //                 f = open(str(self.fileName), "w")
//     //                 outString = ""
//     //                 for varName in self.variableDescriptions:
//     //                     outString += varName
//     //                     outString += ","

//     //                 f.write(outString[0:-1])

//     //                 f.write('\n')
//     //             if overWrite == False:
//     //                 raise OSError("csv file is not empty!")

//     //     else:
//     //         if makeFile == True:
//     //             f = open(str(self.fileName), "w")

//     //             f.close()
//     //         else:
//     //             raise OSError("csv file not found!")
// }

// // def saveData(self, clearData):
// //     outString = ""
// //     for datapoint in self.currentLog:
// //         currentVar = self.currentLog[str(datapoint)]
// //         if currentVar == None:
// //             outString += "0"
// //         else:
// //             outString += str(currentVar)
// //         outString += ","
// //         if clearData == True:
// //             self.currentLog[str(datapoint)] = None
// //     f = open(str(self.fileName), "a")
// //     f.write(outString[0:-1] + "\n")
// //     // f.write('\n')

// //     use std::{collections::HashMap, fs::{File, OpenOptions}, path::PathBuf};

// // def getVariable(self, variableName):
// //     if str(variableName) in self.currentLog:
// //         return self.currentLog[str(variableName)]
// //     else:
// //         raise IndexError("datapoint not initialized")
